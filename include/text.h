#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <gmp.h>
#include <ctype.h>
#define MAX_LENGTH 4096

void integer_string_to_binary_string(char* str);
void binary_string_to_integer_string(char* str);
void combined_private_string(char* str, mpz_t n, mpz_t u);
void combined_public_string(char* str, mpz_t n, mpz_t g, mpz_t y);
void decombined_public_string(char* str, char* key_size_bit, mpz_t n, mpz_t g, mpz_t y);
void base64_encoder(char* input_str, int len_str);
void base64_decoder(char encoded[], int len_str);