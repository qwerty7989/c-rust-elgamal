#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <gmp.h>
#define TEST_ROUND 100

int check_prime_lehnman(mpz_t n, int t);
int gen_prime(int n, char* filename, mpz_t res);
void gen_with_inverse(mpz_t n, mpz_t e, mpz_t inv_e);
