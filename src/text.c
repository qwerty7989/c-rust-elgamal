#include "text.h"

void binary_string_to_integer_string(char* str)
{
    mpz_t num;

	mpz_init(num);

    mpz_set_str(num, str, 2);
    mpz_get_str(str, 10, num);

	mpz_clear(num);
}

void combined_private_string(char* str, mpz_t n, mpz_t u)
{
    int i;
	long unsigned int key_size_bit = mpz_sizeinbase(n, 2);
	char* tmp = malloc((key_size_bit+2)*sizeof(char));

    strcpy(str,"");

	mpz_get_str(tmp, 2, n);
    strcat(str, tmp);

    mpz_get_str(tmp, 2, u);
	for (i = 0; i < key_size_bit-strlen(tmp); i++) {
		str[key_size_bit+i] = '0';
	}
    str[key_size_bit+i] = '\0';
    strcat(str, tmp);

    free(tmp);
}

void combined_public_string(char* str, mpz_t n, mpz_t g, mpz_t y)
{
    int i;
	long unsigned int key_size_bit = mpz_sizeinbase(n, 2);
	char* tmp = malloc((key_size_bit+2)*sizeof(char));

    strcpy(str,"");

	mpz_get_str(tmp, 2, n);
    strcat(str, tmp);

    mpz_get_str(tmp, 2, g);
	for (i = 0; i < key_size_bit-strlen(tmp); i++) {
		str[key_size_bit+i] = '0';
	}
    str[key_size_bit+i] = '\0';
    strcat(str, tmp);

    mpz_get_str(tmp, 2, y);
	for (i = 0; i < key_size_bit-strlen(tmp); i++) {
		str[(key_size_bit*2)+i] = '0';
	}
    str[(key_size_bit*2)+i] = '\0';
    strcat(str, tmp);

    free(tmp);
}

/**
 * https://www.geeksforgeeks.org/encode-ascii-string-base-64-format/
*/
void base64Encoder(char* input_str, int len_str)
{
    // Character set of base64 encoding scheme
    char char_set[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    // Resultant string
    char *res_str = (char *) malloc(1000 * sizeof(char));

    int index, no_of_bits = 0, padding = 0, val = 0, count = 0, temp;
    int i, j, k = 0;

    for (i = 0; i < len_str; i += 3)
        {
            val = 0, count = 0, no_of_bits = 0;

            for (j = i; j < len_str && j <= i + 2; j++)
            {
                val = val << 8;
                val = val | input_str[j];
                count++;
            }

            no_of_bits = count * 8;
            padding = no_of_bits % 3;
            while (no_of_bits != 0)
            {
                if (no_of_bits >= 6)
                {
                    temp = no_of_bits - 6;
                    index = (val >> temp) & 63;
                    no_of_bits -= 6;
                }
                else
                {
                    temp = 6 - no_of_bits;
                    index = (val << temp) & 63;
                    no_of_bits = 0;
                }
                res_str[k++] = char_set[index];
            }
    }

    // padding is done here
    for (i = 1; i <= padding; i++)
    {
        res_str[k++] = '=';
    }

    res_str[k] = '\0';

    strcpy(input_str, res_str);

    free(res_str);
}