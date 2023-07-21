package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Vector struct {
	x int
	y int
}

func (v *Vector) add(other *Vector) {
	v.x += other.x
	v.y += other.y
}

func readInput() ([]Vector, error) {
	values := []Vector{}
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, " ")
		if len(parts) != 2 {
			return nil, errors.New("Bad data")
		}

		dir := parts[0]
		val, err := strconv.Atoi(parts[1])
		if err != nil {
			return nil, err
		}

		v := Vector{0, 0}
		switch {
		case dir == "forward":
			v.x = val
		case dir == "down":
			v.y = val
		case dir == "up":
			v.y = -val
		default:
			return nil, errors.New("Bad data")
		}
		values = append(values, v)
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return values, nil
}

func solvePartOne(moves []Vector) int {
	position := Vector{0, 0}
	for _, x := range moves {
		position.add(&x)
	}
	return position.x * position.y
}

func solvePartTwo(moves []Vector) int {
	position := 0
	depth := 0
	aim := 0
	for _, m := range moves {
		if m.x == 0 {
			aim += m.y
		} else {
            position += m.x
            depth += m.x * aim
		}
	}
	return position * depth
}

func main() {
	moves, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	var result int
	result = solvePartOne(moves)
	fmt.Printf("Part one: %v\n", result)

	result = solvePartTwo(moves)
	fmt.Printf("Part two: %v\n", result)
}
