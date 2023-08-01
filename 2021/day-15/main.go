package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
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

func (p *Vector) Adjacents(ok func(p *Vector) bool) <-chan Vector {
	ch := make(chan Vector)

	go func() {
		defer close(ch)

		dirs := []Vector{
			{0, -1},
			{0, 1},
			{-1, 0},
			{1, 0},
		}

		for _, v := range dirs {
			v.Add(p)
			if ok(&v) {
				ch <- v
			}
		}
	}()

	return ch
}

type Grid struct {
	RowSize int
	Cells   []int
}

func (grid *Grid) Bounds() (Vector, Vector) {
	br := grid.CoordAt(len(grid.Cells) - 1) // Last cell
	return Vector{0, 0}, br
}

func (grid *Grid) Contains(cell *Vector) bool {
	tl, br := grid.Bounds()
	outside := cell.X < tl.X || cell.X > br.X || cell.Y < tl.Y || cell.Y > br.Y
	return !outside
}

// My ActionScript code from here: https://gist.github.com/tmshv/5013726
func (grid *Grid) CoordAt(index int) Vector {
	i := float64(index)
	size := float64(grid.RowSize)
	row := math.Floor(i / size)
	col := i - row*size
	return Vector{
		X: int(col),
		Y: int(row),
	}
}

// Opposite calculation by ChatGPT
func (grid *Grid) CellIndex(cell *Vector) int {
	return (cell.Y * grid.RowSize) + cell.X
}

func (grid *Grid) ValueAt(cell *Vector) int {
	j := grid.CellIndex(cell)
	return grid.Cells[j]
}

func readInput() (Grid, error) {
	cells := []int{}
	size := 0
	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		row := scanner.Text()
		size = len(row)
		for _, c := range row {
			val, err := strconv.Atoi(string(c))
			if err != nil {
				return Grid{}, err
			}
			cells = append(cells, val)
		}
	}

	if err := scanner.Err(); err != nil {
		return Grid{}, err
	}

	return Grid{RowSize: size, Cells: cells}, nil
}

type Stop struct {
	Position Vector
	Risk     int // The priority of the item.
	index    int // The index of the item in the heap.
}

// func (r *Stop) GetStop() *Stop2 {
// 	return &r.Position[len(r.Value)-1]
// }
//
// func (r *Stop) Next(stop Stop2) *Stop {
// 	value := make([]Stop2, len(r.Position))
// 	copy(value, r.Position)
// 	value = append(value, stop)
// 	return &Stop{
// 		Position: value,
// 		Risk:     r.Risk + stop.Risk,
// 		index:    r.index,
// 	}
// }

// PriorityQueue represents a priority queue.
type PriorityQueue []*Stop

// Len returns the length of the priority queue.
func (pq PriorityQueue) Len() int { return len(pq) }

// Less reports whether the element with index i should sort before the element with index j.
func (pq PriorityQueue) Less(i, j int) bool {
	// We want Pop to give us the lowest, not highest, priority so we use lower than here.
	return pq[i].Risk < pq[j].Risk
}

// Swap swaps the elements with indexes i and j.
func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

// Push pushes an element e with value and priority onto the priority queue.
func (pq *PriorityQueue) Push(e interface{}) {
	n := len(*pq)
	item := e.(*Stop)
	item.index = n
	*pq = append(*pq, item)
}

// Pop removes and returns the top element from the priority queue.
func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	item.index = -1 // for safety
	*pq = old[0 : n-1]
	return item
}

// func Reverse(vs []Vector) <- chan Vector {
//     ch := make(chan Vector)
//     for i := len(vs) - 1; i>=0; i -- {
//
//     }
//     return ch
// }

func FindShort(grid Grid, start, end Vector) ([]Vector, error) {
	pq := make(PriorityQueue, 0)
	heap.Push(&pq, &Stop{
		Position: start,
		Risk:     grid.ValueAt(&start),
		index: 0,
	})
	parents := map[Vector]Vector{}
	seen := map[Vector]bool{}
	seen[start] = true
	score := map[Vector]int{}
	score[start] = 0

	for pq.Len() > 0 {
		stop := heap.Pop(&pq).(*Stop)
		pos := stop.Position

		// Found! Build a route from end to start
		if pos == end {
			cur := end
			stops := []Vector{cur}
			for cur != start {
				p := parents[cur]
				stops = append([]Vector{p}, stops...)
				cur = p
			}
			return stops, nil
		}

		// Add all possible nodes to the queue
		adjacents := pos.Adjacents(func(p *Vector) bool {
			return grid.Contains(p)
		})
		for adj := range adjacents {
			// Ignore is P is visited
			if _, visited := seen[adj]; visited {
                continue
			}
			newRisk := score[pos] + grid.ValueAt(&adj)
			oldRisk := math.MaxInt32
			if s, ok := score[adj]; ok {
				oldRisk = s
			}
			if newRisk < oldRisk {
				next := Stop{
					Position: adj,
					Risk:     newRisk,
				}
				score[adj] = newRisk
				parents[adj] = pos
				seen[adj] = true
				pq.Push(&next)
			}
		}
	}

	return nil, nil
}

func solvePartOne(grid Grid) int {
	start, end := grid.Bounds()
	route, err := FindShort(grid, start, end)
	if err != nil {
		return -1
	}

	// Total risk from step 1
	total := 0
	for _, v := range route[1:] {
		risk := grid.ValueAt(&v)
		total += risk
	}

	tl, br := grid.Bounds()
	br.Sub(&tl)
	stops := map[Vector]int{}
	for _, v := range route {
	       risk := grid.ValueAt(&v)
		stops[v] = risk
	}
	for y := tl.Y; y <= br.Y; y++ {
		for x := tl.X; x <= br.X; x++ {
			v := Vector{x, y}
			char := "."
			if risk, ok := stops[v]; ok {
				char = fmt.Sprintf("%d", risk)
			}
			fmt.Print(char)
		}
		fmt.Println()
	}

	return total
}

func solvePartTwo(grid Grid) int {
	return 0
}

func main() {
	grid, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	var result int
	result = solvePartOne(grid)
	fmt.Printf("Part one: %v\n", result)

	result = solvePartTwo(grid)
	fmt.Printf("Part two: %v\n", result)
}
