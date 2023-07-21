package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func readInput() ([]int, error) {
	values := []int{}
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		val, err := strconv.Atoi(scanner.Text())
		if err != nil {
			return nil, err
		}
		values = append(values, val)
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return values, nil
}

func solvePartOne(input []int) int {
	count := 0
	for i, x := range input {
		if i == 0 {
			continue
		}

		if x > input[i-1] {
			count += 1
		}
	}
	return count
}

func solvePartTwo(input []int) int {
    // apply sliding window size 3 for input
    // next solve it as part one
	windows := make([]int, len(input) - 2)
	for i := 0; i < len(windows); i++ {
        total := 0
		total += input[i]
		total += input[i + 1]
		total += input[i + 2]
		windows[i] = total
	}

    return solvePartOne(windows)
}

func main() {
	measurements, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	var result int
	result = solvePartOne(measurements)
	fmt.Printf("Part one: %v\n", result)

	result = solvePartTwo(measurements)
	fmt.Printf("Part two: %v\n", result)
}
