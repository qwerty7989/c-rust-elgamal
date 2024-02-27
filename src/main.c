#include "gcd.h"
#include "modulo.h"
#include "prime.h"
#include "file_reader.h"

int main(int argc, char* argv[])
{
	unsigned long long a, b;
	a = atoi(argv[1]);
	b = atoi(argv[2]);
	printf("gcd(%d, %d): %d\n", a, b, gcd(a, b));
	printf("as + bt = gcd(a,b) | s = %d\n", gcd_extended(a, b));
	printf("inverse_modulo = %d\n", inverse_modulo(a, b));
	printf("power_module = %d\n", power_modulo(4, 13, 497));
	printf("check_prime(%d): %s\n", a, check_prime(a) ? "Yes": "No");
	printf("check_prime_lehnman(%d): %s\n", a, check_prime_lehnman(a, 100) ? "Yes": "No");
	printf("%s\n", read("src/main.c"));
	printf("read: %s\n", read("src/main.c"));
	printf("gen_prime: %llu\n", gen_prime(64, "bin"));
	unsigned long long* e_both = gen_with_inverse(rand());
	printf("gen_with_inverse: e=%d e^-1=%d\n", e_both[0], e_both[1]);

	return 0;
}
