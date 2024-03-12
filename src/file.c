#include "file.h"

/**
 * Read 1 byte at a time
*/
int* read(char* filename)
{
	FILE *fptr;

	fptr = fopen(filename, "r");

	if (NULL == fptr) {
		printf("file can't be opened \n");
		return 0;
	}

	int i, ch;
	int* data;

	data = malloc(MAX_SIZE*sizeof(int));
	i = 0;
	do {
		ch = fgetc(fptr);
		data[i++] = ch;
	} while (ch != EOF && i < MAX_SIZE);

	fclose(fptr);
	return data;
}

void write(char* filename, char* data, int size)
{
	FILE *fptr;

	fptr = fopen(filename, "w");

	if (NULL == fptr) {
		printf("file can't be opened \n");
		return;
	}

	int i, ch;

	i = 0;
	do {
		ch = data[i++];
		fputc(ch, fptr);
	} while (i < size);

	fclose(fptr);
}