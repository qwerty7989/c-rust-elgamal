#include "gcd.h"
#include "modulo.h"
#include "prime.h"
#include "file.h"
#include "elgamal.h"
#include "text.h"

int main(int argc, char* argv[])
{
	if (argc == 1 || (argc > 1 && argv[1][0] != '-')) {
		printf("Usage: crel [OPTION] ...\n");
		printf("  -g <file> [bits]\t\tgenerate prime number from file\n");
		printf("  -e [number]\t\t\tgenerate random number with modulo inverse\n");
		printf("  -r [number] [modulus]\t\toutput modulo inverse of input number\n");
		printf("  -d [number1] [number2]\toutput gcd(number1, number2)\n");
		printf("  -p [number] [tries]\t\toutput yes if number is probably prime, otherwise no\n");
		printf("  -o <file>\t\t\tReading the file data, return data if readable\n");
		printf("  -k [number]\t\t\tgenerate key for elgamal\n");
		printf("  -k [number] <pub> <key_file>\tgenerate key for elgamal and save to file\n");
		printf("  -a [p] [g] [y] [number]\tencrypt number with elgamal\n");
		printf("  -a <pub> [number]\tencrypt number with elgamal\n");
		printf("  -b [p] [u] [a] [b]\t\tdecrypt number with elgamal\n");
		printf("  -b <key_file> [a] [b]\t\tdecrypt number with elgamal\n");
		printf("  -m [p] [g] [y] <file>\t\tencrypt file with elgamal\n");
		printf("  -m <pub> <file>\t\tencrypt file with elgamal\n");
		printf("  -n [p] [u] <file>\t\tdecrypt file with elgamal\n");
		printf("  -n <key_file> <file>\t\tdecrypt file with elgamal\n");
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
			printf("  -g <file> [bits]\t\tgenerate prime number from file\n");
			printf("crel: please enter both filename and bits\n");
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
			printf("crel: please enter number\n");
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
			printf("crel: please enter both remainder and modulus\n");
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
			printf("crel: please enter both numbers\n");
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
			printf("crel: please enter both the number and tries\n");
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
			printf("crel: please enter filename\n");
		}
		break;
	case 'q':
		if (argc == 3) {
			mpz_t n, g, y, u, a, b, x;

			mpz_inits(n, g, y, u, a, b, x, NULL);

			mpz_set_str(n, argv[2], 10);

			elgamal_keygen(n, g, y, u);

			printf("Keygen\n");
			gmp_printf("n %Zd\ng %Zd\ny %Zd\nu %Zd\n", n, g, y, u);

			mpz_set_si(x, 120);
			elgamal_encrypt_number(n, g, y, a, b, x);

			printf("\nEncrypt\n");
			gmp_printf("n %Zd\ng %Zd\ny %Zd\nu %Zd\na %Zd\nb %Zd\nx %Zd\n", n, g, y, u, a, b, x);

			printf("\nDecrypt\n");
			elgamal_decrypt_number(n, u, a, b, x);
			gmp_printf("n %Zd\ng %Zd\ny %Zd\nu %Zd\na %Zd\nb %Zd\nx %Zd\n", n, g, y, u, a, b, x);

			mpz_clears(n, g, y, u, a, b, x, NULL);
		} else {
			printf("crel: New Function\n");
		}
		break;
	case 'k':
		if (argc == 3) {
			mpz_t n, g, y, u;

			mpz_inits(n, g, y, u, NULL);

			mpz_set_str(n, argv[2], 10);

			elgamal_keygen(n, g, y, u);

			gmp_printf("public key:\n  p=%Zd\n  g=%Zd\n  y=%Zd\nprivate key:\n  u=%Zd\n", n, g, y, u);

			mpz_clears(n, g, y, u, NULL);
		} else if (argc == 5) {
			mpz_t n, g, y, u;

			mpz_inits(n, g, y, u, NULL);

			mpz_set_str(n, argv[2], 10);

			elgamal_keygen(n, g, y, u);

			long unsigned int key_size_bit = mpz_sizeinbase(n, 2);
			char* res_pub = malloc(((key_size_bit+2)*3)*sizeof(char));
			char* res_pri = malloc(((key_size_bit+2)*3)*sizeof(char));
			char* pub_str = malloc(((key_size_bit+2)*4)*sizeof(char));
			char* pri_str = malloc(((key_size_bit+2)*4)*sizeof(char));

			//n, g, y
			combined_public_string(res_pub, n, g, y);
			binary_string_to_integer_string(res_pub);
			base64_encoder(res_pub, strlen(res_pub));

			sprintf(pub_str, "%lu", key_size_bit);
			strcat(pub_str, " ");
			strcat(pub_str, res_pub);
			write(argv[3], pub_str, strlen(pub_str));

			//u
			combined_private_string(res_pri, n, u);
			binary_string_to_integer_string(res_pri);
			base64_encoder(res_pri, strlen(res_pri));

			sprintf(pri_str, "%lu", key_size_bit);
			strcat(pri_str, " ");
			strcat(pri_str, res_pri);
			write(argv[4], pri_str, strlen(pri_str));

			free(res_pub);
			free(res_pri);
			free(pub_str);
			free(pri_str);

			mpz_clears(n, g, y, u, NULL);

		} else {
			printf("  -k [number]\t\t\tgenerate key for elgamal\n");
			printf("  -k [number] <pub> <key_file>\tgenerate key for elgamal and save to file\n");
			printf("crel: please enter number or both number and file to save\n");
		}
		break;
	case 'a':
		if (argc == 4) {
			int i, j;
			mpz_t x, g, y, k;
			mpz_inits(x, g, y, k, NULL);

			char* data = read_char(argv[2]);
			char* key_size_bit = malloc(MAX_SIZE*sizeof(char));

			i = 0;
			while (!isspace(data[i])) {
				key_size_bit[i] = data[i];
				i++;
			}
			key_size_bit[i++] = '\0';

			j = 0;
			while (data[i] != '\0') {
				data[j++] = data[i++];
			}
			data[j] = '\0';

			base64_decoder(data, strlen(data));

			integer_string_to_binary_string(data);
			printf("key %s data %s\n", key_size_bit, data);
			decombined_public_string(data, key_size_bit, g, y, k);

			//mpz_set_str(x, argv[3], 10);

		} else if (argc == 6) {
			mpz_t n, g, y, a, b, x;

			mpz_inits(n, g, y, a, b, x, NULL);

			mpz_set_str(n, argv[2], 10);
			mpz_set_str(g, argv[3], 10);
			mpz_set_str(y, argv[4], 10);
			mpz_set_str(x, argv[5], 10);

			elgamal_encrypt_number(n, g, y, a, b, x);

			gmp_printf("a=%Zd\nb=%Zd\n", a, b);

			mpz_clears(n, g, y, a, b, x, NULL);
		} else {
			printf("  -a [p] [g] [y] [number]\tencrypt number with elgamal\n");
			printf("  -a <pub> [number]\tencrypt number with elgamal\n");
			printf("crel: please enter all required parameter\n");
		}
		break;
	case 'b':
		if (argc == 5) {

		} else if (argc == 6) {
			mpz_t n, u, a, b, x;

			mpz_inits(n, u, a, b, x, NULL);

			mpz_set_str(n, argv[2], 10);
			mpz_set_str(u, argv[3], 10);
			mpz_set_str(a, argv[4], 10);
			mpz_set_str(b, argv[5], 10);

			elgamal_decrypt_number(n, u, a, b, x);

			gmp_printf("x=%Zd\n", x);

			mpz_clears(n, u, a, b, x, NULL);

		} else {
			printf("  -b [p] [u] [a] [b]\t\tdecrypt number with elgamal\n");
			printf("  -b <key_file> [a] [b]\t\tdecrypt number with elgamal\n");
			printf("crel: please enter all required parameter\n");
		}
		break;
	case 'm':
		if (argc == 4) {
		} else if (argc == 6) {
		} else {
			printf("  -m [p] [g] [y] <file>\t\tencrypt file with elgamal\n");
			printf("  -m <pub> <file>\t\tencrypt file with elgamal\n");
			printf("crel: please enter filename and all required parameter\n");
		}
		break;
	case 'n':
		if (argc == 4) {
		} else if (argc == 5) {
		} else {
			printf("  -n [p] [u] <file>\t\tdecrypt file with elgamal\n");
			printf("  -n <key_file> <file>\t\tdecrypt file with elgamal\n");
			printf("crel: please enter filename and all required parameter\n");
		}
	default:
		printf("crel: invalid option\n");
		break;
	}

	return 0;
}



// save block
	//File encrypt/decrypt = P size in bit - 1
// decrypt block
// gen safe prime
// save key to file
	//private key
	//  p = 10001011
	//  u = 00000011
	//8 base64(1000101100000011)
	//public key
	//  p = 10001011
	//  g = 00110111
	//  y = 01100011
	//8 base64(100010110011011101100011)