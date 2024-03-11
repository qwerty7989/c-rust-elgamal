#include "modulo.h"
#include "gcd.h"

/**
 * https://en.wikipedia.org/wiki/Modular_multiplicative_inverse#Extended_Euclidean_algorithm
 * ax = 1 mod m
*/
void inverse_modulo(mpz_t a, mpz_t m, mpz_t x)
{
	mpz_t tmp_m;

	mpz_init(tmp_m);
	mpz_set(tmp_m, m);

	gcd_extended(a, tmp_m, x);
	mpz_mod(x, x, m); // x = x % m;

	while (mpz_cmp_si(x, 0) < 0)
		mpz_add(x, x, m); // x = x + m;

	mpz_clear(tmp_m);

	// return x;
}

/**
 * https://en.wikipedia.org/wiki/Modular_exponentiation#Right-to-left_binary_method
 * c = product of (b^a * 2^i) where i=[0,n-1]
*/
void fast_power_modulo(mpz_t b, mpz_t e, mpz_t m, mpz_t x)
{
	mpz_mod(b, b, m);
	if (mpz_cmp_si(b, 0) == 0) {
		mpz_set_ui(x, 0);
		return;
	}

	mpz_set_ui(x, 1);
	while (mpz_cmp_si(e, 0) > 0) {
		if (mpz_odd_p(e)) {
			// c = (c*b) % m;
			mpz_mul(x, x, b);
			mpz_mod(x ,x ,m);
		}

		mpz_tdiv_q_2exp(e, e, 1); // right shift
		mpz_mul(b, b, b);
		mpz_mod(b ,b ,m);
	}

	// return c;
}
