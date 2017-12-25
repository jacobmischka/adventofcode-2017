use std::fmt;
use std::fs::File;
use std::io::Read;

struct CircleBuffer<T> {
	buf: Vec<T>,
	pos: usize,
	num_steps: usize
}

impl<T> CircleBuffer<T> {
	fn new(buf: Vec<T>, num_steps: usize) -> CircleBuffer<T> {
		CircleBuffer {
			buf,
			pos: 0,
			num_steps
		}
	}

	fn insert(&mut self, v: T) {
		self.pos = ((self.pos + self.num_steps) % self.buf.len()) + 1;
		self.buf.insert(self.pos, v);
	}

	fn get(&self, i: usize) -> &T {
		self.buf.get(i % self.buf.len()).unwrap()
	}
}

impl<T> fmt::Display for CircleBuffer<T> where T: fmt::Display {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let s: String = self.buf.iter().enumerate()
			.fold(String::new(), |mut s, (i, x)| {
				if i == self.pos {
					s.push_str(&format!(" ({}) ", x));
				} else {
					s.push_str(&format!("  {}  ", x));
				}

				s
			});

		writeln!(f, "{}", s)
	}
}

fn part1() {
	const NUM_ITERATIONS: u32 = 2017;
	let input = get_input();
	let num_steps = input.parse::<usize>().unwrap();

	let mut buf: CircleBuffer<u32> = CircleBuffer::new(vec![0], num_steps);

	for i in 1..(NUM_ITERATIONS + 1) {
		buf.insert(i);
	}

	println!("Part 1: {}", buf.get(buf.pos + 1));
}

fn part2() {
	const NUM_ITERATIONS: u32 = 50_000_000;
	let input = get_input();
	let num_steps = input.parse::<usize>().unwrap();

	let mut buf: CircleBuffer<u32> = CircleBuffer::new(vec![0], num_steps);

	for i in 1..(NUM_ITERATIONS + 1) {
		buf.insert(i);
	}

	let zero_pos = (&buf.buf).iter().position(|&x| x == 0).unwrap();

	println!("Part 2: {}", buf.get(zero_pos + 1));
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
