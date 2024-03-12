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

int* read_with_size(char* filename, long unsigned int* size)
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
	*size = i-1;

	fclose(fptr);
	return data;
}

char* read_char(char* filename)
{
	FILE *fptr;

	fptr = fopen(filename, "r");

	if (NULL == fptr) {
		printf("file can't be opened \n");
		return 0;
	}

	int i, ch;
	char* data;

	data = malloc(MAX_SIZE*sizeof(char));
	i = 0;
	do {
		ch = fgetc(fptr);
		data[i++] = ch;
	} while (ch != EOF && i < MAX_SIZE);
	data[i-1] = '\0';

	fclose(fptr);
	return data;
}

void write(char* filename, int* data, int size)
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

void write_char(char* filename, char* data, int size)
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