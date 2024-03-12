#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define MAX_SIZE 8192
#define BYTE_SIZE 8

int* read(char* filename);
char* read_char(char* filename);
void write(char* filename, char* data, int size);