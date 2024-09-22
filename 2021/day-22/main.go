package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"regexp"
	"strconv"
)

type vector struct {
	x float64
	y float64
	z float64
}

type cube struct {
	min vector
	max vector
	val int
}

func readInput() ([]cube, error) {
	instructions := []cube{}
	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		row := scanner.Text()

		pattern := `(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)`
		re := regexp.MustCompile(pattern)
		submatches := re.FindStringSubmatch(row)

		// Extract the values from submatches
		on := submatches[1] == "on"
		xStart, _ := strconv.Atoi(submatches[2])
		xEnd, _ := strconv.Atoi(submatches[3])
		yStart, _ := strconv.Atoi(submatches[4])
		yEnd, _ := strconv.Atoi(submatches[5])
		zStart, _ := strconv.Atoi(submatches[6])
		zEnd, _ := strconv.Atoi(submatches[7])

		val := 1
		if !on {
			val = -1
		}
		i := cube{
			min: *NewFromInt(xStart, yStart, zStart),
			max: *NewFromInt(xEnd, yEnd, zEnd),
			val: val,
		}
		instructions = append(instructions, i)
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return instructions, nil
}

func (c1 *cube) intersect(c2 *cube) (*cube, bool) {
	if c1.max.x < c2.min.x || c1.min.x > c2.max.x || c1.max.y < c2.min.y || c1.min.y > c2.max.y || c1.max.z < c2.min.z || c1.min.z > c2.max.z {
		return nil, false
	}

	return &cube{
		val: c2.val,
		min: vector{
			x: math.Max(c1.min.x, c2.min.x),
			y: math.Max(c1.min.y, c2.min.y),
			z: math.Max(c1.min.z, c2.min.z),
		},
		max: vector{
			x: math.Min(c1.max.x, c2.max.x),
			y: math.Min(c1.max.y, c2.max.y),
			z: math.Min(c1.max.z, c2.max.z),
		},
	}, true
}

func intersections(c *cube, cubes []*cube) []*cube {
	var ics []*cube
	for _, cub := range cubes {
		if i, ok := c.intersect(cub); ok {
			ics = append(ics, i)
		}
	}
	return ics
}

func volume(cube *cube) int {
	w := cube.max.x - cube.min.x + 1
	h := cube.max.y - cube.min.y + 1
	d := cube.max.z - cube.min.z + 1
	return cube.val * int(w*h*d)
}

func solve(cubes []*cube) int {
	var next []*cube
	for _, cub := range cubes {
		ins := intersections(cub, next)
		for _, i := range ins {
			i.val *= -1
		}
		next = append(next, ins...)

		if cub.val == 1 {
			next = append(next, cub)
		}
	}

	var count int
	for _, c := range next {
		count += volume(c)
	}
	return count
}

func solvePartOne(ix []cube) int {
	bounds := Box3{
		Min: vector{-50, -50, -50},
		Max: vector{50, 50, 50},
	}

	var cubes []*cube

	for _, i := range ix {
		box := NewFromMinMax(i.min, i.max)
		if !box.IntersectsBox(&bounds) {
			continue
		}

		cube := i
		cubes = append(cubes, &cube)
	}

	return solve(cubes)
}

func solvePartTwo(ix []cube) int {
	var cubes []*cube

	for _, i := range ix {
		cube := i
		cubes = append(cubes, &cube)
	}

	return solve(cubes)
}

func main() {
	instructions, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	var result int
	result = solvePartOne(instructions[:])
	fmt.Printf("Part one: %v\n", result)

	result = solvePartTwo(instructions)
	fmt.Printf("Part two: %v\n", result)
}
