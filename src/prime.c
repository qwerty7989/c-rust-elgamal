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
int check_prime_lehnman(long long n, int t)
{
	if (n < 4)
		return n == 2 || n == 3;

	if (n % 2 == 0)
		return 0;

	long long a, e, res, i;

	a = 2 + (rand() % (n-3));
	e = (n-1) / 2;
	while (t > 0) {
		if (gcd(a, n) != 1)
			return 0;

		res = fast_power_modulo(a, e, n);

		if (res == 1 || res == n-1) {
			a = 2 + (rand() % (n-3));
			t--;
		} else {
			return 0;
		}
	}

	return 1;
}

long long gen_prime(int n, char *filename)
{
	int i, j, k, len;
	int* data;
	data = malloc(MAX_SIZE*sizeof(int));
	data = read(filename);

	i = 0;
	while (data[i] == 0 && data[i] != EOF) {
		i++;
	}

	k = BYTE_SIZE - 1;
	while (data[i] & (1 << k) == 0) {
		k--;
	}

	long long res;
	len = n / BYTE_SIZE;
	res = data[i++];
	for (j = i + len - 1; i < j; i++) {
		res = res << BYTE_SIZE;
		res = res + data[i];
	}

	if (n % BYTE_SIZE != 0 && n > BYTE_SIZE && k < BYTE_SIZE) {
		res = res << ((n % BYTE_SIZE) + (BYTE_SIZE-k));
		res = res + (data[i] >> (BYTE_SIZE - (n % BYTE_SIZE) - (BYTE_SIZE-k)));
	} else if (n < BYTE_SIZE) {
		if (k >= n) {
			res = res >> (k-n);
		} else {
			res = res << (n-k);
			res = res + (data[i] >> (n-k));
		}
	} else if (k < BYTE_SIZE) {
		res = res << (BYTE_SIZE-k);
		res = res + (data[i] >> k);
	}

	if (res % 2 == 0)
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
