use std::fs::File;
use std::io::Read;

const GENERATOR_A_FACTOR: u64 = 16807;
const GENERATOR_B_FACTOR: u64 = 48271;
const DIVISOR: u64 = 2147483647;

struct Generator {
	factor: u64,
	previous_val: u64
}

impl Generator {
	fn new(factor: u64, previous_val: u64) -> Generator {
		Generator {
			factor,
			previous_val
		}
	}

	fn next_val(&mut self) -> u64 {
		let val = (self.previous_val * self.factor) % DIVISOR;
		self.previous_val = val;

		val
	}

	fn next_val_2(&mut self, criteria: u64) -> u64 {
		let mut val = self.next_val();
		while val % criteria != 0 {
			val = self.next_val();
		}

		val
	}
}

fn part1() {
	let input = get_input();
	let starts: Vec<u64> = input.split("\n")
		.map(|line| line.split(" ").collect())
		.map(|mut words: Vec<&str>| words.pop().unwrap().to_string())
		.map(|s| s.parse::<u64>().unwrap())
		.collect();

	let a_start = *starts.get(0).unwrap();
	let b_start = *starts.get(1).unwrap();

	let mut gen_a = Generator::new(GENERATOR_A_FACTOR, a_start);
	let mut gen_b = Generator::new(GENERATOR_B_FACTOR, b_start);

	let mut matches = 0;
	const NUM_PAIRS: usize = 40_000_000;
	for _ in 0..NUM_PAIRS {
		let mut new_a = pad_to_len(format!("{:b}", gen_a.next_val()), 16);
		let mut new_b = pad_to_len(format!("{:b}", gen_b.next_val()), 16);

		let a_len = new_a.len();
		let b_len = new_b.len();
		let new_a_end = new_a.split_off(a_len - 16);
		let new_b_end = new_b.split_off(b_len - 16);

		if new_a_end == new_b_end {
			matches += 1;
		}
	}

	println!("Part 1: {}", matches);
}

fn part2() {
	let input = get_input();
	let starts: Vec<u64> = input.split("\n")
		.map(|line| line.split(" ").collect())
		.map(|mut words: Vec<&str>| words.pop().unwrap().to_string())
		.map(|s| s.parse::<u64>().unwrap())
		.collect();

	let a_start = *starts.get(0).unwrap();
	let b_start = *starts.get(1).unwrap();

	let mut gen_a = Generator::new(GENERATOR_A_FACTOR, a_start);
	let mut gen_b = Generator::new(GENERATOR_B_FACTOR, b_start);

	let mut matches = 0;
	const NUM_PAIRS: usize = 5_000_000;
	for _ in 0..NUM_PAIRS {
		let mut new_a = pad_to_len(format!("{:b}", gen_a.next_val_2(4)), 16);
		let mut new_b = pad_to_len(format!("{:b}", gen_b.next_val_2(8)), 16);

		let a_len = new_a.len();
		let b_len = new_b.len();
		let new_a_end = new_a.split_off(a_len - 16);
		let new_b_end = new_b.split_off(b_len - 16);

		if new_a_end == new_b_end {
			matches += 1;
		}
	}

	println!("Part 2: {}", matches);
}

fn pad_to_len(mut bin: String, len: usize) -> String {
	while bin.len() < len {
		bin.insert(0, '0');
	}

	bin
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
