#include "prime.h"
#include "gcd.h"
#include "modulo.h"
#include "file_reader.h"

int check_prime(long long n)
{
	if (n <= 1)
		return 0;

	for (long long i = 2; i <= sqrt(n); i++)
		if (n % i == 0)
			return 0;

	return 1;
}

/**
 * https://www.youtube.com/watch?v=zmhUlVck3J0
 * Doing this 100 times, the likely chance that we are wrong about
 * prime number in this test is
 * 1/1606938044258990275541962092341162602522202993782792835301376
 * or 6/10^61 or 1 in quintillion quintillion.
*/
int check_prime_lehnman(long long n, int k)
{
	if (n < 4)
		return n == 2 || n == 3;

	if (n % 2 == 0 || n % 3 == 0)
		return 0;

	long long a, e, x;

	a = 2 + (rand()%(n-3));
	e = (n-1) / 2;
	while (k > 0) {
		if (gcd(a, n) != 1)
			return 0;

		x = fast_power_modulo(a, e, n);

		if (x == 1 || x == n-1) {
			a = 2 + (rand()%(n-3));
			k--;
		} else {
			return 0;
		}
	}

	return 1;
}

/**
 * 1. Check for non-zero byte
 * 2. Check for the first 1s bit
 * 3. Fill the prime with 8-bit shift
 * 4. Fill the missing bit if n % 8 != 0 or the first bit in 2. is not 8th bit.
 *
 * Example:
 *	01100100, first bit found at 7th, want 19 bits number.
			19 % 8 == 3, so fill the missing bit by 3+1=4
*/
long long gen_prime(int n, char *filename)
{
	int i;
	int* data;

	data = malloc(MAX_SIZE*sizeof(int));
	data = read(filename);

	i = 0;
	while (data[i] == 0 && data[i] != EOF) {
		i++;
	}

	if (data[i] == EOF) {
		printf("crel: Not able to find suitable bit to generate prime (file size too small or no data in file)\n");
		return -1;
	}

	int k, len, m;
	long long res;

	k = BYTE_SIZE;
	while ((data[i] & (1 << (k-1))) == 0) {
		k--;
	}

	res = data[i++];
	for (len = i + (n/BYTE_SIZE) - 1; i < len; i++) {
		res = res << BYTE_SIZE;
		res = res + data[i];
	}

	if (n < BYTE_SIZE) {
		m = abs(n-k);
		res = (k < n) ? (res<<m)+(data[i]>>(BYTE_SIZE-m)) : (res>>m);
	} else {
		m = (BYTE_SIZE-k) + (n%BYTE_SIZE);
		res = (res << m) + (data[i] >> (BYTE_SIZE-m));
	}

	if (!(res & 1))
		res += 1;

	while (!check_prime_lehnman(res, TEST_ROUND)) {
		res += 2;
	}

	free(data);
	return res;
}

long long* gen_with_inverse(long long n)
{
	long long e, inv_e;
	long long* res;
	res = malloc(2*sizeof(long long));

	do {
		e = rand();
	} while (gcd(e, n) != 1);

	inv_e = inverse_modulo(e, n);

	res[0] = e, res[1] = inv_e;
	return res;
}
