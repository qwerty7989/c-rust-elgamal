#include "text.h"

void integer_string_to_binary_string(char* str)
{
    mpz_t num;

	mpz_init(num);

    mpz_set_str(num, str, 10);
    mpz_get_str(str, 2, num);

	mpz_clear(num);
}

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


void decombined_private_string(char* str, long int key_size_bit, mpz_t n, mpz_t u)
{
    int i, j;
	char* tmp = malloc((key_size_bit+2)*sizeof(char));

    strcpy(tmp,"");

    for (i = 0; i < key_size_bit; i++) {
        tmp[i] = str[i];
    }
    tmp[i] = '\0';
    mpz_set_str(n, tmp, 2);

    for (j = 0; j < key_size_bit; j++, i++) {
        tmp[j] = str[i];
    }
    tmp[j] = '\0';
    mpz_set_str(u, tmp, 2);

    free(tmp);
}

void decombined_public_string(char* str, long int key_size_bit, mpz_t n, mpz_t g, mpz_t y)
{
    int i, j;
	char* tmp = malloc((key_size_bit+2)*sizeof(char));

    strcpy(tmp,"");

    for (i = 0; i < key_size_bit; i++) {
        tmp[i] = str[i];
    }
    tmp[i] = '\0';
    mpz_set_str(n, tmp, 2);

    for (j = 0; j < key_size_bit; j++, i++) {
        tmp[j] = str[i];
    }
    tmp[j] = '\0';
    mpz_set_str(g, tmp, 2);

    for (j = 0; j < key_size_bit; j++, i++) {
        tmp[j] = str[i];
    }
    tmp[j] = '\0';
    mpz_set_str(y, tmp, 2);

    free(tmp);
}

/**
 * https://www.geeksforgeeks.org/encode-ascii-string-base-64-format/
*/
void base64_encoder(char* input_str, int len_str)
{
    char char_set[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    char* res_str;
    res_str = malloc(MAX_LENGTH*sizeof(char));

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

/**
 * https://www.geeksforgeeks.org/decode-encoded-base-64-string-ascii-string/
*/
void base64_decoder(char* encoded, int len_str)
{
    char* decoded_string = malloc(MAX_LENGTH*sizeof(char));

    int i, j, k = 0;
    int num = 0;
    int count_bits = 0;

    for (i = 0; i < len_str; i += 4) {
        num = 0, count_bits = 0;
        for (j = 0; j < 4; j++) {
            if (encoded[i + j] != '=') {
                num = num << 6;
                count_bits += 6;
            }

            if (encoded[i + j] >= 'A' && encoded[i + j] <= 'Z')
                num = num | (encoded[i + j] - 'A');
            else if (encoded[i + j] >= 'a' && encoded[i + j] <= 'z')
                num = num | (encoded[i + j] - 'a' + 26);
            else if (encoded[i + j] >= '0' && encoded[i + j] <= '9')
                num = num | (encoded[i + j] - '0' + 52);
            else if (encoded[i + j] == '+')
                num = num | 62;
            else if (encoded[i + j] == '/')
                num = num | 63;
            else {
                num = num >> 2;
                count_bits -= 2;
            }
        }

        while (count_bits != 0) {
            count_bits -= 8;
            decoded_string[k++] = (num >> count_bits) & 255;
        }
    }

    decoded_string[k] = '\0';

    strcpy(encoded, decoded_string);

    free(decoded_string);
}