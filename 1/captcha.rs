use std::fs::File;
use std::io::Read;

fn part1() {
	let nums = get_nums(get_input());

	let mut sum = 0;

	for i in 0..nums.len() {
		if nums[i] == nums[(i + 1) % nums.len()] {
			sum += nums[i]
		}
	}

	println!("Part 1: {}", sum);
}

fn part2() {
	let nums = get_nums(get_input());

	let mut sum = 0;

	let size = nums.len();

	for i in 0..size {
		if nums[i] == nums[(i + (size / 2)) % size] {
			sum += nums[i]
		}
	}

	println!("Part 2: {}", sum);
}

fn get_nums(s: String) -> Vec<u32> {
	s.trim().bytes()
		.map(|b| { String::from_utf8(vec![b]).unwrap() })
		.map(|s| { s.parse::<u32>().unwrap() })
		.collect()
}

fn get_input() -> String {
	let mut file = File::open("./input/input.txt").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();

	return contents;
}

fn main() {
	part1();
	part2();
}
