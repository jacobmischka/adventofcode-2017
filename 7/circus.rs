use std::collections::HashMap;
use std::io::Read;
use std::fs::File;

#[derive(Debug)]
struct Program {
	name: String,
	weight: u32,
	children: Option<Vec<String>>
}

#[derive(Debug)]
struct Node {
	name: String,
	weight: u32,
	children: Option<Vec<Node>>
}

impl Program {
	fn from_line(line: &str) -> Program {
		let pieces: Vec<&str> = line.split("->").collect();
		let name_side = pieces[0];

		let name_side_pieces: Vec<&str> = name_side.split(" ").collect();
		let name = name_side_pieces[0].to_string();
		let weight = name_side_pieces[1].replace("(", "").replace(")", "").parse::<u32>().unwrap();
		let mut children: Option<Vec<String>> = None;

		if pieces.len() > 1 {
			children = Some(pieces[1].split(", ").map(|s| s.trim().to_string()).collect());
		}

		Program {
			name,
			weight,
			children
		}
	}

	fn nodeable(&self, node_map: &HashMap<String, Node>) -> bool {
		match self.children {
			Some(ref names) => {
				names.iter().all(|name| node_map.contains_key(name))
			},
			None => true
		}
	}
}

impl Node {
	fn from_program(program: &Program, node_map: &mut HashMap<String, Node>) -> Node {

		let children = match program.children {
			Some(ref program_children) => Some(
				program_children.iter()
					.map(|child_name|
						node_map.remove(child_name).unwrap()
					).collect()
			),
			None => None
		};

		Node {
			name: program.name.to_owned(),
			weight: program.weight,
			children
		}
	}

	fn total_weight(&self) -> u32 {
		self.weight + self.child_weight()
	}

	fn child_weight(&self) -> u32 {
		match self.children {
			Some(ref children) => children.iter().fold(0, |acc, ref child| acc + child.total_weight()),
			None => 0
		}
	}

	fn is_balanced(&self) -> bool {
		match self.children {
			Some(ref children) => {
				let mut the_same = true;
				for i in 0..children.len() - 1 {
					if children[i].total_weight() != children[i + 1].total_weight() {
						the_same = false;
					}
				}

				the_same
			},
			None => true
		}
	}
}

fn part1() -> Node {
	let input = get_input();
	let mut program_map = HashMap::new();
	for line in input.split("\n") {
		let program = Program::from_line(line);
		program_map.insert(program.name.to_owned(), program);
	}

	let mut node_map: HashMap<String, Node> = HashMap::new();

	while program_map.len() > 0 {
		let keys: Vec<String> = program_map.keys().map(|k| k.to_owned()).collect();
		for name in keys {
			{
				let program = program_map.get(&name).unwrap();
				if program.nodeable(&node_map) {
					let node = Node::from_program(&program, &mut node_map);
					node_map.insert(node.name.to_owned(), node);
				}
			}

			if node_map.contains_key(&name) {
				program_map.remove(&name);
			}
		}
	}

	let (_, node) = node_map.drain().last().unwrap();
	println!("Part 1: {}", node.name);

	node
}

fn part2(node: &Node) {
	if !node.is_balanced() {
		if let Some(ref children) = node.children {
			if children.iter().all(|child| child.is_balanced()) {
				let weights: Vec<u32> = children.iter().map(|child| child.total_weight()).collect();
				let mut counts: HashMap<u32, u32> = HashMap::new();
				for weight in weights {
					let count = counts.entry(weight).or_insert(0);
					*count += 1;
				}

				let (correct_weight, _) = counts.drain().find(|&(_, count)| count > 1).unwrap();
				let culprit = children.iter().find(|child| child.total_weight() != correct_weight).unwrap();

				println!("Part 2: {}", correct_weight - culprit.child_weight());
			} else {
				for child in children {
					part2(child);
				}
			}
		}
	}
}

fn main() {
	let node = part1();
	part2(&node);
}

fn get_input() -> String {
	let mut file = File::open("./input/input.txt").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();

	return contents.trim().to_string();
}
