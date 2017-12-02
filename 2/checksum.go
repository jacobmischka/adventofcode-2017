package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func part1() {
	f, err := os.Open("./input/input.txt")
	defer f.Close()
	if err != nil {
		panic(err)
	}

	scanner := bufio.NewScanner(f)
	checksum := 0

	for scanner.Scan() {
		pieces := strings.Split(scanner.Text(), "\t")
		var min int = -1
		var max int = -1

		nums := sstois(pieces)

		for _, num := range nums {
			if min == -1 || num < min {
				min = num
			}
			if max == -1 || num > max {
				max = num
			}
		}
		checksum += (max - min)
	}

	fmt.Printf("Part 1: %d\n", checksum)

	if err := scanner.Err(); err != nil {
		panic(err)
	}
}

func part2() {
	f, err := os.Open("./input/input.txt")
	defer f.Close()
	if err != nil {
		panic(err)
	}

	scanner := bufio.NewScanner(f)
	checksum := 0

	for scanner.Scan() {
		pieces := strings.Split(scanner.Text(), "\t")
		is := sstois(pieces)

		length := len(is)
	RowLoop:
		for i := 0; i < length-1; i++ {
			for j := i + 1; j < length; j++ {
				n := float64(is[i])
				d := float64(is[j])
				if d > n {
					tmp := n
					n = d
					d = tmp
				}

				if math.Remainder(n, d) == 0.0 {
					checksum += int(n / d)
					break RowLoop
				}
			}
		}
	}

	fmt.Printf("Part 2: %d\n", checksum)

	if err := scanner.Err(); err != nil {
		panic(err)
	}
}

func sstois(ss []string) []int {
	is := make([]int, len(ss))
	for i, v := range ss {
		num, err := strconv.Atoi(v)
		if err != nil {
			panic(err)
		}
		is[i] = num
	}

	return is
}

func main() {
	part1()
	part2()
}
