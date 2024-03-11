#include <stdio.h>
#include <gmp.h>

void inverse_modulo(mpz_t a, mpz_t m, mpz_t x);
void fast_power_modulo(mpz_t b, mpz_t e, mpz_t m, mpz_t x);