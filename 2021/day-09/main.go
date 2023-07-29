package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"sort"
	"strconv"
)

type Point struct {
	X int
	Y int
}

func (p *Point) Adjacents(pred func(p *Point) bool) []Point {
	points := []Point{
		{X: p.X - 1, Y: p.Y},
		{X: p.X + 1, Y: p.Y},
		{X: p.X, Y: p.Y - 1},
		{X: p.X, Y: p.Y + 1},
	}
	result := []Point{}
	for _, p := range points {
		if pred(&p) {
			result = append(result, p)
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

func findLowPoints(grid *Grid) []Point {
	points := []Point{}
	for i, val := range grid.Cells {
		cell := grid.CoordAt(i)
		adjacents := cell.Adjacents(func(adj *Point) bool {
			return grid.Contains(adj)
		})
		f := len(adjacents)
		for _, adj := range adjacents {
			if grid.ValueAt(&adj) > val {
				f--
			}
		}
		if f == 0 {
			points = append(points, cell)
		}
	}
	return points
}

func solvePartOne(grid *Grid) int {
	count := 0
	for _, p := range findLowPoints(grid) {
		val := grid.ValueAt(&p)
		count += int(val) + 1
	}
	return count
}

func solvePartTwo(grid *Grid) int {
    basins := []int{}
	for _, low := range findLowPoints(grid) {
		queue := []Point{low}
		basin := map[Point]int8{}
		for len(queue) > 0 {
			cell := queue[0]
			queue = queue[1:]
			val := grid.ValueAt(&cell)
			basin[cell] = val
			adjacents := cell.Adjacents(func(adj *Point) bool {
				if !grid.Contains(adj) {
					return false
				}
				if grid.ValueAt(adj) == 9 {
					return false
				}
				if _, ok := basin[*adj]; ok {
					return false
				}
				return true
			})
			for _, adj := range adjacents {
				queue = append(queue, adj)
			}
		}
        basins = append(basins, len(basin))
	}
    sort.Sort(sort.Reverse(sort.IntSlice(basins)))
	return basins[0] * basins[1] * basins[2]
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
