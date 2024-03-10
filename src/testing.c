#include "testing.h"

void big_integer()
{
	char inputStr[1024];
	mpz_t n;

	printf ("Enter your number: ");
  	scanf("%1023s" , inputStr);

	mpz_init(n);
	mpz_set_ui(n,0);

	mpz_set_str(n,inputStr, 10); // n
	mpz_add_ui(n,n,1); // n + 1
	mpz_mul(n,n,n); // n * n

	mpz_out_str(stdout,10,n);
	mpz_clear(n);
}