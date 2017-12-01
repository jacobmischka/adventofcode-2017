import os

def get_input(filename):
	filepath = os.path.join(os.path.dirname(os.path.realpath(__file__)), 'input', filename)
	with open(filepath) as infile:
		return infile.read()
