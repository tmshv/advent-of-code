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

type Scanner struct {
	Beacons        []Vector
	Position       *Vector
	Transformation *mat.Dense
}

func (s *Scanner) ApplyTransform() Scanner {
	var inv mat.Dense
	err := inv.Inverse(s.Transformation)
	if err != nil {
		log.Fatalf("A is not invertible: %v", err)
	}
	beacons := make([]Vector, len(s.Beacons))
	for i, b := range s.Beacons {
		n := b.ApplyMatrix3D(&inv)
		n = n.Add(s.Position)
		beacons[i] = *n
	}
	return Scanner{
		Position:       s.Position,
		Transformation: NewIdentityDense(),
		Beacons:        beacons,
	}
}

func (s *Scanner) Orient(beacons []Vector) bool {
	for _, matrix := range GetTransformVariations() {
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
				s.Transformation = matrix
				return true
			}
		}
	}
	return false
}

type Vector struct {
	X float64
	Y float64
	Z float64
}

func (v *Vector) ToArray() []float64 {
	return []float64{v.X, v.Y, v.Z}
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

func NewIdentityDense() *mat.Dense {
	return mat.NewDense(3, 3, []float64{
		1, 0, 0,
		0, 1, 0,
		0, 0, 1,
	})
}

func readInput() ([]Scanner, error) {
	result := []Scanner{}
	scanner := bufio.NewScanner(os.Stdin)

	var buffer []Vector
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

		result = append(result, Scanner{buffer, nil, nil})
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return result, nil
}

func GetEightOrientationVariants(face, up, side *Vector) []*mat.Dense {
	a := face.ToArray()
	b := up.ToArray()
	c := side.ToArray()
	return []*mat.Dense{
		mat.NewDense(3, 3, []float64{
			a[0], b[0], c[0],
			a[1], b[1], c[1],
			a[2], b[2], c[2],
		}),
		mat.NewDense(3, 3, []float64{
			a[0], -b[0], c[0],
			a[1], -b[1], c[1],
			a[2], -b[2], c[2],
		}),
		mat.NewDense(3, 3, []float64{
			a[0], b[0], -c[0],
			a[1], b[1], -c[1],
			a[2], b[2], -c[2],
		}),
		mat.NewDense(3, 3, []float64{
			a[0], -b[0], -c[0],
			a[1], -b[1], -c[1],
			a[2], -b[2], -c[2],
		}),
		mat.NewDense(3, 3, []float64{
			-a[0], b[0], c[0],
			-a[1], b[1], c[1],
			-a[2], b[2], c[2],
		}),
		mat.NewDense(3, 3, []float64{
			-a[0], -b[0], c[0],
			-a[1], -b[1], c[1],
			-a[2], -b[2], c[2],
		}),
		mat.NewDense(3, 3, []float64{
			-a[0], b[0], -c[0],
			-a[1], b[1], -c[1],
			-a[2], b[2], -c[2],
		}),
		mat.NewDense(3, 3, []float64{
			-a[0], -b[0], -c[0],
			-a[1], -b[1], -c[1],
			-a[2], -b[2], -c[2],
		}),
	}
}

func GetTransformVariations() []*mat.Dense {
	variants := []*mat.Dense{}

	i := Vector{1, 0, 0}
	j := Vector{0, 1, 0}
	k := Vector{0, 0, 1}

	for _, x := range GetEightOrientationVariants(&i, &j, &k) {
		variants = append(variants, x)
	}

	for _, x := range GetEightOrientationVariants(&i, &k, &j) {
		variants = append(variants, x)
	}

	for _, x := range GetEightOrientationVariants(&j, &i, &k) {
		variants = append(variants, x)
	}

	for _, x := range GetEightOrientationVariants(&j, &k, &i) {
		variants = append(variants, x)
	}

	for _, x := range GetEightOrientationVariants(&k, &i, &j) {
		variants = append(variants, x)
	}

	for _, x := range GetEightOrientationVariants(&k, &j, &i) {
		variants = append(variants, x)
	}

	return variants
}

func solve(scanners []Scanner) []Scanner {
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
	scanners[0].Position = &Vector{}
	scanners[0].Transformation = NewIdentityDense()

	oriented := []Scanner{
		scanners[0],
	}
	remaining := scanners[1:]
	for len(remaining) > 0 {
		s := remaining[0]
		remaining = remaining[1:]
		ok := false
		for _, f := range oriented {
			if s.Orient(f.Beacons) {
				oriented = append(oriented, s.ApplyTransform())
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

func solvePartOne(scanners []Scanner) int {
	beacons := map[Vector]bool{}
	for _, s := range scanners {
		for _, b := range s.Beacons {
			beacons[b] = true
		}
	}

	return len(beacons)
}

func solvePartTwo(scanners []Scanner) int {
	var max float64 = 0
	for i, a := range scanners {
		for j, b := range scanners {
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
	scanners, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	scanners = solve(scanners)

	var result int
	result = solvePartOne(scanners)
	fmt.Printf("Part one: %v\n", result)

	result = solvePartTwo(scanners)
	fmt.Printf("Part two: %v\n", result)
}
