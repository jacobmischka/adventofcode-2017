extern crate knot_rs;

use std::fs::File;
use std::io::Read;

use knot_rs::{
	reverse,
	knot_hash,
	LIST_SIZE
};

fn part1() {
	let input = get_input();
	let lengths: Vec<u32> = input.split(",")
		.map(|s| s.parse::<u32>().unwrap())
		.collect();

	let mut list = [0; LIST_SIZE];
	for i in 0..LIST_SIZE {
		list[i] = i as u32;
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

	println!("Part 2: {}", knot_hash(&input));
}

fn main() {
	part1();
	part2();
}

fn get_input() -> String {
	let mut file = File::open("../input/input.txt").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();

	contents.trim().to_string()
}
