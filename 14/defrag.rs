use std::fs::File;
use std::io::Read;


fn main() {
	let input = get_input();

	println!("{}", input.trim());
}

fn get_input() -> String {
	let mut file = File::open("./input/input.txt").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();

	return contents;
}
