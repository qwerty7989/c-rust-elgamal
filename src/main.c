#include "gcd.h"
#include "modulo.h"
#include "prime.h"

int main(int argc, char* argv[])
{
	int a, b;
	a = atoi(argv[1]);
	b = atoi(argv[2]);
	printf("gcd(%d, %d): %d\n", a, b, gcd(a, b));
	printf("as + bt = gcd(a,b) | s = %d\n", gcd_extended(a, b));
	printf("inverse_modulo = %d\n", inverse_modulo(a, b));
	printf("check_prime(%d): %s\n", a, check_prime(a) ? "Yes": "No");
	return 0;
}
