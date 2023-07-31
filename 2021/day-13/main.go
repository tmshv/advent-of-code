package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type Vector struct {
	X int
	Y int
}

func (p *Vector) Add(other *Vector) {
	p.X += other.X
	p.Y += other.Y
}

func (p *Vector) Sub(other *Vector) {
	p.X -= other.X
	p.Y -= other.Y
}

func (p *Vector) Multiply(value int) {
	p.X *= value
	p.Y *= value
}

func (p *Vector) MultiplyScalar(other *Vector) {
	p.X *= other.X
	p.Y *= other.Y
}

func (p *Vector) Scalar(other *Vector) int {
	return p.X*other.X + p.Y*other.Y
}

func (p *Vector) Sign() Vector {
	var x int
	if p.X > 0 {
		x = 1
	} else if p.X < 0 {
		x = -1
	} else {
		x = 0
	}

	var y int
	if p.Y > 0 {
		y = 1
	} else if p.Y < 0 {
		y = -1
	} else {
		y = 0
	}

	return Vector{x, y}
}

func Sub(a, b *Vector) Vector {
	return Vector{
		X: a.X - b.X,
		Y: a.Y - b.Y,
	}
}

func readInput() ([]Vector, []Vector, error) {
	scanner := bufio.NewScanner(os.Stdin)

	// Points
	points := []Vector{}
	for scanner.Scan() {
		row := scanner.Text()
		if row == "" {
			break
		}

		parts := strings.Split(row, ",")
		if len(parts) != 2 {
			return nil, nil, errors.New("Bad point data")
		}

		x, err := strconv.Atoi(parts[0])
		if err != nil {
			return nil, nil, err
		}
		y, err := strconv.Atoi(parts[1])
		if err != nil {
			return nil, nil, err
		}
		points = append(points, Vector{x, y})
	}

	// Folds
	folds := []Vector{}
	for scanner.Scan() {
		row := scanner.Text()
		regex := regexp.MustCompile(`(x|y)=(\d+)`)
		match := regex.FindStringSubmatch(row)
		// Ensure that a match is found
		if len(match) != 3 {
			return nil, nil, errors.New("Bad fold data")
		}

		// Extract the axis ('x' or 'y') and the value
		axis := match[1]
		value, err := strconv.Atoi(match[2])
		if err != nil {
			return nil, nil, err
		}

		var fold Vector
		if axis == "x" {
			fold = Vector{value, 0}
		}
		if axis == "y" {
			fold = Vector{0, value}
		}
		folds = append(folds, fold)
	}
	if err := scanner.Err(); err != nil {
		return nil, nil, err
	}

	return points, folds, nil
}

func Unique(points []Vector) []Vector {
	seen := map[Vector]bool{}
	result := []Vector{}
	for _, point := range points {
		if _, ok := seen[point]; !ok {
			result = append(result, point)
			seen[point] = true
		}
	}
	return result
}

func GetBounds(points []Vector) (Vector, Vector) {
	minPoint := Vector{10000000, 1000000}
	maxPoint := Vector{0, 0}
	for _, point := range points {
		if point.X < minPoint.X {
			minPoint.X = point.X
		}
		if point.Y < minPoint.Y {
			minPoint.Y = point.Y
		}
		if point.X > maxPoint.X {
			maxPoint.X = point.X
		}
		if point.Y > maxPoint.Y {
			maxPoint.Y = point.Y
		}
	}
	return minPoint, maxPoint
}

func Fold(points, folds []Vector) []Vector {
	down := Vector{0, 1}

	for _, fold := range folds {
		sign := fold.Sign()
		horizontal := sign.Scalar(&down) == 0
		for i := range points {
			point := &points[i]

			doFolding := false
			if horizontal && point.X > fold.X {
				doFolding = true
			}
			if !horizontal && point.Y > fold.Y {
				doFolding = true
			}
			// Point is on the foldable part
			if doFolding {
				// Calculate
				// 1. find vector to fold from point
				// 2. keep only foldable direction
				// 3. make it twice bigger (one to reach axis and one more to mirrored location)
				move := Sub(&fold, point)
				move.MultiplyScalar(&sign)
				move.Multiply(2)
				point.Add(&move)
			}
		}
	}

	return Unique(points)
}

func solvePartOne(points, folds []Vector) int {
	folded := Fold(points, folds[:1])
	return len(folded)
}

func solvePartTwo(points, folds []Vector) int {
	folded := Fold(points, folds)
	ps := map[Vector]bool{}
	for _, f := range folded {
		ps[f] = true
	}

	minPoint, maxPoint := GetBounds(folded)
	maxPoint.Sub(&minPoint)

	for y := 0; y <= maxPoint.Y; y++ {
		for x := 0; x <= maxPoint.X; x++ {
			p := Vector{x, y}
			char := "."
			if _, ok := ps[p]; ok {
				char = "#"
			}
			fmt.Print(char)
		}
		fmt.Println()
	}

	return -1
}

func main() {
	points, folds, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	var result int
	result = solvePartOne(points, folds)
	fmt.Printf("Part one: %v\n", result)

	result = solvePartTwo(points, folds)
	fmt.Printf("Part two: %v\n", result)
}
