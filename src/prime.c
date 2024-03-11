#include "prime.h"
#include "gcd.h"
#include "modulo.h"
#include "file_reader.h"

/**
 * https://www.youtube.com/watch?v=zmhUlVck3J0
 * Doing this 100 times, the likely chance that we are wrong about
 * prime number in this test is
 * 1/1606938044258990275541962092341162602522202993782792835301376
 * or 6/10^61 or 1 in quintillion quintillion.
*/
int check_prime_lehnman(mpz_t n, int t)
{
	if (mpz_cmp_si(n, 4) < 0)
		return mpz_cmp_si(n, 2) == 0 || mpz_cmp_si(n, 3) == 0;

	if (mpz_divisible_ui_p(n, 2) || mpz_divisible_ui_p(n, 3))
		return 0;

	mpz_t a, e, x, tmp_a, tmp_e, tmp_n;
	gmp_randstate_t rstate;
	gmp_randinit_mt(rstate);
	gmp_randseed_ui(rstate, rand());

	mpz_init(a);
	mpz_init(e);
	mpz_init(x);
	mpz_init(tmp_a);
	mpz_init(tmp_e);
	mpz_init(tmp_n);

	//// a = 2 + (rand()%(n-3));
	mpz_urandomm(a, rstate, n);
	mpz_sub_ui(tmp_n, n, 3);
	mpz_mod(a, a, tmp_n);
	mpz_add_ui(a, a, 2);

	//// e = (n-1) / 2;
	mpz_sub_ui(tmp_n, n, 1);
	mpz_tdiv_q_2exp(e, tmp_n, 1);

	while (t > 0) {
		mpz_set(tmp_a, a);
		mpz_set(tmp_e, e);
		mpz_set(tmp_n, n);

		gcd(tmp_a, tmp_n);
		if (mpz_cmp_si(tmp_n, 1) != 0) // gcd(a,n) != 1
			return 0;

		mpz_set(tmp_a, a);
		mpz_set(tmp_n, n);
		fast_power_modulo(tmp_a, tmp_e, tmp_n, x);

		mpz_sub_ui(tmp_n, n, 1);
		if (mpz_cmp_si(x, 1) == 0 || mpz_cmp(x, tmp_n) == 0) {
			// a = 2 + (rand()%(n-3));
			mpz_urandomm(a, rstate, n);
			mpz_sub_ui(tmp_n, n, 3);
			mpz_mod(a, a, tmp_n);
			mpz_add_ui(a, a, 2);
			t--;
		} else {
			// free variable from memory
			mpz_clear(a);
			mpz_clear(e);
			mpz_clear(x);
			mpz_clear(tmp_a);
			mpz_clear(tmp_e);
			mpz_clear(tmp_n);

			return 0;
		}
	}

	// free variable from memory
	mpz_clear(a);
	mpz_clear(e);
	mpz_clear(x);
	mpz_clear(tmp_a);
	mpz_clear(tmp_e);
	mpz_clear(tmp_n);

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
int gen_prime(int n, char *filename, mpz_t res)
{
	// Read and check for non-zero data byte
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
		return 0;
	}

	// Convert data byte to bit
	int k, len, m, q, flag;

	k = BYTE_SIZE;
	while ((data[i] & (1 << (k-1))) == 0) {
		k--;
	}

	flag = 0;
	mpz_set_si(res, data[i++]);
	for (len = i + (n/BYTE_SIZE) - 1; i < len; i++) {
		mpz_mul_2exp(res, res, BYTE_SIZE); // left shift
		if (flag || data[i] == EOF) {
			flag = 1;
			mpz_add_ui(res, res, 0);
		} else {
			mpz_add_ui(res, res, data[i]);
		}
	}

	if (flag || data[i] == EOF)
		data[i] = 0;

	if (n < BYTE_SIZE) {
		m = abs(n-k);
		if (k < n) {
			mpz_mul_2exp(res, res, m);
			q = data[i] >> (BYTE_SIZE-m);
			mpz_add_ui(res, res, q);
		} else {
			mpz_tdiv_q_2exp(res, res, m); // right shift
		}
	} else {
		m = (BYTE_SIZE-k) + (n%BYTE_SIZE);
		mpz_mul_2exp(res, res, m);
		q = data[i] >> (BYTE_SIZE-m);
		mpz_add_ui(res, res, q);
	}

	free(data);

	// Find closest probable prime
	if (mpz_even_p(res))
		mpz_add_ui(res ,res, 1);

	while (!check_prime_lehnman(res, TEST_ROUND)) {
		mpz_add_ui(res ,res, 2);
	}

	return 1; // return res
}

void gen_with_inverse(mpz_t n, mpz_t e, mpz_t inv_e)
{
	mpz_t tmp_e, tmp_n;

	mpz_inits(tmp_e, tmp_n, NULL);

	mpz_set(tmp_e, e);
	mpz_set(tmp_n, n);

	gmp_randstate_t rstate;
	gmp_randinit_mt(rstate);
	gmp_randseed_ui(rstate, rand());

	do {
		mpz_urandomm(e, rstate, n);

		mpz_set(tmp_e, e);
		mpz_set(tmp_n, n);
		gcd(tmp_e, tmp_n);
	} while (mpz_cmp_si(tmp_n, 1) != 0);

	mpz_set(tmp_e, e);
	mpz_set(tmp_n, n);
	inverse_modulo(tmp_e, tmp_n, inv_e);

	mpz_clears(tmp_e, tmp_n, rstate, NULL);

	// return res;
}
