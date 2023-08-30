package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Sensor struct {
	I        int
	Position *Vector3
	Matrix   *Matrix3D
	Beacons  []Vector3
}

func (s *Sensor) Apply() Sensor {
	beacons := make([]Vector3, len(s.Beacons))
	for i, b := range s.Beacons {
    p := s.Position
    // .ApplyMatrix3D(s.Matrix)
    n := b.ApplyMatrix3D(s.Matrix)
    n = n.Sub(p)
    // n = n.Sub(s.Position)
		beacons[i] = n
	}
	return Sensor{
		I:        s.I,
		Position: s.Position,
		Matrix:   NewIdentity(),
		Beacons:  beacons,
	}
}

func MatrixFromVectors(a, b, c Vector3) *Matrix3D {
	return NewMatrix3D([16]float64{
		a.X, b.X, c.X, 0,
		a.Y, b.Y, c.Y, 0,
		a.Z, b.Z, c.Z, 0,
		0, 0, 0, 1,
	})
}

func GetFourUpOrientationVariants(face, a, b Vector3) []Matrix3D {
	return []Matrix3D{
		*MatrixFromVectors(face, a, b),
		*MatrixFromVectors(face, a, b.Mult(-1)),
		*MatrixFromVectors(face, a.Mult(-1), b),
		*MatrixFromVectors(face, a.Mult(-1), b.Mult(-1)),
	}
}

type Vector3 struct {
	X float64
	Y float64
	Z float64
}

func (v *Vector3) Mult(val float64) Vector3 {
	return Vector3{
		v.X * val,
		v.Y * val,
		v.Z * val,
	}
}

func (v *Vector3) Add(val *Vector3) Vector3 {
	return Vector3{
		v.X + val.X,
		v.Y + val.Y,
		v.Z + val.Z,
	}
}

func (v *Vector3) Sub(val *Vector3) Vector3 {
	return Vector3{
		v.X - val.X,
		v.Y - val.Y,
		v.Z - val.Z,
	}
}

func (v *Vector3) ApplyMatrix3D(m *Matrix3D) Vector3 {
	return Vector3{
		m.v[v11]*v.X + m.v[v21]*v.Y + m.v[v31]*v.Z + m.v[v41],
		m.v[v12]*v.X + m.v[v22]*v.Y + m.v[v32]*v.Z + m.v[v42],
		m.v[v13]*v.X + m.v[v23]*v.Y + m.v[v33]*v.Z + m.v[v43],
	}
}

func NewVector3(val []float64) *Vector3 {
	return &Vector3{
		val[0],
		val[1],
		val[2],
	}
}

func readInput() ([]Sensor, error) {
	sensors := []Sensor{}
	scanner := bufio.NewScanner(os.Stdin)

	var buffer []Vector3
	i := 0
	for {
		// Skip row --- scanner X ---
		// Stop at EOL
		if !scanner.Scan() {
			break
		}

		buffer = []Vector3{}
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

func GetSensorVariants() []Matrix3D {
	var vs []Matrix3D
	variants := []Matrix3D{}

	vs = GetFourUpOrientationVariants(
		Vector3{1, 0, 0},
		Vector3{0, 1, 0},
		Vector3{0, 0, 1},
	)
	for _, x := range vs {
		variants = append(variants, x)
	}

	vs = GetFourUpOrientationVariants(
		Vector3{-1, 0, 0},
		Vector3{0, 1, 0},
		Vector3{0, 0, 1},
	)
	for _, x := range vs {
		variants = append(variants, x)
	}

	vs = GetFourUpOrientationVariants(
		Vector3{0, 1, 0},
		Vector3{1, 0, 0},
		Vector3{0, 0, 1},
	)
	for _, x := range vs {
		variants = append(variants, x)
	}

	vs = GetFourUpOrientationVariants(
		Vector3{0, -1, 0},
		Vector3{1, 0, 0},
		Vector3{0, 0, 1},
	)
	for _, x := range vs {
		variants = append(variants, x)
	}

	vs = GetFourUpOrientationVariants(
		Vector3{0, 0, 1},
		Vector3{1, 0, 0},
		Vector3{0, 1, 0},
	)
	for _, x := range vs {
		variants = append(variants, x)
	}
	vs = GetFourUpOrientationVariants(
		Vector3{0, 0, -1},
		Vector3{1, 0, 0},
		Vector3{0, 1, 0},
	)
	for _, x := range vs {
		variants = append(variants, x)
	}

	return variants
}

func AdjustSensor(sensor *Sensor, beacons []Vector3) bool {
	for _, matrix := range GetSensorVariants() {
		subtractCounts := map[Vector3]int{}
		for _, beacon := range beacons {
			for _, b := range sensor.Beacons {
				t := b.ApplyMatrix3D(&matrix)
				sub := beacon.Sub(&t)
				subtractCounts[sub]++
			}
		}
		for pos, count := range subtractCounts {
			if count >= 12 {
				sensor.Position = &pos
				sensor.Matrix = &matrix
				return true
			// } else {
   //      log.Printf("%+v", subtractCounts)
      }
		}
	}
	return false
}

func solvePartOne(sensors []Sensor) int {
	// 0 Sensor is identity
	sensors[0].Position = &Vector3{}
	sensors[0].Matrix = NewIdentity()

	// found := []Sensor{
	// 	sensors[0],
	// }
	// q := sensors[1:]
	// for len(q) > 0 {
	// 	log.Printf("found=%d remaining=%d", len(found), len(q))
	// 	next := []Sensor{}
	// 	for _, blank := range q {
	// 		a := true
	// 		for _, f := range found {
	// 			if AdjustSensor(&blank, &f) {
	//          r := blank.Apply()
	// 				found = append(found, r)
	// 				a = false
	// 				break
	// 			}
	// 		}
	// 		if a {
	// 			next = append(next, blank)
	// 		}
	// 	}
	// 	q = next
	// }

	return len(sensors)
}

func solvePartTwo(sensors []Sensor) int {
	// 0 Sensor is identity
	sensors[0].Position = &Vector3{}
	sensors[0].Matrix = NewIdentity()

	sensor0 := sensors[0]

	s1 := sensors[1]
	if AdjustSensor(&s1, sensor0.Beacons) {
		log.Printf("S1 position: %v", s1.Position)
		s1 = s1.Apply()
	}

	s := sensors[4]
	if AdjustSensor(&s, s1.Beacons) {
		log.Printf("Found position: %v", s.Position)
	}

	return 0
}

func main() {
	sensors, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	var result int
	result = solvePartOne(sensors)
	fmt.Printf("Part one: %v\n", result)

	result = solvePartTwo(sensors)
	fmt.Printf("Part two: %v\n", result)
}
