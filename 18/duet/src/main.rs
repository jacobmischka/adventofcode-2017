use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
enum Instruction {
	Snd(i64),
	Set(char, i64),
	Add(char, i64),
	Mul(char, i64),
	Mod(char, i64),
	Rcv(char),
	Jgz(i64, i64)
}

#[derive(Debug)]
struct Cpu {
	registers: HashMap<char, i64>,
	pointer: usize,
	last_played_sound: Option<i64>
}

impl Cpu {
	fn new() -> Cpu {

		Cpu {
			registers: HashMap::new(),
			pointer: 0,
			last_played_sound: None
		}
	}

	fn get_reg_val(&self, reg: char) -> Option<i64> {
		// FIXME: If registers known ahead of time return None if not found
		match self.registers.get(&reg) {
			Some(&val) => Some(val),
			None => Some(0)
		}
	}

	fn get_val(&self, s: &str) -> Option<i64> {
		if let Ok(val) = s.parse::<i64>() {
			Some(val)
		} else if let Some(c) = s.chars().nth(0) {
			self.get_reg_val(c)
		} else {
			None
		}
	}

	fn get_instruction(&self, asm: &str) -> Option<Instruction> {
		use Instruction::*;

		let pieces: Vec<&str> = asm.split(" ").collect();
		if let Some(&instr_type) = pieces.get(0) {
			match instr_type {
				"snd" => {
					if let Some(s) = pieces.get(1) {
						if let Some(v) = self.get_val(s) {
							return Some(Snd(v));
						}
					}
				},
				"set" => {
					if let (Some(a_s), Some(b_s)) = (pieces.get(1), pieces.get(2)) {
						if let (Some(a_v), Some(b_v)) = (a_s.chars().nth(0), self.get_val(b_s)) {
							return Some(Set(a_v, b_v));
						}
					}
				},
				"add" => {
					if let (Some(a_s), Some(b_s)) = (pieces.get(1), pieces.get(2)) {
						if let (Some(a_v), Some(b_v)) = (a_s.chars().nth(0), self.get_val(b_s)) {
							return Some(Add(a_v, b_v));
						}
					}
				},
				"mul" => {
					if let (Some(a_s), Some(b_s)) = (pieces.get(1), pieces.get(2)) {
						if let (Some(a_v), Some(b_v)) = (a_s.chars().nth(0), self.get_val(b_s)) {
							return Some(Mul(a_v, b_v));
						}
					}
				},
				"mod" => {
					if let (Some(a_s), Some(b_s)) = (pieces.get(1), pieces.get(2)) {
						if let (Some(a_v), Some(b_v)) = (a_s.chars().nth(0), self.get_val(b_s)) {
							return Some(Mod(a_v, b_v));
						}
					}
				},
				"rcv" => {
					if let Some(s) = pieces.get(1) {
						if let Some(v) = s.chars().nth(0) {
							return Some(Rcv(v));
						}
					}
				},
				"jgz" => {
					if let (Some(a_s), Some(b_s)) = (pieces.get(1), pieces.get(2)) {
						if let (Some(a_v), Some(b_v)) = (self.get_val(a_s), self.get_val(b_s)) {
							return Some(Jgz(a_v, b_v));
						}
					}
				},
				_ => {}
			};
		}

		None
	}

	fn perform_instruction(&mut self, instr: Instruction) -> Option<i64> {
		use Instruction::*;

		match instr {
			Snd(x) => {
				self.last_played_sound = Some(x);
				self.pointer += 1;
				return Some(x);
			},
			Set(reg, y) => {
				self.registers.insert(reg, y);
				self.pointer += 1;
			},
			Add(reg, y) => {
				if let Some(x) = self.get_reg_val(reg) {
					self.registers.insert(reg, x + y);
					self.pointer += 1;
				} else {
					panic!("No register {}", reg);
				}
			},
			Mul(reg, y) => {
				if let Some(x) = self.get_reg_val(reg) {
					self.registers.insert(reg, x * y);
					self.pointer += 1;
				} else {
					panic!("No register {}", reg);
				}
			},
			Mod(reg, y) => {
				if let Some(x) = self.get_reg_val(reg) {
					self.registers.insert(reg, x % y);
					self.pointer += 1;
				} else {
					panic!("No register {}", reg);
				}
			},
			Rcv(reg) => {
				if let Some(x) = self.get_reg_val(reg) {
					if x != 0 {
						return self.last_played_sound;
					}
				}
				self.pointer += 1;
			},
			Jgz(x, y) => {
				if x > 0 {
					// There might be a better way to do this but this should work
					let diff = y.abs() as usize;
					if y < 0 {
						self.pointer -= diff;
					} else {
						self.pointer += diff;
					}
				} else {
					self.pointer += 1;
				}
			}
		}

		None
	}

	fn run(&mut self, asm: Vec<&str>) {
		loop {
			if let Some(asm_instr) = asm.get(self.pointer) {
				if let Some(instr) = self.get_instruction(asm_instr) {
					match instr {
						Instruction::Rcv(_) => {
							if let Some(played_sound) = self.perform_instruction(instr) {
								if played_sound != 0 {
									println!("Part 1: {}", played_sound);
									return;
								}
							}
						},
						_ => {
							self.perform_instruction(instr);
						}
					}
				} else {
					panic!("Couldn't get instruction from asm {}", asm_instr);
				}
			} else {
				panic!("Couldn't read asm line {}", self.pointer);
			}
		}
	}
}

#[derive(Debug)]
struct Program {
	registers: HashMap<char, i64>,
	pointer: usize,
	receive_queue: VecDeque<i64>,
	waiting: bool
}

impl Program {
	fn new(pid: i64) -> Program {

		let mut registers = HashMap::new();
		registers.insert('p', pid);

		Program {
			registers,
			pointer: 0,
			receive_queue: VecDeque::new(),
			waiting: false
		}
	}

	fn get_reg_val(&self, reg: char) -> Option<i64> {
		// FIXME: If registers known ahead of time return None if not found
		match self.registers.get(&reg) {
			Some(&val) => Some(val),
			None => Some(0)
		}
	}

	fn get_val(&self, s: &str) -> Option<i64> {
		if let Ok(val) = s.parse::<i64>() {
			Some(val)
		} else if let Some(c) = s.chars().nth(0) {
			self.get_reg_val(c)
		} else {
			None
		}
	}

	fn get_instruction(&self, asm: &str) -> Option<Instruction> {
		use Instruction::*;

		let pieces: Vec<&str> = asm.split(" ").collect();
		if let Some(&instr_type) = pieces.get(0) {
			match instr_type {
				"snd" => {
					if let Some(s) = pieces.get(1) {
						if let Some(v) = self.get_val(s) {
							return Some(Snd(v));
						}
					}
				},
				"set" => {
					if let (Some(a_s), Some(b_s)) = (pieces.get(1), pieces.get(2)) {
						if let (Some(a_v), Some(b_v)) = (a_s.chars().nth(0), self.get_val(b_s)) {
							return Some(Set(a_v, b_v));
						}
					}
				},
				"add" => {
					if let (Some(a_s), Some(b_s)) = (pieces.get(1), pieces.get(2)) {
						if let (Some(a_v), Some(b_v)) = (a_s.chars().nth(0), self.get_val(b_s)) {
							return Some(Add(a_v, b_v));
						}
					}
				},
				"mul" => {
					if let (Some(a_s), Some(b_s)) = (pieces.get(1), pieces.get(2)) {
						if let (Some(a_v), Some(b_v)) = (a_s.chars().nth(0), self.get_val(b_s)) {
							return Some(Mul(a_v, b_v));
						}
					}
				},
				"mod" => {
					if let (Some(a_s), Some(b_s)) = (pieces.get(1), pieces.get(2)) {
						if let (Some(a_v), Some(b_v)) = (a_s.chars().nth(0), self.get_val(b_s)) {
							return Some(Mod(a_v, b_v));
						}
					}
				},
				"rcv" => {
					if let Some(s) = pieces.get(1) {
						if let Some(v) = s.chars().nth(0) {
							return Some(Rcv(v));
						}
					}
				},
				"jgz" => {
					if let (Some(a_s), Some(b_s)) = (pieces.get(1), pieces.get(2)) {
						if let (Some(a_v), Some(b_v)) = (self.get_val(a_s), self.get_val(b_s)) {
							return Some(Jgz(a_v, b_v));
						}
					}
				},
				_ => {}
			};
		}

		None
	}

	fn add_to_receive_queue(&mut self, x: i64) {
		self.receive_queue.push_back(x);
	}

	fn perform_instruction(&mut self, instr: Instruction) -> Option<i64> {
		use Instruction::*;

		match instr {
			Snd(x) => {
				self.pointer += 1;
				return Some(x);
			},
			Set(reg, y) => {
				self.registers.insert(reg, y);
				self.pointer += 1;
			},
			Add(reg, y) => {
				if let Some(x) = self.get_reg_val(reg) {
					self.registers.insert(reg, x + y);
					self.pointer += 1;
				} else {
					panic!("No register {}", reg);
				}
			},
			Mul(reg, y) => {
				if let Some(x) = self.get_reg_val(reg) {
					self.registers.insert(reg, x * y);
					self.pointer += 1;
				} else {
					panic!("No register {}", reg);
				}
			},
			Mod(reg, y) => {
				if let Some(x) = self.get_reg_val(reg) {
					self.registers.insert(reg, x % y);
					self.pointer += 1;
				} else {
					panic!("No register {}", reg);
				}
			},
			Rcv(reg) => {
				if let Some(x) = self.receive_queue.pop_front() {
					self.registers.insert(reg, x);
					self.waiting = false;
					self.pointer += 1;
				} else {
					self.waiting = true;
				}
			},
			Jgz(x, y) => {
				if x > 0 {
					// There might be a better way to do this but this should work
					let diff = y.abs() as usize;
					if y < 0 {
						self.pointer -= diff;
					} else {
						self.pointer += diff;
					}
				} else {
					self.pointer += 1;
				}
			}
		}

		None
	}

	fn step(&mut self, asm: &Vec<&str>) -> Result<Option<i64>, String> {
		if let Some(asm_instr) = asm.get(self.pointer) {
			if let Some(instr) = self.get_instruction(asm_instr) {
				Ok(self.perform_instruction(instr))
			} else {
				Err(format!("Couldn't get instruction from asm {}", asm_instr))
			}
		} else {
			Err(format!("Couldn't read asm line {}", self.pointer))
		}
	}
}

fn part1() {
	let input = get_input();
	let asm = input.split("\n").collect();
	let mut cpu = Cpu::new();
	cpu.run(asm);
}

fn part2() {
	let input = get_input();
	let asm = input.split("\n").collect();
	let mut p0 = Program::new(0);
	let mut p1 = Program::new(1);

	let mut p0_running = true;
	let mut p1_running = true;

	let mut times_p1_sent = 0;
	loop {
		if p0_running {
			if let Ok(result) = p0.step(&asm) {
				if let Some(p0_sent) = result {
					p1.add_to_receive_queue(p0_sent);
				}
			} else {
				p0_running = false;
			}
		}

		if p1_running {
			if let Ok(result) = p1.step(&asm) {
				if let Some(p1_sent) = result {
					times_p1_sent += 1;
					p0.add_to_receive_queue(p1_sent);
				}
			} else {
				p1_running = false;
			}
		}

		if (!p0_running || p0.waiting) && (!p1_running || p1.waiting) {
			break;
		}
	}

	println!("Part 2: {}", times_p1_sent);
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
