import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.FileSystems;

import java.util.Arrays;
import java.util.HashMap;
import java.util.List;

class Cpu {
	final static int REGISTER = 0;
	final static int OPERATION = 1;
	final static int OPERAND = 2;
	final static int CONDITION_REG = 4;
	final static int COMPARATOR = 5;
	final static int CONDITION_VAL = 6;
	
	HashMap<String, Integer> registers;
	
	Cpu() {
		registers = new HashMap<>();
	}
	
	int getRegister(String regName) {
		return registers.getOrDefault(regName, 0);
	}
	
	boolean conditional(String reg, String comparator, int val) {
		int regVal = getRegister(reg);
		switch (comparator) {
			case "==":
				return regVal == val;
			case ">":
				return regVal > val;
			case "<":
				return regVal < val;
			case "<=":
				return regVal <= val;
			case ">=":
				return regVal >= val;
			case "!=":
				return regVal != val;
		}
		
		return false;
	}
	
	void adjustRegister(String register, String operation, int operand) {
		switch (operation) {
			case "inc":
				// Nothing to do
				break;
			case "dec":
				operand *= -1;
				break;
			default:
				return;
		}
		
		registers.put(register, getRegister(register) + operand);
	}
	
	int max() {
		return registers.values().stream()
			.reduce(0, (max, val) -> val > max ? val : max);
	}
}



class Registers {
	
	
	static void part1() {
		List<String> instructions = getInput();
		Cpu cpu = new Cpu();
		
		String[] pieces;
		for (String line : instructions) {
			pieces = line.split(" ");
			
			if (cpu.conditional(
				pieces[Cpu.CONDITION_REG],
				pieces[Cpu.COMPARATOR],
				Integer.parseInt(pieces[Cpu.CONDITION_VAL])
			))
				cpu.adjustRegister(
					pieces[Cpu.REGISTER],
					pieces[Cpu.OPERATION],
					Integer.parseInt(pieces[Cpu.OPERAND])
				);
		}
		
		int max = cpu.max();
		System.out.println("Part 1: " + max);
	}
	
	static void part2() {
		List<String> instructions = getInput();
		Cpu cpu = new Cpu();
		int max = 0;
		
		String[] pieces;
		int currentMax;
		for (String line : instructions) {
			pieces = line.split(" ");
			
			if (cpu.conditional(
				pieces[Cpu.CONDITION_REG],
				pieces[Cpu.COMPARATOR],
				Integer.parseInt(pieces[Cpu.CONDITION_VAL])
			)) {
				cpu.adjustRegister(
					pieces[Cpu.REGISTER],
					pieces[Cpu.OPERATION],
					Integer.parseInt(pieces[Cpu.OPERAND])
				);
				currentMax = cpu.max();
				if (currentMax > max)
					max = currentMax;
			}
		}
		
		System.out.println("Part 2: " + max);
	}
	
	static List<String> getInput() {
		try {
			return Files.readAllLines(
				FileSystems.getDefault().getPath("input", "input.txt")
			);
		} catch (IOException e) {
			System.err.println("Failed to read input");
			System.exit(1);
			return null;
		}
	}
	
	public static void main(String[] args) {
		part1();
		part2();
	}
}