package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"

	"github.com/tmshv/advent-of-code/2021/day-22/boxtree"
)

type instruction struct {
	min boxtree.Vector
	max boxtree.Vector
	on  bool
}

func readInput() ([]instruction, error) {
	instructions := []instruction{}
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

		i := instruction{
			min: *boxtree.NewFromInt(xStart, yStart, zStart),
			max: *boxtree.NewFromInt(xEnd, yEnd, zEnd),
			on:  on,
		}
		instructions = append(instructions, i)
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return instructions, nil
}

type pair [2]int

func pairs(n int) <-chan pair {
	ch := make(chan pair)
	go func() {
		defer close(ch)
		for i := 0; i < n; i++ {
			for j := i + 1; j < n; j++ {
				p := pair{i, j}
				ch <- p
			}
		}
	}()
	return ch
}

func union(boxes []*boxtree.Box3) []*boxtree.Box3 {
	// Nothing to union
	if len(boxes) < 2 {
		return boxes[:]
	}

	result := boxes[:1]
	q := boxes[1:]
	for len(q) > 0 {
		box := q[0]
		q = q[1:]

		intesect := -1
		for i, b := range result {
			if box.IntersectsBox(b) {
				intesect = i
				break
			}
		}
		if intesect >= 0 {
			parts := box.Subtract(result[intesect])
			result = append(result, parts...)
		}
	}

	return result
}

func add(box *boxtree.Box3, boxes []*boxtree.Box3) []*boxtree.Box3 {
	var result []*boxtree.Box3

	var inter bool
	for _, b := range boxes {
		if b.IntersectsBox(box) {
			parts := box.Subtract(b)
			result = append(result, parts...)
			inter = true
		}
	}
	if !inter {
		result = append(result, box)
	}

	return result
}

func sub(box *boxtree.Box3, boxes []*boxtree.Box3) []*boxtree.Box3 {
	var result []*boxtree.Box3

	for _, b := range boxes {
		if b.IntersectsBox(box) {
			parts := b.Subtract(box)
			result = append(result, parts...)
		} else {
			result = append(result, b)
		}
	}

	return result
}

func volume(b *boxtree.Box3) float64 {
	return float64((b.Max.X - b.Min.X) * (b.Max.Y - b.Min.Y) * (b.Max.Z - b.Min.Z))
}

func volumes(bs []*boxtree.Box3) float64 {
	var vol float64
	for _, b := range bs {
		vol += volume(b)
	}
	return vol
}

func solvePartOne(ix []instruction) int {
	bounds := boxtree.Box3{
		Min: boxtree.Vector{-50, -50, -50},
		Max: boxtree.Vector{50, 50, 50},
	}

	boxes := []*boxtree.Box3{}
	exp := boxtree.Vector{X: 1, Y: 1, Z: 1}
	for _, i := range ix {
		box := boxtree.NewFromMinMax(i.min, i.max)
		if !box.IntersectsBox(&bounds) {
			fmt.Println("ignore", box)
			continue
		}

		box.Max.Add(&exp)

		if i.on {
			parts := add(box, boxes)
			boxes = append(boxes, parts...)
			fmt.Println(len(boxes))
			// fmt.Println("on box", *box, "parts", len(parts), "+", volumes(parts))
			// for _, b := range parts {
			// 	fmt.Println("...part", *b, volume(b))
			// }
		} else {
			boxes = sub(box, boxes)
			// boxes = append(boxes, parts...)
			// fmt.Println("of box", *box, "-", volumes(boxes))
			// for _, b := range parts {
			// 	fmt.Println("...-part", *b, volume(b))
			// }
		}
	}

	count := 0
	for _, box := range boxes {
		// cube := *box
		// cube.Expand(0.5)
		// cube.Max.Add(&exp)
		// fmt.Println("vol", *box, volume(box))
		count += int(volume(box))
	}
	return count
}

func solvePartTwo(ix []instruction) int {
	return 0
}

func main() {
	instructions, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	var result int
	result = solvePartOne(instructions)
	fmt.Printf("Part one: %v\n", result)

	result = solvePartTwo(instructions)
	fmt.Printf("Part two: %v\n", result)
}
