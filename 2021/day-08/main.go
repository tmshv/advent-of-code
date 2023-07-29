package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"os"
	"strings"
)

// Digit definition from the task
//
//   0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....
//
//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg

func getReferenceDigits() []int8 {
	const (
		A = 0
		B = 1
		C = 2
		D = 3
		E = 4
		F = 5
		G = 6
	)
	return []int8{
		// 0
		1<<A | 1<<B | 1<<C | 1<<E | 1<<F | 1<<G,

		// 1
		1<<C | 1<<F,

		// 2
		1<<A | 1<<C | 1<<D | 1<<E | 1<<G,

		// 3
		1<<A | 1<<C | 1<<D | 1<<F | 1<<G,

		// 4
		1<<B | 1<<C | 1<<D | 1<<F,

		// 5
		1<<A | 1<<B | 1<<D | 1<<F | 1<<G,

		// 6
		1<<A | 1<<B | 1<<D | 1<<E | 1<<F | 1<<G,

		// 7
		1<<A | 1<<C | 1<<F,

		// 8
		1<<A | 1<<B | 1<<C | 1<<D | 1<<E | 1<<F | 1<<G,

		// 9
		1<<A | 1<<B | 1<<C | 1<<D | 1<<F | 1<<G,
	}
}

type Sample struct {
	noise  []int8
	digits []int8
}

func (s *Sample) decode(shifts []int8, ref map[int8]bool) bool {
	decoded := make([]int8, len(s.noise))
	for i, n := range s.noise {
		powers := int8ToPowers(n)
		d := getDigit(powers, shifts)
		if _, ok := ref[d]; !ok {
			return false
		}
		decoded[i] = d
	}
	return true
}

func getDigit(xs, shifts []int8) int8 {
	var digit int8
	for _, x := range xs {
		shift := shifts[int(x)]
		digit |= 1 << shift
	}
	return digit
}

func permutations(arr []int8) [][]int8 {
	var helper func([]int8, int)
	res := [][]int8{}
	helper = func(arr []int8, n int) {
		if n == 1 {
			tmp := make([]int8, len(arr))
			copy(tmp, arr)
			res = append(res, tmp)
		} else {
			for i := 0; i < n; i++ {
				helper(arr, n-1)
				if n%2 == 1 {
					tmp := arr[i]
					arr[i] = arr[n-1]
					arr[n-1] = tmp
				} else {
					tmp := arr[0]
					arr[0] = arr[n-1]
					arr[n-1] = tmp
				}
			}
		}
	}
	helper(arr, len(arr))
	return res
}

func strToInt8(str string) int8 {
	var num int8
	for _, c := range str {
		num |= 1 << int(c-'a')
	}
	return num
}

func int8ToPowers(num int8) []int8 {
	powers := []int8{}
	for i := 0; i < 8; i++ {
		if num>>i&1 == 1 {
			powers = append(powers, int8(i))
		}
	}
	return powers
}

func readInput() ([]Sample, error) {
	samples := []Sample{}
	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		parts := strings.Split(scanner.Text(), " | ")
		if len(parts) != 2 {
			return nil, errors.New("Bad data")
		}

		noise := []int8{}
		for _, x := range strings.Split(parts[0], " ") {
			noise = append(noise, strToInt8(x))
		}

		digits := []int8{}
		for _, x := range strings.Split(parts[1], " ") {
			digits = append(digits, strToInt8(x))
		}

		samples = append(samples, Sample{
			noise:  noise,
			digits: digits,
		})
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return samples, nil
}

func solvePartOne(samples []Sample) int {
	reference := getReferenceDigits()
	ref := map[int8]bool{}
	for _, i := range reference {
		ref[i] = true
	}

	partOne := map[int8]bool{}
	partOne[reference[1]] = true
	partOne[reference[4]] = true
	partOne[reference[7]] = true
	partOne[reference[8]] = true

	count := 0
	for _, sample := range samples {
		for _, shifts := range permutations([]int8{0, 1, 2, 3, 4, 5, 6}) {
			if !sample.decode(shifts, ref) {
				continue
			}
			for _, d := range sample.digits {
				powers := int8ToPowers(d)
				digit := getDigit(powers, shifts)
				if _, ok := partOne[digit]; ok {
					count += 1
				}
			}
		}
	}

	return count
}

func solvePartTwo(samples []Sample) int {
	return 0
}

func main() {
	samples, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	var result int
	result = solvePartOne(samples)
	fmt.Printf("Part one: %v\n", result)

	result = solvePartTwo(samples)
	fmt.Printf("Part two: %v\n", result)
}
