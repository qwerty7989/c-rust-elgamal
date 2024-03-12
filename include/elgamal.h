#include <stdio.h>
#include <stdlib.h>
#include <gmp.h>

int check_generator(mpz_t p_root);
int gen_generator(mpz_t n, mpz_t g);
int elgamal_keygen(mpz_t n, mpz_t g, mpz_t y, mpz_t u);
int elgamal_encrypt_number(mpz_t n, mpz_t g, mpz_t y, mpz_t a, mpz_t b, mpz_t x);
void elgamal_decrypt_number(mpz_t n, mpz_t u, mpz_t a, mpz_t b, mpz_t x);