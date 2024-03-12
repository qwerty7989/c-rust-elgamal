#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define MAX_SIZE 40960
#define BYTE_SIZE 8

int* read(char* filename);
int* read_with_size(char* filename, long unsigned int* size);
char* read_char(char* filename);
void write(char* filename, int* data, int size);
void write_char(char* filename, char* data, int size);