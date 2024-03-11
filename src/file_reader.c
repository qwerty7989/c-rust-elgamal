#include "file_reader.h"

/**
 * Read 1 byte at a time
*/
int* read(char* filename)
{
	FILE *ptr;

	ptr = fopen(filename, "r");

	if (NULL == ptr)
		printf("file can't be opened \n");

	int i, ch;
	int* data;

	data = malloc(MAX_SIZE*sizeof(int));
	i = 0;
	do {
		ch = fgetc(ptr);
		data[i++] = ch;
	} while (ch != EOF && i < MAX_SIZE);

	fclose(ptr);
	return data;
}