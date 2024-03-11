#include "gcd.h"

void gcd(mpz_t a, mpz_t b)
{
	mpz_t tmp;

	mpz_init(tmp);

	while (mpz_cmp_si(a, 0) != 0) {
		mpz_set(tmp, a); // tmp = a;
		mpz_mod(a, b, a); // a = b % a;
		mpz_set(b, tmp); // b = tmp;
	}

	mpz_clear(tmp);

	//return b;
}

/**
 * https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Description
 * https://mathsanew.com/articles/extended_euclid_algorithm.pdf
 * https://www.math.ualberta.ca/~isaac/math324/s11/euclidean.pdf
 * https://www.csd.uwo.ca/~mmorenom/CS424/Lectures/EuclideanMethods.html/node4.html
 * as + bt = gcd(a, b)
*/
void gcd_extended(mpz_t a, mpz_t b, mpz_t x)
{
	mpz_t s0, s1, t0, t1, tmp, q;

	mpz_inits(s0, s1, t0, t1, tmp, q, NULL);

	mpz_set_si(s0, 1);
	mpz_set_si(t0, 0);
	mpz_set_si(s1, 0);
	mpz_set_si(t1, 1);

	while (mpz_cmp_si(b, 0) != 0) {
		mpz_tdiv_q(q, a, b);

		mpz_set(tmp, b); // tmp = b;
		mpz_mul(b, q, tmp); // b = a - (q*tmp);
		mpz_sub(b, a, b);
		mpz_set(a, tmp); //a = tmp;

		mpz_set(tmp, s1); // tmp = s1;
		mpz_mul(s1, q, tmp); // s1 = s0 - (q*tmp);
		mpz_sub(s1, s0, s1);
		mpz_set(s0, tmp); //s0 = tmp;

		mpz_set(tmp, t1); // tmp = t1;
		mpz_mul(t1, q, tmp); // t1 = t0 - (q*tmp);
		mpz_sub(t1, t0, t1);
		mpz_set(t0, tmp); //t0 = tmp;
	}

	mpz_set(x, s0);

	mpz_clears(s0, s1, t0, t1, tmp, q, NULL);

	// return s0;
}