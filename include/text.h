#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <gmp.h>

void binary_string_to_integer_string(char* str);
void combined_private_string(char* str, mpz_t n, mpz_t u);
void combined_public_string(char* str, mpz_t n, mpz_t g, mpz_t y);
void base64Encoder(char* input_str, int len_str);