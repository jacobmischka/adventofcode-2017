extern crate knot_rs;

use std::fs::File;
use std::io::Read;

use knot_rs::knot_hash;

const NUM_ROWS: u32 = 128;

fn part1() {
	let input = get_input();
	let mut num_squares = 0;
	for row in 0..NUM_ROWS {
		let r = knot_hash(&format!("{}-{}", &input, row));
		let b = to_binary(&r);
		num_squares += b.chars().fold(0, |acc, b| match b {
			'1' => acc + 1,
			_ => acc
		});
	}

	println!("Part 1: {}", num_squares);
}

fn part2() {
	// TODO
}

fn main() {
	part1();
}

fn to_binary(input: &str) -> String {
	input.chars()
		.map(|c| format!("{:b}", c.to_digit(16).unwrap()))
		.map(|mut b| {
			while b.len() < 4 {
				b.insert(0, '0');
			}
			b
		})
		.fold(String::new(), |acc, b| acc + &b)
}

fn get_input() -> String {
	let mut file = File::open("../input/input.txt").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();

	contents.trim().to_string()
}
