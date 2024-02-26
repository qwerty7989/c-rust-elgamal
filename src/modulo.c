#include "modulo.h"
#include "gcd.h"

int inverse_modulo(int a, int p)
{
	int s, r;
	s = gcd_extended(a, p);
	r = s % p;

	while (r < 0)
		r += p;

	return r;
}