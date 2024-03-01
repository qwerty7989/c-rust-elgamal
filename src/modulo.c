#include "modulo.h"
#include "gcd.h"

/**
 * https://en.wikipedia.org/wiki/Modular_multiplicative_inverse#Extended_Euclidean_algorithm
 * ax = 1 mod m
*/
long long inverse_modulo(long long a, long long m)
{
	long long x;
	x = gcd_extended(a, m);
	x = x % m;

	while (x < 0)
		x += m;

	return x;
}

/**
 * https://en.wikipedia.org/wiki/Modular_exponentiation#Memory-efficient_method
 * c = b^e mod m
*/
long long power_modulo(long long b, long long e, long long m)
{
	if (m == 1)
		return 0;

	long long c, i;
	for (i = 0, c = 1; i < e; i++) {
		c = (c*b) % m;
	}

	return c;
}

/**
 * https://en.wikipedia.org/wiki/Modular_exponentiation#Right-to-left_binary_method
 * c = product of (b^a * 2^i) where i=[0,n-1]
*/
long long fast_power_modulo(long long b, long long e, long long m)
{
	b = b % m;
	if (b == 0)
		return 0;

	long long c;

	c = 1;
	while (e > 0) {
		if (e & 1)
			c = (c*b) % m;

		e = e >> 1;
		b = (b*b) % m;
	}

	return c;
}
