package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
)

type Point struct {
	X int
	Y int
}

var ADJ = [8]Point{
	{-1, -1},
	{0, -1},
	{1, -1},
	{-1, 0},
	{1, 0},
	{-1, 1},
	{0, 1},
	{1, 1},
}

func (p *Point) Add(other *Point) {
	p.X += other.X
	p.Y += other.Y
}

func (p *Point) Adjacents(pred func(p *Point) bool) []Point {
	result := []Point{}
	for _, a := range ADJ {
		a.Add(p)
		if pred(&a) {
			result = append(result, a)
		}
	}
	return result
}

type Grid struct {
	RowSize int
	Cells   []int8
}

func (grid *Grid) Bbox() (int, int, int, int) {
	br := grid.CoordAt(len(grid.Cells) - 1) // Last cell
	l := 0
	r := br.X
	t := 0
	b := br.Y
	return l, r, t, b
}

func (grid *Grid) Contains(cell *Point) bool {
	l, r, t, b := grid.Bbox()
	outside := cell.X < l || cell.X > r || cell.Y < t || cell.Y > b
	return !outside
}

// My ActionScript code from here: https://gist.github.com/tmshv/5013726
func (grid *Grid) CoordAt(index int) Point {
	i := float64(index)
	size := float64(grid.RowSize)
	row := math.Floor(i / size)
	col := i - row*size
	return Point{
		X: int(col),
		Y: int(row),
	}
}

// Opposite calculation by ChatGPT
func (grid *Grid) CellIndex(cell *Point) int {
	return (cell.Y * grid.RowSize) + cell.X
}

func (grid *Grid) ValueAt(cell *Point) int8 {
	j := grid.CellIndex(cell)
	return int8(grid.Cells[j])
}

func (grid *Grid) IncrementAt(cell *Point) int8 {
	i := grid.CellIndex(cell)
	grid.Cells[i] += 1
	return grid.Cells[i]
}

func (grid *Grid) Print() {
	for y := 0; y < grid.RowSize; y++ {
		for x := 0; x < grid.RowSize; x++ {
			val := grid.ValueAt(&Point{x, y})
			if val == 0 {
				fmt.Print("0")
			} else {
				fmt.Print(val)
			}
		}
		fmt.Println()
	}
}

func (grid *Grid) Step() int {
	// First, the energy level of each octopus increases by 1.
	for i := range grid.Cells {
		grid.Cells[i] += 1
	}

	// Then, any octopus with an energy level greater than 9 flashes.
	// This increases the energy level of all adjacent octopuses by 1,
	// including octopuses that are diagonally adjacent.
	// If this causes an octopus to have an energy level greater than 9,
	// it also flashes.
	// This process continues as long
	// as new octopuses keep having their energy level increased beyond 9.
	// (An octopus can only flash at most once per step.)
	flashed := map[Point]bool{}
	queue := []Point{}
	for i, energy := range grid.Cells {
		if energy > 9 {
			p := grid.CoordAt(i)
			queue = append(queue, p)
		}
	}
	for len(queue) > 0 {
		f := queue[0]
		queue = queue[1:]
		if _, ok := flashed[f]; ok {
			continue
		}
		flashed[f] = true
		adjacents := f.Adjacents(func(adj *Point) bool {
			return grid.Contains(adj)
		})
		for _, adj := range adjacents {
			if grid.IncrementAt(&adj) > 9 {
				queue = append(queue, adj)
			}
		}
	}

	// Finally,
	// any octopus that flashed during this step
	// has its energy level set to 0,
	// as it used all of its energy to flash.
	for pos := range flashed {
		i := grid.CellIndex(&pos)
		grid.Cells[i] = 0
	}

	return len(flashed)
}

func readInput() (Grid, error) {
	cells := []int8{}
	s := 0
	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		row := scanner.Text()
		s = len(row)
		for _, c := range row {
			val, err := strconv.Atoi(string(c))
			if err != nil {
				return Grid{}, err
			}
			cells = append(cells, int8(val))
		}
	}

	if err := scanner.Err(); err != nil {
		return Grid{}, err
	}

	return Grid{RowSize: s, Cells: cells}, nil
}

func solvePartOne(grid *Grid) int {
	count := 0
	for i := 0; i < 100; i++ {
		count += grid.Step()
	}
	return count
}

func solvePartTwo(grid *Grid) int {
	return 0
}

func main() {
	grid, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	var result int
	result = solvePartOne(&grid)
	fmt.Printf("Part one: %v\n", result)

	result = solvePartTwo(&grid)
	fmt.Printf("Part two: %v\n", result)
}
