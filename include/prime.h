#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#define TEST_ROUND 100

int check_prime(unsigned long long n);
int check_prime_lehnman(unsigned long long n, int t);
unsigned long long gen_prime(int n, char* filename);
unsigned long long* gen_with_inverse(unsigned long long n);