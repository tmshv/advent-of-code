package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func pow(x, y int) int {
	if y == 0 {
		return 1
	}
	val := x // x ^ 1
	for i := 2; i <= y; i++ {
		val = val * x
	}
	return val
}

func readInput() ([]int, int, error) {
	var size int
	values := []int{}
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		line := scanner.Text()
		size = len(line)
		val, err := strconv.ParseInt(line, 2, 64)
		if err != nil {
			return nil, 0, err
		}
		values = append(values, int(val))
	}

	if err := scanner.Err(); err != nil {
		return nil, 0, err
	}

	return values, size, nil
}

func solvePartOne(input []int, size int) int {
	ones := make([]int, size)
	for _, x := range input {
		for bit := 0; bit < size; bit++ {
			mask := pow(2, bit)
			if x&mask > 0 {
				ones[bit] += 1
				// log.Printf("one at %d, %d", x, bit)
			}
		}
	}
	inputLen := float64(len(input))
	gamma := 0
	for i, count := range ones {
		// one is common number at position I
		if float64(count)/inputLen > 0.5 {
			mask := pow(2, i)
			gamma = gamma | mask
		}
	}

	// invert gamma and trim high bits
	epsilon := ^gamma
	mask := (1 << size) - 1
	epsilon = epsilon & mask

	return gamma * epsilon
}

func filterByCommonBitCriteria(numbers []int, bit int, most bool) []int {
	mask := pow(2, bit)

	// Count all Ones in numbers at Bit position
	ones := 0.0
	for _, x := range numbers {
		if x&mask > 0 {
			ones += 1.0
		}
	}

	match := 0
	onesIsMostCommon := ones/float64(len(numbers)) >= 0.5
	if onesIsMostCommon && most {
		match = 1
	} else if !onesIsMostCommon && !most {
		match = 1
	}

	result := []int{}
	for _, x := range numbers {
		v := (x & mask) >> bit
		if v == match {
			result = append(result, x)
		}
	}
	return result
}

func solvePartTwo(input []int, size int) int {
	oxygen := 0
	numbers := input
	for i := size - 1; i >= 0; i-- {
		numbers = filterByCommonBitCriteria(numbers, i, true)
		if len(numbers) == 1 {
			oxygen = numbers[0]
			break
		}
	}

	co2 := 0
	numbers = input
	for i := size - 1; i >= 0; i-- {
		numbers = filterByCommonBitCriteria(numbers, i, false)
		if len(numbers) == 1 {
			co2 = numbers[0]
			break
		}
	}

	return oxygen * co2
}

func main() {
	input, size, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	var result int
	result = solvePartOne(input, size)
	fmt.Printf("Part one: %v\n", result)

	result = solvePartTwo(input, size)
	fmt.Printf("Part two: %v\n", result)
}
