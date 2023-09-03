package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"os"
	"strings"
)

type Dice interface {
	Next() int
	Rolls() int
}

type DeterministicDice struct {
	value int
	max   int
	rolls int
}

func (dice *DeterministicDice) Next() int {
	v := dice.value
	dice.value++
	if dice.value > dice.max {
		dice.value = 1
	}
	dice.rolls++

	return v
}

func (dice *DeterministicDice) Rolls() int {
	return dice.rolls
}

func NewDice(max int) *DeterministicDice {
	return &DeterministicDice{
		value: 1,
		max:   max,
		rolls: 0,
	}
}

type Player struct {
	Position int64
	Score    int64
}

func NewPlayer(start int64) *Player {
	return &Player{
		Position: start,
		Score:    0,
	}
}

type State struct {
	player1   Player
	player2   Player
	turn bool
}

func (s *State) Roll(num int64) {
	var p *Player
	if s.turn {
		p = &s.player1
	} else {
		p = &s.player2
	}

	p.Position += num
	p.Position = (p.Position-1)%10 + 1

    p.Score += p.Position

	s.turn = !s.turn
}

func readInput() (int64, int64, error) {
	var parts []string
	scanner := bufio.NewScanner(os.Stdin)

	// P1
	scanner.Scan()
	parts = strings.Split(scanner.Text(), ": ")
	if len(parts) != 2 {
		return -1, -1, errors.New("Bad input")
	}
	var player1 int64
	fmt.Sscanf(parts[1], "%d", &player1)

	// P2
	scanner.Scan()
	parts = strings.Split(scanner.Text(), ": ")
	if len(parts) != 2 {
		return -1, -1, errors.New("Bad input")
	}
	var player2 int64
	fmt.Sscanf(parts[1], "%d", &player2)

	if err := scanner.Err(); err != nil {
		return -1, -1, err
	}

	return player1, player2, nil
}

func product3(min, max int) <-chan []int {
	ch := make(chan []int)

	go func() {
		defer close(ch)

		for i := min; i < max; i++ {
			for j := min; j < max; j++ {
				for k := min; k < max; k++ {
					ch <- []int{i, j, k}
				}
			}
		}
	}()

	return ch
}

func product() []int64 {
	res := []int64{}
	for x := range product3(1, 4) {
		res = append(res, int64(x[0]+x[1]+x[2]))
	}
	return res
}

type solver2 struct {
	win     int64
	cache   map[State][2]int64
	product []int64
}

func (s *solver2) solve(g State) (int64, int64) {
	if g.player1.Score >= s.win {
		return 1, 0
	} else if g.player2.Score >= s.win {
		return 0, 1
	}

	if r, ok := s.cache[g]; ok {
		return r[0], r[1]
	}

	var results [][2]int64

	for _, rolled := range s.product {
		gCopy := g
		gCopy.Roll(rolled)

		w1, w2 := s.solve(gCopy)
		results = append(results, [2]int64{w1, w2})
	}

	var w1 int64 = 0
	var w2 int64 = 0
	for _, res := range results {
		w1 += res[0]
		w2 += res[1]
	}

	s.cache[g] = [2]int64{w1, w2}

	return w1, w2
}

type solver1 struct {
	win  int64
	dice Dice
}

func (s *solver1) rolls() int64 {
	return int64(s.dice.Rolls())
}

func (player *Player) roll(dice Dice) bool {
    var x int
    x += dice.Next()
    x += dice.Next()
    x += dice.Next()
	player.Position += int64(x)
	player.Position = (player.Position-1)%10 + 1
	player.Score += player.Position

	return player.Score >= 1000
}

func (s *solver1) solve(game State) (int64, int64) {
	for {
        var x int
        x += s.dice.Next()
        x += s.dice.Next()
        x += s.dice.Next()

        game.Roll(int64(x))

        if game.player1.Score >= s.win || game.player2.Score >= s.win {
            break
        }
	}

	return game.player1.Score, game.player2.Score
}

func solvePartOne(p1, p2 int64) int64 {
	s := solver1{
		win:  1000,
		dice: NewDice(100),
	}
	game := State{
		player1:   *NewPlayer(p1),
		player2:   *NewPlayer(p2),
		turn: true,
	}
	w1, w2 := s.solve(game)
    lose := w1
    if w2 < w1 {
        lose = w2
    }

	return s.rolls() * lose
}

func solvePartTwo(p1, p2 int64) int64 {
	s := solver2{
		win:     21,
		cache:   map[State][2]int64{},
		product: product(),
	}
	game := State{
		player1:   *NewPlayer(p1),
		player2:   *NewPlayer(p2),
		turn: true,
	}
	w1, w2 := s.solve(game)

	if w1 > w2 {
		return w1
	}
	return w2
}

func main() {
	p1, p2, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	var result int64
	result = solvePartOne(p1, p2)
	fmt.Printf("Part one: %v\n", result)

	result = solvePartTwo(p1, p2)
	fmt.Printf("Part two: %v\n", result)
}
