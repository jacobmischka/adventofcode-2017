use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Move {
	NW,
	N,
	NE,
	SE,
	S,
	SW
}

impl Move {
	fn from_str(i: &str) -> Result<Move, ()> {
		use Move::*;

		match i {
			"n" => Ok(N),
			"ne" => Ok(NE),
			"se" => Ok(SE),
			"s" => Ok(S),
			"sw" => Ok(SW),
			"nw" => Ok(NW),
			_ => Err(())
		}
	}
}

fn dec_move(map: &mut HashMap<Move, u32>, m: Move) -> bool {
	let v = map.entry(m).or_insert(0);
	if *v > 0 {
		*v -= 1;
		true
	} else {
		false
	}
}

fn inc_move(map: &mut HashMap<Move, u32>, m: Move) {
	let v = map.entry(m).or_insert(0);
	*v += 1;
}

fn get_distance_and_max(moves: &[Move]) -> (u32, u32) {
	use Move::*;

	let mut map: HashMap<Move, u32> = HashMap::new();
	let mut dist = 0;
	let mut max_dist = 0;
	for m in moves {
		match *m {
			N => {
				if dec_move(&mut map, S) {
					continue;
				} else if dec_move(&mut map, SE) {
					inc_move(&mut map, NE);
					continue;
				} else if dec_move(&mut map, SW) {
					inc_move(&mut map, NW);
					continue;
				}
			},
			NE => {
				if dec_move(&mut map, SW) {
					continue;
				} else if dec_move(&mut map, S) {
					inc_move(&mut map, SE);
					continue;
				} else if dec_move(&mut map, NW) {
					inc_move(&mut map, N);
					continue;
				}
			},
			NW => {
				if dec_move(&mut map, SE) {
					continue;
				} else if dec_move(&mut map, S) {
					inc_move(&mut map, SW);
					continue;
				} else if dec_move(&mut map, NE) {
					inc_move(&mut map, N);
					continue;
				}
			},
			S => {
				if dec_move(&mut map, N) {
					continue;
				} else if dec_move(&mut map, NE) {
					inc_move(&mut map, SE);
					continue;
				} else if dec_move(&mut map, NW) {
					inc_move(&mut map, SW);
					continue;
				}
			},
			SE => {
				if dec_move(&mut map, NW) {
					continue;
				} else if dec_move(&mut map, N) {
					inc_move(&mut map, NE);
					continue;
				} else if dec_move(&mut map, SW) {
					inc_move(&mut map, S);
					continue;
				}
			},
			SW => {
				if dec_move(&mut map, NE) {
					continue;
				} else if dec_move(&mut map, N) {
					inc_move(&mut map, NW);
					continue;
				} else if dec_move(&mut map, SE) {
					inc_move(&mut map, S);
					continue;
				}
			}
		}

		inc_move(&mut map, *m);
		dist = map.values().fold(0, |acc, &x| acc + x);
		if dist > max_dist {
			max_dist = dist;
		}
	}

	(dist, max_dist)
}


fn get_input() -> String {
	let mut file = File::open("./input/input.txt").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();

	return contents;
}

fn main() {
	let input = get_input();
	let moves: Vec<Move> = input.trim().split(",")
		.map(|s| Move::from_str(s).unwrap())
		.collect();

	let (dist, max) = get_distance_and_max(&moves[..]);
	println!("Part 1: {}", dist);
	println!("Part 2: {}", max);
}
