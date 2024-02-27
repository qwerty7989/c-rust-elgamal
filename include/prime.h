#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#define TEST_ROUND 100

int check_prime(long long n);
int check_prime_lehnman(long long n, int t);
long long gen_prime(int n, char* filename);
long long* gen_with_inverse(long long n);
