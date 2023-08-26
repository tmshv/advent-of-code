package main

import (
	"testing"
)

func TestAdd(t *testing.T) {
	var s1 Snailfish
	var s2 Snailfish
	var s3 Snailfish
	var a string
	var b string
	var result string
	var out string

	a = "[1,2]"
	b = "[[3,4],5]"
	result = "[[1,2],[[3,4],5]]"
	s1, _ = NewSnailfish(a)
	s2, _ = NewSnailfish(b)

	s3 = s1.Add(s2)
	out = s3.ToString()
	if result != out {
		t.Errorf("Wrong add of %s + %s: %s is not equal %s", a, b, out, result)
	}
}

func TestExplode(t *testing.T) {
	var s Snailfish
	var num string
	var result string
	var out string

	num = "[[[[[9,8],1],2],3],4]"
	result = "[[[[0,9],2],3],4]"
	s, _ = NewSnailfish(num)
	s.Explode()
	out = s.ToString()
	if result != out {
		t.Errorf("Wrong explode of %s: %s is not equal %s", num, out, result)
	}

	num = "[7,[6,[5,[4,[3,2]]]]]"
	result = "[7,[6,[5,[7,0]]]]"
	s, _ = NewSnailfish(num)
	s.Explode()
	out = s.ToString()
	if result != out {
		t.Errorf("Wrong explode of %s: %s is not equal %s", num, out, result)
	}

	num = "[[6,[5,[4,[3,2]]]],1]"
	result = "[[6,[5,[7,0]]],3]"
	s, _ = NewSnailfish(num)
	s.Explode()
	out = s.ToString()
	if result != out {
		t.Errorf("Wrong explode of %s: %s is not equal %s", num, out, result)
	}

	num = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"
	result = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
	s, _ = NewSnailfish(num)
	s.Explode()
	out = s.ToString()
	if result != out {
		t.Errorf("Wrong explode of %s: %s is not equal %s", num, out, result)
	}

	num = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
	result = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
	s, _ = NewSnailfish(num)
	s.Explode()
	out = s.ToString()
	if result != out {
		t.Errorf("Wrong explode of %s: %s is not equal %s", num, out, result)
	}
}

func TestSplit(t *testing.T) {
	var num int
	var s Snailfish
	var result string
	var out string

	num = 10
	result = "[5,5]"
	s = NewSnailfishFromInt(num)
	s.Split()
	out = s.ToString()
	if result != out {
		t.Errorf("Wrong split of %d: %s is not equal %s", num, out, result)
	}

	num = 11
	result = "[5,6]"
	s = NewSnailfishFromInt(num)
	s.Split()
	out = s.ToString()
	if result != out {
		t.Errorf("Wrong split of %d: %s is not equal %s", num, out, result)
	}

	num = 12
	result = "[6,6]"
	s = NewSnailfishFromInt(num)
	s.Split()
	out = s.ToString()
	if result != out {
		t.Errorf("Wrong split of %d: %s is not equal %s", num, out, result)
	}
}

func TestReduce(t *testing.T) {
	a, _ := NewSnailfish("[[[[4,3],4],4],[7,[[8,4],9]]]")
	b, _ := NewSnailfish("[1,1]")
	result := "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
	s := a.Add(b)
	s.Reduce()
	out := s.ToString()
	if result != out {
		t.Errorf("Wrong reduce of %s: %s is not equal %s", s.ToString(), out, result)
	}
}

func TestExample1(t *testing.T) {
	result := "[[[[1,1],[2,2]],[3,3]],[4,4]]"
	nums := []string{
		"[1,1]",
		"[2,2]",
		"[3,3]",
		"[4,4]",
	}
	a, _ := NewSnailfish(nums[0])
	for _, n := range nums[1:] {
		b, _ := NewSnailfish(n)
		a = a.Add(b)
		a.Reduce()
	}
	out := a.ToString()
	if result != out {
		t.Errorf("Wrong sum of %s: %s is not equal %s", nums, out, result)
	}
}

func TestExample2(t *testing.T) {
	result := "[[[[3,0],[5,3]],[4,4]],[5,5]]"
	nums := []string{
		"[1,1]",
		"[2,2]",
		"[3,3]",
		"[4,4]",
		"[5,5]",
	}
	a, _ := NewSnailfish(nums[0])
	for _, n := range nums[1:] {
		b, _ := NewSnailfish(n)
		a = a.Add(b)
		a.Reduce()
	}
	out := a.ToString()
	if result != out {
		t.Errorf("Wrong sum of %s: %s is not equal %s", nums, out, result)
	}
}

func TestExample3(t *testing.T) {
	result := "[[[[5,0],[7,4]],[5,5]],[6,6]]"
	nums := []string{
		"[1,1]",
		"[2,2]",
		"[3,3]",
		"[4,4]",
		"[5,5]",
		"[6,6]",
	}
	a, _ := NewSnailfish(nums[0])
	for _, n := range nums[1:] {
		b, _ := NewSnailfish(n)
		a = a.Add(b)
		a.Reduce()
	}
	out := a.ToString()
	if result != out {
		t.Errorf("Wrong sum of %s: %s is not equal %s", nums, out, result)
	}
}

func TestExampleLarge(t *testing.T) {
	result := "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
	nums := []string{
		"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
		"[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
		"[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
		"[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
		"[7,[5,[[3,8],[1,4]]]]",
		"[[2,[2,2]],[8,[8,1]]]",
		"[2,9]",
		"[1,[[[9,3],9],[[9,0],[0,7]]]]",
		"[[[5,[7,4]],7],1]",
		"[[[[4,2],2],6],[8,7]]",
	}
	a, _ := NewSnailfish(nums[0])
	for _, n := range nums[1:] {
		b, _ := NewSnailfish(n)
		a = a.Add(b)
		a.Reduce()
	}
	out := a.ToString()
	if result != out {
		t.Errorf("Wrong sum of %s: %s is not equal %s", nums, out, result)
	}
}

func TestMagnitude(t *testing.T) {
	nums := []string{
		"[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
		"[[1,2],[[3,4],5]]",
		"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
		"[[[[1,1],[2,2]],[3,3]],[4,4]]",
		"[[[[3,0],[5,3]],[4,4]],[5,5]]",
		"[[[[5,0],[7,4]],[5,5]],[6,6]]",
		"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
	}
	results := []int{
		4140,
		143,
		1384,
		445,
		791,
		1137,
		3488,
	}

	for i, n := range nums {
		s, _ := NewSnailfish(n)
		m := s.Magnitude()
		if m != results[i] {
			t.Errorf("Wrong magnitude of %s: %d is not equal %d", n, m, results[i])
		}
	}
}
