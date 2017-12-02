import os

def part1():
	captcha = get_input('input.txt').strip()

	s = 0
	length = len(captcha)
	for i, char in enumerate(captcha):
		if char == captcha[(i + 1) % length]:
			s += int(char)

	print('Part 1: {}'.format(s))

def part2():
	captcha = get_input('input.txt').strip()

	s = 0
	length = len(captcha)
	for i, char in enumerate(captcha):
		if char == captcha[(i + int(length / 2)) % length]:
			s += int(char)

	print('Part 2: {}'.format(s))

def get_input(filename):
	filepath = os.path.join(os.path.dirname(os.path.realpath(__file__)), 'input', filename)
	with open(filepath) as infile:
		return infile.read()

if __name__ == '__main__':
	part1()
	part2()
