#include "gcd.h"
#include "modulo.h"
#include "prime.h"
#include "file_reader.h"

int main(int argc, char* argv[])
{
	if (argc == 1 || (argc > 1 && argv[1][0] != '-')) {
		printf("Usage: crel [OPTION] ...\n");
		printf("  -g <file> [bits]\t\tgenerate prime number from file.\n");
		printf("  -e [number]\t\t\tgenerate random number with modulo inverse\n");
		printf("  -r [number] [modulus]\t\toutput modulo inverse of input number\n");
		printf("  -d [number1] [number2]\toutput gcd(number1, number2)\n");
		printf("  -p [number] [tries]\t\toutput yes if number is probably prime, otherwise no\n");
		printf("  -o <file>\t\t\tReading the file data, return data if readable\n");
		return 0;
	}

	switch(argv[1][1]) {
	case 'g':
		if (argc == 4) {
			mpz_t res;

			mpz_init(res);

			if (gen_prime(atoi(argv[3]), argv[2], res)) {
				gmp_printf("%Zd\n", res);
				//mpz_out_str(stdout, 2, res);
				//printf("\n");
			}

			mpz_clear(res);
		} else {
			printf("  -g <file> [bits]\t\tgenerate prime number from file.\n");
			printf("crel: Please enter both filename and bits\n");
		}
		break;
	case 'e':
		if (argc == 3) {
			mpz_t n, e, inv_e;

			mpz_inits(n, e, inv_e, NULL);

			mpz_set_str(n, argv[2], 10);
			gen_with_inverse(n, e, inv_e);

			gmp_printf("e is %Zd\ne^-1 is %Zd\n", e, inv_e);

			mpz_clears(n, e, inv_e, NULL);
		} else {
			printf("  -e [number]\t\t\tgenerate random number with modulo inverse\n");
			printf("crel: Please enter number\n");
		}
		break;
	case 'r':
		if (argc == 4) {
			mpz_t a, m, x;

			mpz_inits(a, m, x, NULL);

			mpz_set_str(a, argv[2], 10);
			mpz_set_str(m, argv[3], 10);

			inverse_modulo(a, m, x);

			gmp_printf("%Zd\n", x);

			mpz_clears(a, m, x, NULL);
		} else {
			printf("  -r [number] [modulus]\toutput modulo inverse of input number\n");
			printf("crel: Please enter both remainder and modulus\n");
		}
		break;
	case 'd':
		if (argc == 4) {
			mpz_t a, b;

			mpz_inits(a, b, NULL);

			mpz_set_str(a, argv[2], 10);
			mpz_set_str(b, argv[3], 10);

			gcd(a, b);

			gmp_printf("%Zd\n", b);

			mpz_clears(a, b, NULL);
		} else {
			printf("  -d [number1] [number2]\toutput gcd(number1, number2)\n");
			printf("crel: Please enter both numbers\n");
		}
		break;
	case 'p':
		if (argc == 4) {
			mpz_t res;

			mpz_init(res);

			mpz_set_str(res, argv[2], 10);

			printf("%s ", argv[2]);

			if (check_prime_lehnman(res, atoi(argv[3]))) {
				printf("is prime\n");
			} else {
				printf("is not prime\n");
			}

			mpz_clear(res);
		} else {
			printf("  -p [number] [tries]\t\toutput yes if number is probably prime, otherwise no\n");
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
	default:
		break;
	}

	return 0;
}
