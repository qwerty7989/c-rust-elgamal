#include "modulo.h"
#include "gcd.h"

long long inverse_modulo(long long a, long long modulus)
{
	long long s, r;
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
long long power_modulo(long long b, long long e, long long n)
{
	if (n == 1)
		return 0;

	long long c, e_prime;
	for (c = 1, e_prime = 0; e_prime < e; e_prime++) {
		c = (c*b) % n;
	}

	return c;
}

long long fast_power_modulo(long long b, long long e, long long n)
{
	long long res;
	res = 1;

	b = b % n;

	if (b == 0) return 0;

	while (e > 0) {
		if (e & 1)
			res = (res*b) % n;

		e = e >> 1;
		b = (b*b) % n;	
	}

	return res;
}
