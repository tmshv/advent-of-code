package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"

	"github.com/ernestosuarez/itertools"
)

type Vector struct {
	x float64
	y float64
}

func (v *Vector) Add(other *Vector) Vector {
	return Vector{v.x + other.x, v.y + other.y}
}

func (v *Vector) Sign() {
	if v.x < 0 {
		v.x = -1
	} else if v.x > 0 {
		v.x = 1
	} else {
		v.x = 0
	}
	if v.y < 0 {
		v.y = -1
	} else if v.y > 0 {
		v.y = 1
	} else {
		v.y = 0
	}
}

type Line struct {
	x1 float64
	y1 float64
	x2 float64
	y2 float64
}

func (line *Line) GetDirection() Vector {
	x := line.x2 - line.x1
	y := line.y2 - line.y1
	v := Vector{x, y}
	v.Sign()
	return v
}

func (line *Line) Trace() []Vector {
	dir := line.GetDirection()
	cursor := Vector{line.x1, line.y1}
	e := Vector{line.x2, line.y2}
	vs := []Vector{}
	for {
		vs = append(vs, cursor)
		cursor = cursor.Add(&dir)
		if cursor == e {
			vs = append(vs, cursor)
			break
		}
	}
	return vs
}

func (line *Line) IsOrthogonal() bool {
	return line.x1 == line.x2 || line.y1 == line.y2
}

func extractNumbers(input string) (int, int, int, int) {
	// Define the regular expression pattern
	pattern := `(\d+),(\d+)\s->\s(\d+),(\d+)`

	// Compile the regular expression
	regex := regexp.MustCompile(pattern)

	// Find all matches in the input string
	matches := regex.FindStringSubmatch(input)

	// Extract the numbers from the submatches
	num1 := matches[1] // First number before comma
	num2 := matches[2] // Second number after comma
	num3 := matches[3] // Third number before comma
	num4 := matches[4] // Fourth number after comma

	// Convert the numbers from strings to integers
	var n1, n2, n3, n4 int
	fmt.Sscanf(num1, "%d", &n1)
	fmt.Sscanf(num2, "%d", &n2)
	fmt.Sscanf(num3, "%d", &n3)
	fmt.Sscanf(num4, "%d", &n4)

	// Return the extracted numbers
	return n1, n2, n3, n4
}

func readInput() ([]*Line, error) {
	lines := []*Line{}
	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		str := scanner.Text()
		x1, y1, x2, y2 := extractNumbers(str)
		line := Line{float64(x1), float64(y1), float64(x2), float64(y2)}
		lines = append(lines, &line)
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return lines, nil
}

func intersect(a, b []Vector) []Vector {
	result := []Vector{}
	for _, x := range a {
		for _, y := range b {
			if x == y {
				result = append(result, x)
			}
		}
	}
	return result
}

func solve(lines [][]Vector) int {
	crosses := map[Vector]int{}
	for pair := range itertools.GenCombinations(len(lines), 2) {
		a := lines[pair[0]]
		b := lines[pair[1]]

		is := intersect(a, b)
		for _, i := range is {
			crosses[i] += 1
		}
	}

	return len(crosses)
}
func solvePartOne(lines []*Line) int {
	orthogonal := [][]Vector{}
	for _, line := range lines {
		if line.IsOrthogonal() {
			orthogonal = append(orthogonal, line.Trace())
		}
	}

	return solve(orthogonal)
}

func solvePartTwo(lines []*Line) int {
	orthogonal := make([][]Vector, len(lines))
	for i, line := range lines {
		orthogonal[i] = line.Trace()
	}
	return solve(orthogonal)
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
