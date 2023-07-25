package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Fish struct {
	Amount   int
	Interval int
	TTL      int
}

func (fish *Fish) Reset() {
	fish.TTL = fish.Interval
}

func (fish *Fish) Tick() bool {
	if fish.TTL == 0 {
		fish.TTL = 6
		return true
	}
	fish.TTL -= 1
	return false
}

func readInput() ([]*Fish, error) {
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

	count := map[int]int{}
	for _, x := range numbers {
		count[x] += 1
	}

	fishes := []*Fish{}
	for interval, amount := range count {
		fish := Fish{
            Amount: amount,
            Interval: interval,
            TTL: interval,
        }
		fishes = append(fishes, &fish)
	}
	return fishes, nil
}

func solve(fishes []*Fish, cycles int) int {
	population := fishes
	for i := 0; i < cycles; i++ {
		newFishesInThisCycle := 0
		for _, fish := range population {
			if fish.Tick() {
				newFishesInThisCycle += fish.Amount
			}
		}
		if newFishesInThisCycle > 0 {
			fish := &Fish{
				Amount:   newFishesInThisCycle,
				Interval: 8,
				TTL:      8,
			}
			population = append(population, fish)
		}
	}

	total := 0
	for _, fish := range population {
		total += fish.Amount
	}

	return total
}

func solvePartOne(fishes []*Fish) int {
	return solve(fishes, 80)
}

func solvePartTwo(fishes []*Fish) int {
	return solve(fishes, 256)
}

func main() {
	fishes, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	var result int
	result = solvePartOne(fishes)
	fmt.Printf("Part one: %v\n", result)

    for _, f := range fishes {
        f.Reset()
    }

	result = solvePartTwo(fishes)
	fmt.Printf("Part two: %v\n", result)
}
