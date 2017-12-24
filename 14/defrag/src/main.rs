extern crate knot_rs;

use std::cmp;
use std::iter;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

use knot_rs::knot_hash;

const NUM_ROWS: usize = 128;
const NUM_COLS: usize = 128;

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

#[derive(Debug)]
struct Grid<T> {
	grid: Vec<Vec<T>>,
	num_rows: usize,
	num_cols: usize
}

impl<T> Grid<T> {
	fn new(grid: Vec<Vec<T>>, num_rows: usize, num_cols: usize) -> Grid<T> {
		Grid {
			grid,
			num_rows,
			num_cols
		}
	}

	fn get(&self, row_index: usize, col_index: usize) -> Option<&T> {
		if row_index > self.num_rows {
			return None;
		}
		if  col_index > self.num_cols {
			return None;
		}

		match self.grid.get(row_index) {
			Some(row) => {
				match row.get(col_index) {
					Some(cell) => Some(cell),
					None => None
				}
			},
			None => None
		}
	}

	fn set(&mut self, row_index: usize, col_index: usize, val: T) {
		if let Some(row) = self.grid.get_mut(row_index) {
			if row.len() < col_index {
				panic!("Grid vec not correct size");
			} else {
				row.push(val);
				row.swap_remove(col_index);
			}
		} else {
			panic!("Grid vec not correct size");
		}
	}
}

#[derive(Debug)]
struct DiskGroups {
	grid: Grid<Option<u32>>
}

impl DiskGroups {
	fn new(rows: Vec<Vec<Option<u32>>>, num_rows: usize, num_cols: usize) -> DiskGroups {
		let grid = Grid::new(rows, num_rows, num_cols);
		DiskGroups {
			grid
		}
	}

	fn get_group(&self, row: usize, col: usize) -> Option<u32> {
		match self.grid.get(row, col) {
			Some(v) => *v,
			None => None
		}
	}

	fn set_group(&mut self, row: usize, col: usize, group: u32) -> u32 {
		let mut group = group;

		if row > 0 {
			if let Some(g) = self.get_group(row - 1, col) {
				group = cmp::min(group, g);
			}
		}
		if let Some(g) = self.get_group(row + 1, col) {
			group = cmp::min(group, g);
		}
		if col > 0 {
			if let Some(g) = self.get_group(row, col - 1) {
				group = cmp::min(group, g);
			}
		}
		if let Some(g) = self.get_group(row, col + 1) {
			group = cmp::min(group, g);
		}

		self.grid.set(row, col, Some(group));

		if row > 0 {
			if let Some(g) = self.get_group(row - 1, col) {
				if g != group {
					self.set_group(row - 1, col, group);
				}
			}
		}
		if let Some(g) = self.get_group(row + 1, col) {
			if g != group {
				self.set_group(row + 1, col, group);
			}
		}

		if col > 0 {
			if let Some(g) = self.get_group(row, col - 1) {
				if g != group {
					self.set_group(row, col - 1, group);
				}
			}
		}
		if let Some(g) = self.get_group(row, col + 1) {
			if g != group {
				self.set_group(row, col + 1, group);
			}
		}

		group
	}
}

fn part2() {
	let input = get_input();
	let disk: Grid<char> = Grid::new((0..NUM_ROWS)
		.map(|row| knot_hash(&format!("{}-{}", &input, row)))
		.map(|r| to_binary(&r))
		.map(|r| r.chars().collect())
		.collect(), NUM_ROWS, NUM_COLS);

	let mut groups = DiskGroups::new((0..NUM_ROWS).map(|_| {
		iter::repeat(None).take(NUM_COLS as _).collect()
	}).collect(), NUM_ROWS, NUM_COLS);

	let mut working_group = 0;

	for row_index in 0..NUM_ROWS {
		for col_index in 0..NUM_COLS {
			if Some(&'1') == disk.get(row_index, col_index) {
				let set_group = groups.set_group(row_index, col_index, working_group);
				if set_group == working_group {
					working_group += 1
				}
			}
		}
	}

	let mut group_set: HashSet<u32> = HashSet::new();
	for row_index in 0..NUM_ROWS {
		for col_index in 0..NUM_COLS {
			if let Some(g) = groups.get_group(row_index, col_index) {
				group_set.insert(g);
			}
		}
	}


	println!("Part 2: {}", group_set.len());
}

fn main() {
	part1();
	part2();
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
