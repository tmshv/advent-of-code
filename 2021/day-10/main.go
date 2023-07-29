package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func readInput() ([]string, error) {
	lines := []string{}
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return lines, nil
}

func solvePartOne(lines []string) int {
	pair := map[rune]rune{
		')': '(',
		']': '[',
		'}': '{',
		'>': '<',
	}
	cost := map[rune]int{
		')': 3,
		']': 57,
		'}': 1197,
		'>': 25137,
	}
	score := 0
	for _, line := range lines {
		stack := []rune{}
		for _, r := range line {
			if _, ok := pair[r]; ok {
				last := stack[len(stack)-1]
				stack = stack[:len(stack)-1]
				if pair[r] != last {
					score += cost[r]
					break
				}
			} else {
				stack = append(stack, r)
			}
		}
	}
	return score
}

func solvePartTwo(lines []string) int {
	count := 0
	return count
}

func main() {
	lines, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	var result int
	result = solvePartOne(lines)
	fmt.Printf("Part one: %v\n", result)

	result = solvePartTwo(lines)
	fmt.Printf("Part two: %v\n", result)
}
