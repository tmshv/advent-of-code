package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

func readInput() ([]int, error) {
	numbers := []int{}
	scanner := bufio.NewScanner(os.Stdin)

	// Parse numbers
	scanner.Scan()
	line := scanner.Text()
	for _, x := range strings.Split(line, ",") {
		val, err := strconv.ParseInt(x, 10, 64)
		if err != nil {
			return nil, err
		}
		numbers = append(numbers, int(val))
	}
	scanner.Scan() // Skip one line after numbers

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return numbers, nil
}

func median(numbers []int) int {
    sort.Ints(numbers)
    c := int(float64(len(numbers)) / 2.0)
    return numbers[c]
}

func solvePartOne(crabs []int) int {
    goal := median(crabs)
    fuel := 0.0
    for _, crab := range crabs {
        fuel += math.Abs(float64(crab - goal))
    }
    return int(fuel)
}

func solvePartTwo(numbers []int) int {
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
