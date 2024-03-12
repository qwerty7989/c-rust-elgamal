#include "elgamal.h"
#include "gcd.h"
#include "prime.h"
#include "modulo.h"

// a^(p-1/2) = -1 mod p
int check_generator(mpz_t p_root)
{
	mpz_sub_ui(p_root, p_root, 1);
	mpz_tdiv_q_2exp(p_root, p_root, 1);

	if (check_prime_lehnman(p_root, TEST_ROUND)) {
		return 1;
	} else {
		return 0;
	}
}

int gen_generator(mpz_t n, mpz_t g)
{
	mpz_t a, p_root, c, x;
	mpz_t tmp_a, tmp_p_root, tmp_n;
	mpz_inits(a, p_root, c, x, tmp_a, tmp_p_root, tmp_n, NULL);

	mpz_set(p_root, n);

	if (!check_generator(p_root))
		return 0;

	gmp_randstate_t rstate;
	gmp_randinit_mt(rstate);
	gmp_randseed_ui(rstate, rand());

	mpz_set(tmp_n, n);

	mpz_urandomm(a, rstate, n);
	mpz_set(tmp_a, a);
	mpz_set(tmp_p_root, p_root);
	fast_power_modulo(tmp_a, tmp_p_root, tmp_n, c);

	if (mpz_cmp_si(c, 1) == 0) {
		mpz_set(tmp_a, a);
		mpz_set(tmp_n, n);
		inverse_modulo(tmp_a, tmp_n, x);
		mpz_set(g, x);
	} else {
		mpz_set(g, a);
	}

	mpz_clears(a, p_root, c, x, tmp_a, tmp_p_root, tmp_n, rstate, NULL);

	return 1;
}

/**
 * Generate public and private key pair
*/
int elgamal_keygen(mpz_t n, mpz_t g, mpz_t y, mpz_t u)
{
	if (!gen_generator(n, g))
		return 0;

	mpz_t tmp_g, tmp_u, tmp_n;
	mpz_inits(tmp_g, tmp_u, tmp_n, NULL);

	gmp_randstate_t rstate;
	gmp_randinit_mt(rstate);
	gmp_randseed_ui(rstate, rand());
	mpz_urandomm(u, rstate, n);

	mpz_set(tmp_g, g);
	mpz_set(tmp_u, u);
	mpz_set(tmp_n, n);

	fast_power_modulo(tmp_g, tmp_u, tmp_n, y);

	mpz_clears(tmp_g, tmp_u, tmp_n, rstate, NULL);

	return 1;
}

/**
 * Encrypt text or file
*/
int elgamal_encrypt_number(mpz_t n, mpz_t g, mpz_t y, mpz_t a, mpz_t b, mpz_t x)
{
	mpz_t k, tmp_g, tmp_y, tmp_k, tmp_n;
	mpz_inits(k, tmp_g, tmp_y, tmp_k, tmp_n, NULL);

	gmp_randstate_t rstate;
	gmp_randinit_mt(rstate);
	gmp_randseed_ui(rstate, rand());

	do {
		mpz_sub_ui(tmp_n, n, 1);
		mpz_urandomm(k, rstate, n);
		mpz_set(tmp_k, k);
		gcd(tmp_k, tmp_n);
	} while (mpz_cmp_si(tmp_n, 1) != 0);

	mpz_set(tmp_g, g);
	mpz_set(tmp_k, k);
	mpz_set(tmp_n, n);
	fast_power_modulo(tmp_g, tmp_k, tmp_n, a); //a = g^k mod n

	mpz_set(tmp_y, y);
	mpz_set(tmp_k, k);
	mpz_set(tmp_n, n);

	fast_power_modulo(tmp_y, tmp_k, tmp_n, b); //b1 = y^k mod n
	mpz_mul(b, b, x); //b = b1 * X mod n
	mpz_mod(b, b, n);

	mpz_clears(k, tmp_g, tmp_y, tmp_k, tmp_n, rstate, NULL);
	return 0;
}

/**
 * Decrypt text or file
*/
void elgamal_decrypt_number(mpz_t n, mpz_t u, mpz_t a, mpz_t b, mpz_t x)
{
	mpz_t tmp_n, tmp_u, tmp_a, tmp_b;
	mpz_inits(tmp_n, tmp_u, tmp_a, tmp_b, NULL);

	mpz_set(tmp_a, a);
	mpz_set(tmp_u, u);
	mpz_set(tmp_n, n);
	fast_power_modulo(tmp_a, tmp_u, tmp_n, x); // a^u

	mpz_set(tmp_a, a);
	mpz_set(tmp_n, n);
	inverse_modulo(x, tmp_n, x); // inverse of (a^u)

	mpz_mul(x, x, b); // x1 * b
	mpz_mod(x, x, n); // x2 mod n

	mpz_clears(tmp_n, tmp_u, tmp_a, tmp_b, NULL);
}
