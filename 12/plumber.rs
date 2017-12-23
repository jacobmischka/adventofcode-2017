use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;

fn part1() {
	let map = get_map();

	let mut zero_set: HashSet<u32> = HashSet::new();
	get_group(&map, 0, &mut zero_set);
	println!("Part 1: {}", zero_set.len());
}

fn part2() {
	let map = get_map();

	let mut set: HashSet<u32> = HashSet::new();
	let mut num_groups = 0;
	for program in map.keys() {
		if !set.contains(program) {
			num_groups += 1;
			get_group(&map, *program, &mut set);
		}
	}

	println!("Part 2: {}", num_groups);
}

fn get_map() -> HashMap<u32, HashSet<u32>> {
	let input = get_input();

	let mut map: HashMap<u32, HashSet<u32>> = HashMap::new();

	for line in input.trim().split("\n") {
		let pieces: Vec<&str> = line.split("<->").collect();
		let lhs = pieces[0].trim().parse::<u32>().unwrap();
		let rhs_programs: Vec<u32> = pieces[1].trim().split(",").map(|s| s.trim().parse::<u32>().unwrap()).collect();
		let lhs_set = map.entry(lhs).or_insert(HashSet::new());
		for prog in rhs_programs {
			lhs_set.insert(prog);
		}
	}

	map
}

fn get_group(map: &HashMap<u32, HashSet<u32>>, program: u32, set: &mut HashSet<u32>) {
	if let Some(program_set) = map.get(&program) {
		for p in program_set.iter() {
			if !set.contains(p) {
				set.insert(*p);
				get_group(map, *p, set);
			}
		}
	}
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
