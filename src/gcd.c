#include "gcd.h"

int gcd(int a, int b)
{
	int temp;
	while (a != 0) {
		temp = a;
		a = b % a;
		b = temp;
	}
	return b;
}

int gcd_extended(int a, int p)
{
	int s0, s1, t0, t1;

	s0 = 1, s1 = 0;
	t0 = 0, t1 = 1;

	int tmp, q;
	while (p != 0) {
		q = a / p;

		tmp = p;
		p = a - q*tmp;
		a = tmp;

		tmp = s1;
		s1 = s0 - q*tmp;
		s0 = tmp;

		tmp = t1;
		t1 = t0 - q*tmp;
		t0 = tmp;
	}

	/**
	 * https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Description
	 * as + pt = gcd(a, p)
	*/
	return s0;
}