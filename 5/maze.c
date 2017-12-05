#include <stdio.h>
#include <stdlib.h>

typedef struct VecStruct {
	int* arr;
	int capacity;
	int size;
} Vec;

int* increaseArray(int* arr, int newSize) {

}

void reallocateVec(Vec* vec, int newSize) {
	int* newArr = malloc(sizeof(int) * newSize);
	for (int i = 0; i < vec->capacity; i++) {
		newArr[i] = vec->arr[i];
	}
	vec->arr = newArr;
}

Vec getInput() {
	int initialSize = 100;
	Vec vec = {
		.capacity = initialSize,
		.arr = malloc(sizeof(int) * initialSize),
		.size = 0
	};

	FILE *fp;
	char *line = NULL;
	size_t len = 0;
	ssize_t read;
	fp = fopen("./input/input.txt", "r");
	if (fp == NULL) {
		exit(1);
	}

	int instruction;
	while ((read = getline(&line, &len, fp)) != -1) {
		instruction = atoi(line);
		vec.arr[vec.size++] = instruction;
		if (vec.size >= vec.capacity) {
			reallocateVec(&vec, vec.capacity * 2);
		}
	}

	return vec;
}

// void printArr(int* arr) {
// 	int len = sizeof(arr) / sizeof(int);
// }

void part1() {
	int steps = 0;
	Vec instructions = getInput();


	// printf("Part 1: %s\n", steps);
}

int main() {
	part1();
}
