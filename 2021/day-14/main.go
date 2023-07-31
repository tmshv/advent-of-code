package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"math"
	"os"
	"regexp"
)

type Rule struct {
	Pair      string
	Insertion string
}

func (r *Rule) Match(pair string) bool {
	return r.Pair == pair
}

// func (r *Rule) Transform(val string, index int) string {
//     // return val[:index] + r.Insertion + val[index:]
// }

func (r *Rule) Transform(pair string) string {
	return string(pair[0]) + r.Insertion + string(pair[1])
}

func readInput() (string, []Rule, error) {
	regex := regexp.MustCompile(`(\w{2}) -> (\w)`)
	scanner := bufio.NewScanner(os.Stdin)

	// Template
	scanner.Scan()
	template := scanner.Text()

	// Empty line
	scanner.Scan()

	// Rules
	rules := []Rule{}
	for scanner.Scan() {
		row := scanner.Text()
		match := regex.FindStringSubmatch(row)
		// Ensure that a match is found
		if len(match) != 3 {
			return "", nil, errors.New("Bad rules data")
		}

		// Extract the axis ('x' or 'y') and the value
		rule := Rule{
			Pair:      match[1],
			Insertion: match[2],
		}

		rules = append(rules, rule)
	}

	if err := scanner.Err(); err != nil {
		return "", nil, err
	}

	return template, rules, nil
}

func SplitToChunks(val string, n int) <-chan string {
	ch := make(chan string)

	go func() {
		defer close(ch)

		for i := 0; i < len(val)-n+1; i++ {
			j := i + n
			pair := val[i:j]

			ch <- pair
		}
	}()

	return ch
}

func Polymerize(template string, rules []Rule, steps int) map[rune]int {
	ruleMap := map[string]Rule{}
	for _, r := range rules {
		ruleMap[r.Pair] = r
	}

	var polymer string
	for i := 0; i < steps; i++ {
		polymer = ""
		for pair := range SplitToChunks(template, 2) {
			if rule, ok := ruleMap[pair]; ok {
				t := rule.Transform(pair)
				if len(polymer) == 0 {
					polymer += t
				} else {
					polymer += t[1:]
				}
			}
		}
		template = polymer
	}

	count := map[rune]int{}
	for _, r := range polymer {
		count[r]++
	}
    return count
}

func PolymerizeFast(init string, rules []Rule, steps int) map[rune]int {
	ruleMap := map[string]Rule{}
	for _, r := range rules {
		ruleMap[r.Pair] = r
	}

	elements := map[rune]int{}
	for _, r := range init {
		elements[r]++
	}

    polymer := map[string]int{}
	for pair := range SplitToChunks(init, 2) {
		polymer[pair]++
	}

	for i := 0; i < steps; i++ {
        nextPolymer := map[string]int{}

        for pair, count := range polymer {
            a := string(pair[0])
            b := string(pair[1])

            rule := ruleMap[pair]

            nextPolymer[a+rule.Insertion] += count
            nextPolymer[rule.Insertion+b] += count

            elements[rune(rule.Insertion[0])] += count
        }

        polymer = nextPolymer
	}

	return elements
}

func FindMostAndLeastCommon(m map[rune]int) (int, int) {
	var max = math.MinInt64 // Initialize max as the smallest possible integer
	var min = math.MaxInt64 // Initialize min as the largest possible integer

	for _, val := range m {
		if val > max {
			max = val
		}

		if val < min {
			min = val
		}
	}

	return max, min
}

func solvePartOne(template string, rules []Rule) int {
	count := PolymerizeFast(template, rules, 10)
	most, least := FindMostAndLeastCommon(count)
	return most - least
}

func solvePartTwo(template string, rules []Rule) int {
	count := PolymerizeFast(template, rules, 40)
	most, least := FindMostAndLeastCommon(count)
	return most - least
}

func main() {
	template, rules, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	var result int
	result = solvePartOne(template, rules)
	fmt.Printf("Part one: %v\n", result)

	result = solvePartTwo(template, rules)
	fmt.Printf("Part two: %v\n", result)
}
