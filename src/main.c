#include "gcd.h"
#include "prime.h"

int main()
{
	unsigned long long a, b;
	a = 101;
	b = 26;
	printf("gcd(%llu, %llu): %d\n", a, b, gcd(a, b));
	printf("isPrime(%llu): %s\n", a, is_prime(a) ? "Yes": "No");
	return 0;
}