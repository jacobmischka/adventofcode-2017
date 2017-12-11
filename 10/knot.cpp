#include <iostream>
#include <iomanip>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>

using namespace std;

string getInput() {
	string input = "";
	string tmp;
	ifstream fp;
	fp.open("./input/input.txt");
	while (!fp.eof()) {
		getline(fp, tmp);
		input = input + tmp;
	}

	return input;
}

vector<string> split(string &s, char delimiter) {
	vector<string> pieces;
	string piece;
	istringstream stream(s);
	while (getline(stream, piece, delimiter)) {
		pieces.push_back(piece);
	}

	return pieces;
}

void print_arr(int arr[], int size) {
	cout << "[ ";
	for (int i = 0; i < size - 1; i++) {
		cout << arr[i] << ", ";
	}
	cout << arr[size - 1] << " ]" << endl;
}

void reverse(int arr[], int size, int start, int end) {
	int tmp;

	while (start <= end) {
		tmp = arr[start % size];
		arr[start % size] = arr[end % size];
		arr[end % size] = tmp;

		start++;
		end--;
	}
}

string to_hex(int arr[], int size) {
	stringstream stream;
	for (int i = 0; i < size; i++) {
		stream << hex << arr[i];
	}

	return stream.str();
}

void part1() {
	const int LIST_SIZE = 256;

	string str = getInput();
	vector<string> str_lengths = split(str, ',');
	vector<int> lengths;

	vector<string>::const_iterator s_iter;
	for (s_iter = str_lengths.begin(); s_iter != str_lengths.end(); s_iter++) {
		lengths.push_back(stoi(*s_iter));
	}

	int list[LIST_SIZE] = {};
	for (int i = 0; i < LIST_SIZE; i++) {
		list[i] = i;
	}

	int skip = 0;
	int position = 0;

	int length;
	vector<int>::const_iterator iter;
	for (iter = lengths.begin(); iter != lengths.end(); iter++) {
		length = *iter;

		reverse(list, LIST_SIZE, position, position + length - 1);

		position += length + skip;
		skip++;
	}

	cout << "Part 1: " << list[0] * list[1] << endl;
}

void part2() {
	const int LIST_SIZE = 256;
	const int NUM_ROUNDS = 64;
	const int BLOCK_SIZE = 16;

	string str = getInput();
	vector<int> lengths;
	for (int i = 0; i < str.length(); i++) {
		lengths.push_back((int)str[i]);
	}

	// Magic numbers from instructions
	lengths.push_back(17);
	lengths.push_back(31);
	lengths.push_back(73);
	lengths.push_back(47);
	lengths.push_back(23);

	int list[LIST_SIZE] = {};
	for (int i = 0; i < LIST_SIZE; i++) {
		list[i] = i;
	}

	int skip = 0;
	int position = 0;

	int length;
	vector<int>::const_iterator iter;
	for (int i = 0; i < NUM_ROUNDS; i++) {
		for (iter = lengths.begin(); iter != lengths.end(); iter++) {
			length = *iter;

			reverse(list, LIST_SIZE, position, position + length - 1);

			position += length + skip;
			skip++;
		}
	}

	int dense_hash_size = LIST_SIZE / BLOCK_SIZE;
	int dense_hash[dense_hash_size] = {};
	for (int i = 0; i < dense_hash_size; i++) {
		for (int j = 0; j < BLOCK_SIZE; j++) {
			dense_hash[i] = dense_hash[i] ^ list[i * dense_hash_size + j];
		}
	}

	cout << "Part 2: " << to_hex(dense_hash, dense_hash_size) << endl;
}

int main() {
	part1();
	part2();
}
