use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
	Asc,
	Desc
}

#[derive(Debug)]
struct FirewallLayer {
	depth: u32,
	range: u32,
	scanner_pos: u32,
	scanner_direction: Direction
}

impl FirewallLayer {
	fn step(&mut self) {
		match self.scanner_direction {
			Direction::Asc => {
				self.scanner_pos += 1;
			},
			Direction::Desc => {
				self.scanner_pos -= 1;
			}
		}

		if self.scanner_pos == self.range - 1 {
			self.scanner_direction = Direction::Desc;
		} else if self.scanner_pos == 0 {
			self.scanner_direction = Direction::Asc;
		}
	}
}

#[derive(Debug)]
struct Firewall {
	layers: HashMap<u32, FirewallLayer>,
	deepest_layer: u32
}

impl Firewall {
	fn from_input(input: String) -> Firewall {
		let mut layers = HashMap::new();
		let mut deepest_layer = 0;

		for line in input.trim().split("\n") {
			let pieces: Vec<&str> = line.split(":").collect();
			let depth = pieces[0].trim().parse::<u32>().unwrap();
			let range = pieces[1].trim().parse::<u32>().unwrap();
			layers.insert(depth, FirewallLayer {
				depth,
				range,
				scanner_pos: 0,
				scanner_direction: Direction::Asc
			});

			if depth > deepest_layer {
				deepest_layer = depth;
			}
		}

		Firewall {
			layers,
			deepest_layer
		}
	}

	fn step(&mut self) {
		for layer in self.layers.values_mut() {
			layer.step();
		}
	}
}

fn part1() {
	let input = get_input();
	let mut firewall = Firewall::from_input(input);
	let mut pos = 0;
	let mut severity = 0;
	while pos < firewall.deepest_layer {
		pos += 1;
		firewall.step();
		if let Some(layer) = firewall.layers.get(&pos) {
			if layer.scanner_pos == 0 {
				severity += layer.depth * layer.range;
			}
		}
	}

	println!("Part 1: {}", severity);
}

fn part2() {
	let input = get_input();
	let firewall = Firewall::from_input(input);
	let mut delay = 0;
	'delay_loop: loop {
		for layer in firewall.layers.values() {
			if (delay + layer.depth) % ((layer.range - 1) * 2) == 0 {
				delay += 1;
				continue 'delay_loop;
			}
		}

		break;
	}

	println!("Part 2: {}", delay);
}

fn main() {
	part1();
	part2();
}

fn get_input() -> String {
	let mut file = File::open("./input/input.txt").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();

	return contents;
}
