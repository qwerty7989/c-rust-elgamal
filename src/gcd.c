#include "gcd.h"

unsigned long long gcd(unsigned long long a, unsigned long long b)
{
	unsigned long long temp;
	while (a != 0) {
		temp = a;
		a = b % a;
		b = temp;
	}
	return b;
}

int gcd_extended(unsigned long long a, unsigned long long b)
{
	while (b != 0) {
		unsigned long long q = a / b;
		unsigned long long r = a % b;

		a = b;
		b = r;
	}
	return a == 1;
}