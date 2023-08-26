package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func readInput() ([]string, error) {
	result := []string{}
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		num := scanner.Text()
		result = append(result, num)
	}
	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return result, nil
}

func solvePartOne(ns []string) int {
	numbers := make([]Snailfish, len(ns))
	for i, n := range ns {
		num, err := NewSnailfish(n)
		if err != nil {
			panic(err)
		}
		numbers[i] = num
	}

	sum := numbers[0]
	for _, n := range numbers[1:] {
		sum = sum.Add(n)
		sum.Reduce()
	}
	return sum.Magnitude()
}

func solvePartTwo(ns []string) int {
	max := 0
	for i, a := range ns {
		for j, b := range ns {
			if i == j {
				continue
			}

			sa, err := NewSnailfish(a)
			if err != nil {
				panic(err)
			}
			sb, _ := NewSnailfish(b)
			if err != nil {
				panic(err)
			}
			sum := sa.Add(sb)
			sum.Reduce()

			mag := sum.Magnitude()
			if mag > max {
				max = mag
			}
		}
	}
	return max
}

func main() {
	numbers, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	var result int

	result = solvePartOne(numbers)
	fmt.Printf("Part one: %v\n", result)

	result = solvePartTwo(numbers)
	fmt.Printf("Part two: %v\n", result)
}
