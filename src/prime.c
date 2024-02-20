#include "prime.h"

int check_prime(unsigned long long n)
{
	if (n <= 1)
		return 0;

	for (unsigned long long i = 2; i <= sqrt(n); i++)
		if (n % i == 0)
			return 0;

	return 1;
}