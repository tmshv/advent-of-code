package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

type Board struct {
	Size   int
	Items  []int
	Marked map[int]bool
}

func (b *Board) Reset() {
	b.Marked = make(map[int]bool)
}

func (b *Board) Mark(num int) bool {
	b.Marked[num] = true

	// Check win by row
	for x := 0; x < b.Size; x++ {
		count := 0
		for y := 0; y < b.Size; y++ {
			i := b.CellIndex(x, y)
			n := b.Items[i]
			if _, ok := b.Marked[n]; ok {
				count++
			}
		}
		if count == b.Size {
			return true
		}
	}

	// Check win by column
	for y := 0; y < b.Size; y++ {
		count := 0
		for x := 0; x < b.Size; x++ {
			i := b.CellIndex(x, y)
			n := b.Items[i]
			if _, ok := b.Marked[n]; ok {
				count++
			}
		}
		if count == b.Size {
			return true
		}
	}

	return false
}

func (b *Board) SumUnmarked() int {
	sum := 0
	for x := 0; x < b.Size; x++ {
		for y := 0; y < b.Size; y++ {
			i := b.CellIndex(x, y)
			n := b.Items[i]
			if _, ok := b.Marked[n]; !ok {
				sum += n
			}
		}
	}
	return sum
}

// My ActionScript code from here: https://gist.github.com/tmshv/5013726
func (b *Board) CoordAt(index int) (int, int) {
	i := float64(index)
	length := float64(b.Size)
	row := math.Floor(i / length)
	col := i - row*length
	return int(row), int(col)
}

// Opposite calculation by ChatGPT
func (b *Board) CellIndex(row int, column int) int {
	return (row * b.Size) + column
}

func newBoard5(numbers []int) *Board {
	return &Board{5, numbers, make(map[int]bool)}
}

func readInput() ([]int, []*Board, error) {
	numbers := []int{}
	boards := []*Board{}

	scanner := bufio.NewScanner(os.Stdin)

	// Parse numbers
	scanner.Scan()
	line := scanner.Text()
	for _, x := range strings.Split(line, ",") {
		val, err := strconv.ParseInt(x, 10, 64)
		if err != nil {
			return nil, nil, err
		}
		numbers = append(numbers, int(val))
	}
	scanner.Scan() // Skip one line after numbers

	// Parse boards
	buffer := []int{}
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			if len(buffer) > 0 {
				boards = append(boards, newBoard5(buffer))
			}
			buffer = []int{}
			continue
		}
		line = strings.Trim(line, " ")
		for _, x := range strings.Split(line, " ") {
			if x == "" {
				continue
			}
			val, err := strconv.ParseInt(x, 10, 64)
			if err != nil {
				return nil, nil, err
			}
			buffer = append(buffer, int(val))
		}
	}
	if len(buffer) > 0 {
		boards = append(boards, newBoard5(buffer))
	}

	if err := scanner.Err(); err != nil {
		return nil, nil, err
	}

	return numbers, boards, nil
}

func solvePartOne(boards []*Board, numbers []int) int {
	for _, n := range numbers {
		for _, b := range boards {
			if b.Mark(n) {
				return n * b.SumUnmarked()
			}
		}
	}

	return 0
}

func solvePartTwo(boards []*Board, numbers []int) int {
	win := map[*Board]int{}
	for _, n := range numbers {
		for _, b := range boards {
			if b.Mark(n) {
				win[b] = n
				if len(win) == len(boards) { // End at this moment
					return n * b.SumUnmarked()
				}
			}
		}
	}

	return 0
}

func main() {
	numbers, boards, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	var result int
	result = solvePartOne(boards, numbers)
	fmt.Printf("Part one: %v\n", result)

	for _, b := range boards {
		b.Reset()
	}

	result = solvePartTwo(boards, numbers)
	fmt.Printf("Part two: %v\n", result)
}
