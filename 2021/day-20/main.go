package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
)

type Vector struct {
	X int
	Y int
}

func (p *Vector) ToFloat64() (float64, float64) {
	return float64(p.X), float64(p.Y)
}

func (p *Vector) Set(x, y int) {
	p.X = x
	p.Y = y
}

func (p *Vector) Add(other *Vector) {
	p.X += other.X
	p.Y += other.Y
}

func (p *Vector) Sub(other *Vector) {
	p.X -= other.X
	p.Y -= other.Y
}

func (p *Vector) Adjacents() []*Vector {
	vectors := []*Vector{
		{-1, -1},
		{0, -1},
		{1, -1},

		{-1, 0},
		{0, 0},
		{1, 0},

		{-1, 1},
		{0, 1},
		{1, 1},
	}

	for _, v := range vectors {
		v.Add(p)
	}

	return vectors
}

func readInput() ([]bool, *Image, error) {
	scanner := bufio.NewScanner(os.Stdin)

	scanner.Scan()
	row := scanner.Text()
	enhance := make([]bool, len(row))
	for i, r := range row {
		enhance[i] = r == '#'
	}

	// Skip empty line
	scanner.Scan()

	canvas := map[Vector]bool{}
	y := 0
	for scanner.Scan() {
		for x, r := range scanner.Text() {
			light := Vector{x, y}
			canvas[light] = r == '#'
		}
		y++
	}

	if err := scanner.Err(); err != nil {
		return nil, nil, err
	}

	return enhance, &Image{canvas}, nil
}

type Image struct {
	Canvas map[Vector]bool
}

func (image *Image) CountLit() int {
	result := 0
	for pixel := range image.Canvas {
		if image.Canvas[pixel] {
			result++
		}
	}

	return result
}

func (image *Image) Bounds() (int, int, int, int) {
	var xmin, xmax, ymin, ymax int
	xmin = math.MaxInt32
	ymin = math.MaxInt32
	for k := range image.Canvas {
		if k.X < xmin {
			xmin = k.X
		}
		if k.X > xmax {
			xmax = k.X
		}
		if k.Y < ymin {
			ymin = k.Y
		}
		if k.Y > ymax {
			ymax = k.Y
		}
	}
	return xmin, xmax, ymin, ymax
}

func (image *Image) Extend(val int, color bool) {
	xmin, xmax, ymin, ymax := image.Bounds()
	for x := xmin - val; x <= xmax+val; x++ {
		for y := ymin - val; y <= ymax+val; y++ {
			p := Vector{x, y}
			if _, ok := image.Canvas[p]; !ok {
				image.Canvas[p] = color
			}
		}
	}
}

func (image *Image) Enhance(offset *Vector, enhance []bool) {
	xmin, xmax, ymin, ymax := image.Bounds()
	xmin += offset.X
	xmax -= offset.X
	ymin += offset.Y
	ymax -= offset.Y
	next := map[Vector]bool{}
	pixel := &Vector{0, 0}
	for y := ymin; y <= ymax; y++ {
		for x := xmin; x <= xmax; x++ {
			pixel.Set(x, y)
			index := image.GetEnhanceAt(pixel)
			next[*pixel] = enhance[index]
		}
	}
	image.Canvas = next
}

func (image *Image) GetEnhanceAt(pixel *Vector) int {
	index := 0
	shift := 8
	for _, v := range pixel.Adjacents() {
		bit := 0
		if c, ok := image.Canvas[*v]; ok && c {
			bit = 1
		}
		index |= bit << shift
		shift--
	}
	return index
}

func (image *Image) Print() {
	xmin, xmax, ymin, ymax := image.Bounds()
	for y := ymin; y <= ymax; y++ {
		for x := xmin; x <= xmax; x++ {
			p := Vector{x, y}
			if image.Canvas[p] {
				fmt.Print("#")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}
	fmt.Println()
}

func solve(steps int, enhance []bool, image *Image) int {
	// determine what the infinite space will do,
	// if zero index is "." then it will stay "."
	infiniteSpaceStaysBlack := !enhance[0]

	for i := 0; i < steps; i++ {
		infiniteWhite := i%2 == 1
		infiniteColor := false
		if !infiniteSpaceStaysBlack && infiniteWhite {
			infiniteColor = true
		}

		image.Extend(3, infiniteColor)
		image.Enhance(&Vector{2, 2}, enhance)
	}

	return image.CountLit()
}

func solvePartOne(enhance []bool, image *Image) int {
  return solve(2, enhance, image)
}

func solvePartTwo(enhance []bool, image *Image) int {
  return solve(48, enhance, image) // two steps from part one
}

func main() {
	enhance, image, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	var result int
	result = solvePartOne(enhance, image)
	fmt.Printf("Part one: %v\n", result)

	result = solvePartTwo(enhance, image)
	fmt.Printf("Part two: %v\n", result)
}
