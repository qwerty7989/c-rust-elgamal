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
		printf("  -o <file>\t\t\tReading the file data, return data if readable\n");
	}

	switch(argv[1][1]) {
		case 'g':
			if (argc == 4) {
				printf("%lld\n", gen_prime(atoi(argv[3]), argv[2]));
			} else {
				printf("  -g <file> [bits]\t\tgenerate prime number from file.\n");
				printf("crel: Please enter both filename and bits\n");
			}
			break;
		case 'r':
			if (argc == 3) {
				long long* res = gen_with_inverse(atoi(argv[2]));
				printf("e: %lld\ne^-1: %lld\n", res[0], res[1]);
				free(res);
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
		case 'o':
			if (argc == 3) {
				int* data = read(argv[2]);
				for (int i = 0; data[i] != EOF; i++)
					printf("%02x", data[i]);
				printf("\n");
				free(data);
			} else {
				printf("  -o <file>\t\t\tReading the file data, return data if readable\n");
				printf("crel: Please enter filename\n");
			}
			break;

		case 'd': // debug
			break;
		default:
			printf("WAT\n");
			break;
	}

	return 0;
}
