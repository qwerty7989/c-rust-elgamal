#include "gcd.h"

long long gcd(long long a, long long b)
{
	long long temp;
	while (a != 0) {
		temp = a;
		a = b % a;
		b = temp;
	}
	return b;
}

/**
 * https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Description
 * https://mathsanew.com/articles/extended_euclid_algorithm.pdf
 * https://www.math.ualberta.ca/~isaac/math324/s11/euclidean.pdf
 * https://www.csd.uwo.ca/~mmorenom/CS424/Lectures/EuclideanMethods.html/node4.html
 * as + bt = gcd(a, b)
*/
long long gcd_extended(long long a, long long b)
{
	long long s0, s1, t0, t1;
	long long tmp, q;

	s0 = t1 = 1;
	t0 = s1 = 0;
	while (b != 0) {
		q = a / b;

		tmp = b;
		b = a - (q*tmp);
		a = tmp;

		tmp = s1;
		s1 = s0 - (q*tmp);
		s0 = tmp;

		tmp = t1;
		t1 = t0 - (q*tmp);
		t0 = tmp;
	}

	return s0;
}