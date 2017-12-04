import java.io.File
import kotlin.collections.Set
import kotlin.collections.HashSet

fun part1() {
	var numValid = 0
	
	for (line in getInput()) {
		if (isPassphraseValid(line.trim()))
			numValid++
	}
	
	println("Part 1: " + numValid)
}

fun part2() {
	var numValid = 0
	
	for (line in getInput()) {
		if (isPassphraseValid2(line.trim()))
			numValid++
	}
	
	println("Part 2: " + numValid)
}

fun isPassphraseValid(passphrase: String): Boolean {
	val set: HashSet<String> = HashSet<String>()
	
	for (word in passphrase.split(" ")) {
		if (set.contains(word))
			return false
		
		set.add(word)
	}
	
	return true
}

fun isPassphraseValid2(passphrase: String): Boolean {
	val set: HashSet<String> = HashSet<String>()
	
	for (word in passphrase.split(" ")) {
		for (permutation in getPermutations(word)) {
			if (set.contains(permutation))
				return false
		}
			
		set.add(word)
	}
	
	return true
}

fun getPermutations(str: String): List<String> {
	return getPermutations("", str)
}

fun getPermutations(prefix: String, str: String): List<String> {
	val list: ArrayList<String> = ArrayList<String>()
	
	if (str.length == 1) {
		list.add(prefix + str)
	} else {
		for (i in str.indices) {
			list.addAll(getPermutations(prefix + str[i], str.substring(0, i) + str.substring(i + 1)))
		}
	}
	
	
	return list
}

fun getInput(): List<String> {
	return File("./input/input.txt").readLines()
}

fun main() {
	part1()
	part2()
}

main()