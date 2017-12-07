#include <stdio.h>
#include <stdlib.h>

typedef struct VecStruct {
	int* arr;
	int capacity;
	int size;
} Vec;

typedef struct InStructions {
	Vec vec;
	int counter;
} Instructions;

void printVecHighlight(Vec* vec, int highlight) {
	printf("[ ");
	for (int i = 0; i < vec->size - 1; i++) {
		if (i == highlight)
			printf("(%d), ", vec->arr[i]);
		else
			printf("%d, ", vec->arr[i]);
	}
	printf("%d ", vec->arr[vec->size - 1]);
	printf("]\n");
}

void printVec(Vec* vec) {
	printVecHighlight(vec, -1);
}

void reallocateVec(Vec* vec, int newSize) {
	int* newArr = malloc(sizeof(int) * newSize);
	for (int i = 0; i < vec->capacity; i++) {
		newArr[i] = vec->arr[i];
	}
	free(vec->arr);
	vec->arr = newArr;
	vec->capacity = newSize;
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
		fprintf(stderr, "ERROR: input not found\n");
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

void part1() {
	long steps = 0;
	Vec instructions = getInput();

	long counter = 0;
	int tmp;
	while (counter < instructions.size) {
		tmp = counter;
		counter += instructions.arr[counter];
		instructions.arr[tmp] += 1;
		steps++;
	}

	printf("Part 1: %d\n", steps);
}

void part2() {
	long steps = 0;
	Vec instructions = getInput();

	long counter = 0;
	int tmp;
	while (counter < instructions.size) {
		tmp = counter;
		counter += instructions.arr[counter];
		if (instructions.arr[tmp] >= 3) {
			instructions.arr[tmp] -= 1;
		} else {
			instructions.arr[tmp] += 1;
		}
		steps++;
	}

	printf("Part 2: %d\n", steps);
}


int main() {
	part1();
	part2();
}
