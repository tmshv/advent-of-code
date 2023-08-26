package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func readInput() ([]Snailfish, error) {
	result := []Snailfish{}
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		row := scanner.Text()
		num, err := NewSnailfish(row)
		if err != nil {
			return nil, err
		}
		result = append(result, num)
	}
	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return result, nil
}

func solvePartOne(ns []Snailfish) int {
	sum := ns[0]
	for _, n := range ns[1:] {
		sum = sum.Add(n)
		sum.Reduce()
	}
	return sum.Magnitude()
}

func solvePartTwo(ns []Snailfish) int {
	return 0
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
