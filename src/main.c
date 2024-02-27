#include "gcd.h"
#include "modulo.h"
#include "prime.h"
#include "file_reader.h"

int main(int argc, char* argv[])
{
	long long a, b;

	if (argc == 1) {
		printf("Usage: crel [OPTION] ...\n");
		printf("  -g <file> [bits]\t\tgenerate prime number from file.\n");
		printf("  -r [number]\t\t\tgenerate random number with modulo inverse\n");
		printf("  -i [remainder] [modulus]\toutput modulo inverse of input number\n");
		printf("  -x [base] [expo] [modulus]\toutput modulo exponentiation of input number\n");
		printf("  -c [number1] [number2]\toutput gcd(number1, number2)\n");
		printf("  -p [number]\t\t\toutput yes if number is prime, otherwise no\n");
		printf("  -l [number] [tries]\t\toutput yes if number is probably prime, otherwise no\n");
	}

	switch(argv[1][1]) {
		case 'g':
			if (argc == 4) {
				printf("%llu\n", gen_prime(atoi(argv[3]), argv[2]));
			} else {
				printf("  -g <file> [bits]\t\tgenerate prime number from file.\n");
				printf("crel: Please enter both filename and bits\n");
			}
			break;
		case 'r':
			if (argc == 3) {
				long long* res = gen_with_inverse(atoi(argv[2]));
				printf("e: %lld\ne^-1: %lld\n", res[0], res[1]);
			} else {
				printf("  -r [number]\t\t\tgenerate random number with modulo inverse\n");
				printf("crel: Please enter number\n");
			}
			break;
		case 'i':
			if (argc == 4) {
				printf("%lld\n", inverse_modulo(atoi(argv[2]), atoi(argv[3])));
			} else {
				printf("  -i [remainder] [modulus]\toutput modulo inverse of input number\n");
				printf("crel: Please enter both remainder and modulus\n");
			}
			break;
		case 'x':
			if (argc == 5) {
				printf("%lld\n", power_modulo(atoi(argv[2]), atoi(argv[3]), atoi(argv[4])));
			} else {
				printf("  -x [base] [expo] [modulus]\toutput modulo exponentiation of input number\n");
				printf("crel: Please enter all the base, expo, and modulus\n");
			}
			break;
		case 'c':
			if (argc == 4) {
				printf("%lld\n", gcd(atoi(argv[2]), atoi(argv[3])));
			} else {
				printf("  -c [number1] [number2]\toutput gcd(number1, number2)\n");
				printf("crel: Please enter both numbers\n");
			}
			break;
		case 'p':
			if (argc == 3) {
				printf("%lld is %s\n", atoi(argv[2]), check_prime(atoi(argv[2])) ? "prime": "not prime");
			} else {
				printf("  -p [number]\t\t\toutput yes if number is prime, otherwise no\n");
				printf("crel: Please enter both numbers\n");
			}
			break;
		case 'l':
			if (argc == 4) {
				printf("%lld is %s\n", atoi(argv[2]), check_prime_lehnman(atoi(argv[2]), atoi(argv[3])) ? "prime": "not prime");
			} else {
				printf("  -l [number] [tries]\t\toutput yes if number is probably prime, otherwise no\n");
				printf("crel: Please enter both the number and tries\n");
			}
			break;
		default:
			printf("WAT\n");
			break;
	}

	//a = atoi(argv[1]);
	//b = atoi(argv[2]);
	//printf("gcd(%d, %d): %d\n", a, b, gcd(a, b));
	//printf("as + bt = gcd(a,b) | s = %d\n", gcd_extended(a, b));
	//printf("inverse_modulo = %d\n", inverse_modulo(a, b));
	//printf("power_module = %d\n", power_modulo(4, 13, 497));
	//printf("check_prime(%d): %s\n", a, check_prime(a) ? "Yes": "No");
	//printf("check_prime_lehnman(%d): %s\n", a, check_prime_lehnman(a, 100) ? "Yes": "No");
	//printf("%s\n", read("src/main.c"));
	//printf("read: %s\n", read("src/main.c"));
	//long long* e_both = gen_with_inverse(rand());
	//printf("gen_with_inverse: e=%d e^-1=%d\n", e_both[0], e_both[1]);



	return 0;
}
