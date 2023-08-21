package main

import (
	"bufio"
	"math"
	"fmt"
	"log"
	"os"
	"regexp"
)

var (
	gravity Vector = Vector{0, -1}
	left    Vector = Vector{-1, 0}
	right   Vector = Vector{1, 0}
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

type Bounds struct {
	Min Vector
	Max Vector
}

func (b *Bounds) Contains(v *Vector) bool {
	outside := v.X < b.Min.X || v.X > b.Max.X || v.Y < b.Min.Y || v.Y > b.Max.Y
	return !outside
}

type Probe struct {
	Position Vector
	Velocity Vector
}

func (p *Probe) Step() {
	p.Position.Add(&p.Velocity)

	p.Velocity.Add(&gravity)
	if p.Velocity.X < 0 {
		p.Velocity.X++
	} else if p.Velocity.X > 0 {
		p.Velocity.X--
	}
}

func readInput() (Bounds, error) {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()
	if err := scanner.Err(); err != nil {
		return Bounds{}, err
	}
	row := scanner.Text()

	re := regexp.MustCompile(`target area: x=([\d-]+)..([\d-]+), y=([\d-]+)..([\d-]+)`)
	matches := re.FindStringSubmatch(row)

	var xMin, xMax, yMin, yMax int
	fmt.Sscanf(matches[1], "%d", &xMin)
	fmt.Sscanf(matches[2], "%d", &xMax)
	fmt.Sscanf(matches[3], "%d", &yMin)
	fmt.Sscanf(matches[4], "%d", &yMax)

	b := Bounds{
		Min: Vector{xMin, yMin},
		Max: Vector{xMax, yMax},
	}
	return b, nil
}

func MaxY(vs []Vector) int {
	result := math.MinInt32
	for _, v := range vs {
		if v.Y > result {
			result = v.Y
		}
	}
	return result
}

func DropProbe(vel Vector, zone *Bounds) int {
	probe := Probe{
		Position: Vector{0, 0},
		Velocity: vel,
	}
	track := []Vector{probe.Position}
	for {
		probe.Step()
		track = append(track, probe.Position)
		if zone.Contains(&probe.Position) {
			return MaxY(track)
		}
        if probe.Position.Y < zone.Min.Y {
            return -1
        }
	}
}

func DropProbe2(vel Vector, zone *Bounds) bool {
	probe := Probe{
		Position: Vector{0, 0},
		Velocity: vel,
	}
	for {
		probe.Step()
		if zone.Contains(&probe.Position) {
			return true
		}
        if probe.Position.Y < zone.Min.Y {
            return false
        }
	}
}

func solvePartOne(zone Bounds) int {
	best := math.MinInt32
	for x := 0; x < 100; x++ {
		for y := 0; y < 1000; y++ {
			vel := Vector{x, y}
			y := DropProbe(vel, &zone)
			if y == -1 {
				continue
			}
			if y > best {
				best = y
			}
		}
	}
	return best
}

func solvePartTwo(zone Bounds) int {
	count := 0
	for x := 0; x < 1000; x++ {
		for y := -2000; y < 2000; y++ {
			vel := Vector{x, y}
			if DropProbe2(vel, &zone) {
				count++
			}
		}
	}
	return count
}

func main() {
	zone, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	var result int
	result = solvePartOne(zone)
	fmt.Printf("Part one: %v\n", result)

	result = solvePartTwo(zone)
	fmt.Printf("Part two: %v\n", result)
}
