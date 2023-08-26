package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"math"
)

var wrongPair error = errors.New("Pair should be [A, B]")

type Snailfish struct {
	One   *Snailfish
	Two   *Snailfish
	Value int
}

func (s *Snailfish) ToString() string {
	if s.Value != -1 {
		return fmt.Sprintf("%d", s.Value)
	}

	return fmt.Sprintf("[%v,%v]", s.One.ToString(), s.Two.ToString())
}

func (s *Snailfish) ToFloat64() float64 {
	return float64(s.Value)
}

func (s *Snailfish) Magnitude() int {
	if s.IsNumber() {
		return s.Value
	}

    return s.One.Magnitude() * 3 + s.Two.Magnitude() * 2

}

func (s *Snailfish) Reduce() {
	for {
		if s.Explode() {
			continue
		}
		if s.Split() {
			continue
		}

		break
		// res := s.ToString()
		// reduced, _ := NewSnailfish(res)
		// return reduced
	}
}

func (s Snailfish) Add(other Snailfish) Snailfish {
	return Snailfish{
		One:   &s,
		Two:   &other,
		Value: -1,
	}
}

func (s *Snailfish) Split() bool {
	if !s.IsNumber() {
		if s.One.Split() {
			return true
		}
		if s.Two.Split() {
			return true
		}
	}

	if s.Value < 10 {
		return false
	}

	var val int
	val = int(math.Floor(s.ToFloat64() / 2))
	one := NewSnailfishFromInt(val)
	s.One = &one

	val = int(math.Ceil(s.ToFloat64() / 2))
	two := NewSnailfishFromInt(val)
	s.Two = &two

	// Reset value
	s.Value = -1

	return true
}

func (s *Snailfish) buildParents(parents map[*Snailfish]*Snailfish) {
	if !s.IsNumber() {
		parents[s.One] = s
		parents[s.Two] = s
		s.One.buildParents(parents)
		s.Two.buildParents(parents)
	}
}

func (s *Snailfish) getRelation(other *Snailfish) int {
	if s.IsNumber() {
		return -1
	}

	if s.One == other {
		return 1
	}

	if s.Two == other {
		return 2
	}

	return -1
}

func (s *Snailfish) Explode() bool {
	pair, pairPosition := s.getExplodePair(4)
	if pair == nil {
		return false
	}

	parents := map[*Snailfish]*Snailfish{}
	s.buildParents(parents)

	// Propagate Left value of pair to first parent regular value
	addToLeft := func(num *Snailfish, value int) {
		for {
			if parent, ok := parents[num]; ok {
				rel := parent.getRelation(num)
				// Left value on right position
				// Move top down
				if rel == 2 {
					next := parent.One
					for {
						if next.IsNumber() {
							next.Value += value
							return
						}
						next = next.Two
					}
				}
				// Move bottom up
				if parent.One.IsNumber() {
					parent.One.Value += value
					break
				}
				num = parent
			} else {
				break
			}
		}
	}
	addToLeft(pair, pair.One.Value)

	// Propagate Right value of pair to first parent regular value
	addToRight := func(num *Snailfish, value int) {
		for {
			if parent, ok := parents[num]; ok {
				rel := parent.getRelation(num)
				// Rigth value on left position
				// Move top down
				if rel == 1 {
					next := parent.Two
					for {
						if next.IsNumber() {
							next.Value += value
							return
						}
						next = next.One
					}
				}
				// Move bottom up
				if parent.Two.IsNumber() {
					parent.Two.Value += value
					break
				}
				num = parent
			} else {
				break
			}
		}
	}
	addToRight(pair, pair.Two.Value)

	// Replace exploded pair with zero regular number
	zero := NewSnailfishFromInt(0)
	if pairPosition == 1 {
		parents[pair].One = &zero
	} else if pairPosition == 2 {
		parents[pair].Two = &zero
	}

	return true
}

func (s *Snailfish) getExplodePair(count int) (*Snailfish, int) {
	if s.IsNumber() {
		return nil, -1
	}

	if count == 0 {
		return s, -1
	}

	var pair *Snailfish
	var i int

	pair, i = s.One.getExplodePair(count - 1)
	if i == -1 {
		i = 1
	}
	if pair != nil {
		return pair, i
	}

	pair, i = s.Two.getExplodePair(count - 1)
	if i == -1 {
		i = 2
	}
	if pair != nil {
		return pair, i
	}

	return nil, 0
}

func (s *Snailfish) IsNumber() bool {
	return s.Value != -1
}

// Creates Snailfish number from string
// Examples:
// [1,2]
// [[[[1,2],[3,4]],[[5,6],[7,8]]],9]
func NewSnailfish(input string) (Snailfish, error) {
	// variable to store the parsed data
	var parsedData interface{}

	// parsing the input string into the parsedData variable
	err := json.Unmarshal([]byte(input), &parsedData)
	if err != nil {
		return Snailfish{}, err
	}

	return newFromInterface(parsedData)
}

func NewSnailfishFromInt(val int) Snailfish {
	return Snailfish{
		Value: val,
	}
}

func newFromInterface(val interface{}) (Snailfish, error) {
	switch val.(type) {
	case float64:
		value := val.(float64)
		return NewSnailfishFromInt(int(value)), nil
	case []interface{}:
		pair := val.([]interface{})
		if len(pair) != 2 {
			return Snailfish{}, wrongPair
		}
		one, err := newFromInterface(pair[0])
		if err != nil {
			return Snailfish{}, err
		}
		two, err := newFromInterface(pair[1])
		if err != nil {
			return Snailfish{}, err
		}
		return Snailfish{
			One:   &one,
			Two:   &two,
			Value: -1,
		}, nil
	default:
		return Snailfish{}, nil
	}
}
