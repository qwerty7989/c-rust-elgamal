#include "modulo.h"
#include "gcd.h"

unsigned long long inverse_modulo(unsigned long long a, unsigned long long modulus)
{
	unsigned long long s, r;
	s = gcd_extended(a, modulus);
	r = s % modulus;

	while (r < 0)
		r += modulus;

	return r;
}

/**
 * https://en.wikipedia.org/wiki/Modular_exponentiation#Memory-efficient_method
 * c = b^e mod n
*/
unsigned long long power_modulo(unsigned long long b, unsigned long long e, unsigned long long n)
{
	if (n == 1)
		return 0;

	unsigned long long c, e_prime;
	for (c = 1, e_prime = 0; e_prime < e; e_prime++) {
		c = (c*b) % n;
	}

	return c;
}