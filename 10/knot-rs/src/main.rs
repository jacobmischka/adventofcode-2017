extern crate hex;

use std::fs::File;
use std::io::Read;

const LIST_SIZE: usize = 256;
const NUM_ROUNDS: usize = 64;
const BLOCK_SIZE: usize = 16;

fn reverse(arr: &mut [u32], start: usize, end: usize) {
	let mut start = start;
	let mut end = end;
	let len = arr.len();

	while start <= end {
		let tmp = arr[start % len];
		arr[start % len] = arr[end % len];
		arr[end % len] = tmp;

		start += 1;
		end -= 1;
	}
}

pub fn knot_hash(input_lengths: &Vec<u32>) -> String {
	let mut lengths: Vec<u32> = Vec::new();
	lengths.clone_from_slice(input_lengths);

	// Magic numbers from instructions
	for length in [17, 31, 73, 47, 23].iter() {
		lengths.append(length as u32);
	}

	let mut list = [0; LIST_SIZE];
	for i in 0..LIST_SIZE {
		list[i] = i;
	}

	let mut skip = 0;
	let mut position = 0;
	for _ in 0..NUM_ROUNDS {
		for length in lengths {
			reverse(&mut list, position as usize, (position + length - 1) as usize);

			position += length + skip;
			skip += 1;
		}
	}

	const DENSE_HASH_SIZE: usize = LIST_SIZE / BLOCK_SIZE;
	let mut dense_hash: [u8; DENSE_HASH_SIZE] = [0; DENSE_HASH_SIZE];
	for i in 0..DENSE_HASH_SIZE {
		for j in 0..BLOCK_SIZE {
			dense_hash[i] = dense_hash[i] ^ list[i * DENSE_HASH_SIZE + j] as u8;
		}
	}

	hex::encode(dense_hash)
}

fn part1() {
	let input = get_input();
	let lengths: Vec<u32> = input.trim().split(",")
		.map(|s| s.parse::<u32>().unwrap())
		.collect();

	let mut list = [0; LIST_SIZE as usize];
	for i in 0..LIST_SIZE {
		list[i as usize] = i;
	}

	let mut skip = 0;
	let mut position = 0;
	for length in lengths {
		reverse(&mut list, position as usize, (position + length - 1) as usize);

		position += length + skip;
		skip += 1;
	}

	println!("Part 1: {}", list[0] * list[1]);
}

fn part2() {
	let input = get_input();
	let lengths: Vec<u32> = input.trim().split(",")
		.map(|s| s.parse::<u32>().unwrap())
		.collect();

	println!("Part 2: {}", knot_hash(&lengths));
}

fn main() {
	part1();
	part2();
}

fn get_input() -> String {
	let mut file = File::open("../input/input.txt").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();

	return contents;
}
