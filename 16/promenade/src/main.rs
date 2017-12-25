use std::fmt;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq, Eq)]
enum DanceMove {
	Spin(usize),
	Exchange(usize, usize),
	Partner(char, char)
}

#[derive(Debug)]
struct Dancers {
	programs: Vec<char>
}

impl Dancers {
	fn new(programs: Vec<char>) -> Dancers {
		Dancers { programs }
	}

	fn perform_move(&mut self, dance_move: &DanceMove) {
		use DanceMove::*;

		match *dance_move {
			Spin(num_spins) => {
				// TODO
				let at = self.programs.len() - num_spins;
				let mut tmp = self.programs.split_off(at);
				tmp.append(&mut self.programs);
				self.programs = tmp;
			},
			Exchange(a, b) => {
				self.programs.swap(a, b);
			},
			Partner(a, b) => {
				let a_index = (self.programs[..].iter())
					.position(|&x| x == a)
					.unwrap();
				let b_index = (self.programs[..].iter())
					.position(|&x| x == b)
					.unwrap();

				self.programs.swap(a_index, b_index);
			}
		}
	}
}

impl fmt::Display for Dancers {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let s = self.programs[..].iter().fold(String::new(), |mut acc, &c| { acc.push(c); acc });
		write!(f, "{}", s)
	}
}

impl DanceMove {
	fn from_str(s: &str) -> Result<DanceMove, ()> {
		use DanceMove::*;
		let (move_type, params) = s.split_at(1);
		match move_type {
			"s" => Ok(Spin(params.parse::<usize>().unwrap())),
			"x" => {
				let params: Vec<usize> = params.split("/")
					.map(|s| s.parse::<usize>().unwrap())
					.collect();

				if let (Some(a), Some(b)) = (params.get(0), params.get(1)) {
					Ok(Exchange(*a, *b))
				} else {
					Err(())
				}
			},
			"p" => {
				let pieces: Vec<char> = params.chars().collect();

				if let (Some(a), Some(b)) = (pieces.get(0), pieces.get(2)) {
					Ok(Partner(*a, *b))
				} else {
					Err(())
				}
			},
			_ => Err(())
		}
	}
}

fn part1() {
	let input = get_input();

	let mut dancers = Dancers::new("abcdefghijklmnop".chars().collect());
	for move_str in input.split(",") {
		let dance_move = DanceMove::from_str(move_str).unwrap();
		dancers.perform_move(&dance_move);
	}

	println!("Part 1: {}", dancers);
}

fn part2() {
	const NUM_ITERATIONS: usize = 1_000_000_000;
	let input = get_input();

	let start: Vec<char> = "abcdefghijklmnop".chars().collect();
	let mut dancers = Dancers::new(start.clone());
	let mut pos_set: HashSet<String> = HashSet::new();
	pos_set.insert(format!("{}", dancers));

	let mut num_until_repeat = 0;
	loop {
		for move_str in input.split(",") {
			let dance_move = DanceMove::from_str(move_str).unwrap();
			dancers.perform_move(&dance_move);
		}
		num_until_repeat += 1;

		if pos_set.contains(&format!("{}", dancers)) {
			break;
		} else {
			pos_set.insert(format!("{}", dancers));
		}
	}

	for i in 0..(NUM_ITERATIONS % num_until_repeat) {
		for move_str in input.split(",") {
			let dance_move = DanceMove::from_str(move_str).unwrap();
			dancers.perform_move(&dance_move);
		}
	}

	println!("Part 2: {}", dancers);
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
