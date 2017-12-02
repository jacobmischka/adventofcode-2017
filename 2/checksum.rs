use std::fs::File;
use std::io::Read;

fn part1() {
	let input = get_input();
	let mut sum = 0;

	for line in input.trim().split("\n") {
		let nums = get_nums(line.to_string());
		let mut min = 0;
		let mut max = 0;

		for num in nums {
			if min == 0 || num < min {
				min = num;
			}
			if max == 0 || num > max {
				max = num;
			}
		}

		sum += max - min;
	}

	println!("Part 1: {}", sum);
}

fn part2() {
	let input = get_input();
	let mut sum = 0;

	for line in input.trim().split("\n") {
		let nums = get_nums(line.to_string());
		let size = nums.len();
		for i in 0..size {
			for j in (i + 1)..size {
				let mut n = nums[i] as f64;
				let mut d = nums[j] as f64;
				if n < d {
					let tmp = n;
					n = d;
					d = tmp;
				}

				let div = n / d;
				if div.fract() == 0.0 {
					sum += div as u32;
				}
			}
		}
	}

	println!("Part 2: {}", sum);
}

fn get_nums(s: String) -> Vec<u32> {
	s.trim().split("\t").map(|s| { s.parse::<u32>().unwrap() }).collect()
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
