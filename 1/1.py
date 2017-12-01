from utils import get_input

def main():
	checksum = get_input('input.txt').strip()

	s = 0
	length = len(checksum)
	for i, char in enumerate(checksum):
		if char == checksum[(i + 1) % length]:
			s += int(char)

	print(s)

if __name__ == '__main__':
	main()
