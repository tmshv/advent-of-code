package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"

	"gonum.org/v1/gonum/mat"
)

type Sensor struct {
	I        int
	Position *Vector
	Matrix   *mat.Dense
	Beacons  []Vector
}

func (s *Sensor) Apply() Sensor {
	var inv mat.Dense
	err := inv.Inverse(s.Matrix)
	if err != nil {
		log.Fatalf("A is not invertible: %v", err)
	}
	beacons := make([]Vector, len(s.Beacons))
	for i, b := range s.Beacons {
		n := b.ApplyMatrix3D(&inv)
		n = n.Add(s.Position)
		beacons[i] = *n
	}
	return Sensor{
		I:        s.I,
		Position: s.Position,
		Matrix:   NewIdentityDense(),
		Beacons:  beacons,
	}
}

func (s *Sensor) Orient(beacons []Vector) bool {
	for _, matrix := range GetSensorVariants() {
		var inv mat.Dense
		err := inv.Inverse(matrix)
		if err != nil {
			log.Fatalf("A is not invertible: %v", err)
		}
		subtractCounts := map[Vector]int{}
		for _, beacon := range beacons {
			for _, b := range s.Beacons {
				t := b.ApplyMatrix3D(&inv)
				sub := beacon.Sub(t)
				subtractCounts[*sub]++
			}
		}
		for pos, count := range subtractCounts {
			if count >= 12 {
				s.Position = &pos
				s.Matrix = matrix
				return true
			}
		}
	}
	return false
}

func NewIdentityDense() *mat.Dense {
	return mat.NewDense(3, 3, []float64{
		1, 0, 0,
		0, 1, 0,
		0, 0, 1,
	})
}

func MatrixFromVectors(a, b, c *Vector) *mat.Dense {
	return mat.NewDense(3, 3, []float64{
		a.X, a.Y, a.Z,
		b.X, b.Y, b.Z,
		c.X, c.Y, c.Z,
	})
}

func GetFourUpOrientationVariants(face, a, b *Vector) []*mat.Dense {
	return []*mat.Dense{
		MatrixFromVectors(face, a, b),
		MatrixFromVectors(face, a, b.Mult(-1)),
		MatrixFromVectors(face, a.Mult(-1), b),
		MatrixFromVectors(face, a.Mult(-1), b.Mult(-1)),
	}
}

type Vector struct {
	X float64
	Y float64
	Z float64
}

func (v *Vector) Mult(val float64) *Vector {
	return &Vector{
		v.X * val,
		v.Y * val,
		v.Z * val,
	}
}

func (v *Vector) Add(val *Vector) *Vector {
	return &Vector{
		v.X + val.X,
		v.Y + val.Y,
		v.Z + val.Z,
	}
}

func (v *Vector) Sub(val *Vector) *Vector {
	return &Vector{
		v.X - val.X,
		v.Y - val.Y,
		v.Z - val.Z,
	}
}

func (v *Vector) Abs() *Vector {
	return &Vector{
    math.Abs(v.X),
    math.Abs(v.Y),
    math.Abs(v.Z),
	}
}

func (v *Vector) ApplyMatrix3D(m *mat.Dense) *Vector {
	x11 := m.At(0, 0)
	x12 := m.At(1, 0)
	x13 := m.At(2, 0)

	x21 := m.At(0, 1)
	x22 := m.At(1, 1)
	x23 := m.At(2, 1)

	x31 := m.At(0, 2)
	x32 := m.At(1, 2)
	x33 := m.At(2, 2)

	return &Vector{
		x11*v.X + x21*v.Y + x31*v.Z,
		x12*v.X + x22*v.Y + x32*v.Z,
		x13*v.X + x23*v.Y + x33*v.Z,
	}
}

func (v *Vector) ManhattanDistance(val *Vector) float64 {
  s := v.Sub(val).Abs()
  return s.X + s.Y + s.Z
}

func NewVector3(val []float64) *Vector {
	return &Vector{
		val[0],
		val[1],
		val[2],
	}
}

func readInput() ([]Sensor, error) {
	sensors := []Sensor{}
	scanner := bufio.NewScanner(os.Stdin)

	var buffer []Vector
	i := 0
	for {
		// Skip row --- scanner X ---
		// Stop at EOL
		if !scanner.Scan() {
			break
		}

		buffer = []Vector{}
		for scanner.Scan() {
			row := scanner.Text()
			if row == "" {
				break
			}
			parts := strings.Split(row, ",")
			if len(parts) != 3 {
				return nil, errors.New("Bad input")
			}
			numbers := make([]float64, len(parts))
			for i, n := range parts {
				val, err := strconv.Atoi(n)
				if err != nil {
					return nil, err
				}
				numbers[i] = float64(val)
			}
			buffer = append(buffer, *NewVector3(numbers))
		}

		s := Sensor{
			I:        i,
			Position: nil,
			Matrix:   nil,
			Beacons:  buffer,
		}
		sensors = append(sensors, s)
		i++
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return sensors, nil
}

func GetSensorVariants() []*mat.Dense {
	var vs []*mat.Dense
	variants := []*mat.Dense{}

	vs = GetFourUpOrientationVariants(
		&Vector{1, 0, 0},
		&Vector{0, 1, 0},
		&Vector{0, 0, 1},
	)
	for _, x := range vs {
		variants = append(variants, x)
	}

	//
	vs = GetFourUpOrientationVariants(
		&Vector{1, 0, 0},
		&Vector{0, 0, 1},
		&Vector{0, 1, 0},
	)
	for _, x := range vs {
		variants = append(variants, x)
	}

	vs = GetFourUpOrientationVariants(
		&Vector{-1, 0, 0},
		&Vector{0, 1, 0},
		&Vector{0, 0, 1},
	)
	for _, x := range vs {
		variants = append(variants, x)
	}

	//
	vs = GetFourUpOrientationVariants(
		&Vector{-1, 0, 0},
		&Vector{0, 0, 1},
		&Vector{0, 1, 0},
	)
	for _, x := range vs {
		variants = append(variants, x)
	}

	vs = GetFourUpOrientationVariants(
		&Vector{0, 1, 0},
		&Vector{1, 0, 0},
		&Vector{0, 0, 1},
	)
	for _, x := range vs {
		variants = append(variants, x)
	}

	//
	vs = GetFourUpOrientationVariants(
		&Vector{0, 1, 0},
		&Vector{0, 0, 1},
		&Vector{1, 0, 0},
	)
	for _, x := range vs {
		variants = append(variants, x)
	}

	vs = GetFourUpOrientationVariants(
		&Vector{0, -1, 0},
		&Vector{1, 0, 0},
		&Vector{0, 0, 1},
	)
	for _, x := range vs {
		variants = append(variants, x)
	}

	//
	vs = GetFourUpOrientationVariants(
		&Vector{0, -1, 0},
		&Vector{0, 0, 1},
		&Vector{1, 0, 0},
	)
	for _, x := range vs {
		variants = append(variants, x)
	}

	vs = GetFourUpOrientationVariants(
		&Vector{0, 0, 1},
		&Vector{1, 0, 0},
		&Vector{0, 1, 0},
	)
	for _, x := range vs {
		variants = append(variants, x)
	}

	//
	vs = GetFourUpOrientationVariants(
		&Vector{0, 0, 1},
		&Vector{0, 1, 0},
		&Vector{1, 0, 0},
	)
	for _, x := range vs {
		variants = append(variants, x)
	}

	vs = GetFourUpOrientationVariants(
		&Vector{0, 0, -1},
		&Vector{1, 0, 0},
		&Vector{0, 1, 0},
	)
	for _, x := range vs {
		variants = append(variants, x)
	}

	//
	vs = GetFourUpOrientationVariants(
		&Vector{0, 0, -1},
		&Vector{0, 1, 0},
		&Vector{1, 0, 0},
	)
	for _, x := range vs {
		variants = append(variants, x)
	}

	return variants
}

func solve(sensors []Sensor) []Sensor {
	// Algorhitm
	// Each scanner have a transfromation matrix 4x4
	// Scanner 0 have identity 4x4 matrix
	// Other scanners have no known matrix
	// Try each 24 variant of possible tranformation
	// on next unchecked scanner
	// Find inverse of tranformation matrix
	// and apply this inverted matrix on scanner's beacons
	// Check transformed beacons on one of known scanner
	// by subtracting beacons A from beacons B
	// Count this results: common beacons should have same result
	// If count value >= 12
	// This two scanners have overlaps
	// So apply transformation to this Sensor
	// Its Position is a Vectors counted 12 times
	// Apply transformation to scanner to orient it
	// according to 0 scanner

	// 0 Sensor is identity
	sensors[0].Position = &Vector{}
	sensors[0].Matrix = NewIdentityDense()

	oriented := []Sensor{
		sensors[0],
	}
	remaining := sensors[1:]
	for len(remaining) > 0 {
		s := remaining[0]
		remaining = remaining[1:]
		ok := false
		for _, f := range oriented {
			if s.Orient(f.Beacons) {
				oriented = append(oriented, s.Apply())
				ok = true
				break
			}
		}
		if !ok {
			remaining = append(remaining, s)
		}
	}

	return oriented
}

func solvePartOne(sensors []Sensor) int {
	beaconSet := map[Vector]bool{}
	for _, s := range sensors {
		for _, b := range s.Beacons {
			beaconSet[b] = true
		}
	}

	return len(beaconSet)
}

func solvePartTwo(sensors []Sensor) int {
  var max float64 = 0
  for i, a := range sensors {
    for j, b := range sensors {
      if i == j {
        continue
      }
      dist := a.Position.ManhattanDistance(b.Position)
      if dist > max {
        max = dist
      }
    }
  }
	return int(max)
}

func main() {
	sensors, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	sensors = solve(sensors)

	var result int
	result = solvePartOne(sensors)
	fmt.Printf("Part one: %v\n", result)

	result = solvePartTwo(sensors)
	fmt.Printf("Part two: %v\n", result)
}
