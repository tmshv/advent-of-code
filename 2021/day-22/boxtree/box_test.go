package boxtree

import "testing"

func TestBoxSplitX(t *testing.T) {
	tests := []struct {
		test   string
		expect []Box3
		box    Box3
		val    float64
	}{
		{
			test: "split to one pieces if axis on side",
			val:  0,
			expect: []Box3{
				{
					Min: Vector{0, 0, 0},
					Max: Vector{2, 2, 2},
				},
			},
			box: Box3{
				Min: Vector{0, 0, 0},
				Max: Vector{2, 2, 2},
			},
		},
		{
			test: "split to one pieces if axis outside",
			val:  5,
			expect: []Box3{
				{
					Min: Vector{0, 0, 0},
					Max: Vector{2, 2, 2},
				},
			},
			box: Box3{
				Min: Vector{0, 0, 0},
				Max: Vector{2, 2, 2},
			},
		},
		{
			test: "split to two pieces",
			val:  1,
			expect: []Box3{
				{
					Min: Vector{0, 0, 0},
					Max: Vector{1, 0, 0},
				},
				{
					Max: Vector{1, 0, 0},
					Min: Vector{2, 2, 2},
				},
			},
			box: Box3{
				Min: Vector{0, 0, 0},
				Max: Vector{2, 2, 2},
			},
		},
	}

	for _, x := range tests {
		t.Run(x.test, func(t *testing.T) {
			result := x.box.SplitX(x.val)
			size := len(result)
			if size != len(x.expect) {
				for _, box := range result {
					t.Logf("%v", *box)
				}
				t.Errorf("Wrong split x result %d. Expected %d", size, len(x.expect))
			}
		})
	}
}

func TestBoxSplit(t *testing.T) {
	tests := []struct {
		test   string
		expect int
		a      Box3
		b      Box3
	}{
		{
			test:   "one fully inside another",
			expect: 27,
			a: Box3{
				Min: Vector{0, 0, 0},
				Max: Vector{9, 9, 9},
			},
			b: Box3{
				Min: Vector{1, 1, 1},
				Max: Vector{8, 8, 8},
			},
		},
		{
			test:   "one fully cover another",
			expect: 1,
			a: Box3{
				Min: Vector{1, 1, 1},
				Max: Vector{8, 8, 8},
			},
			b: Box3{
				Min: Vector{0, 0, 0},
				Max: Vector{9, 9, 9},
			},
		},
		{
			test:   "one cover half of another by each axis",
			expect: 8,
			a: Box3{
				Min: Vector{0, 0, 0},
				Max: Vector{4, 4, 4},
			},
			b: Box3{
				Min: Vector{2, 2, 2},
				Max: Vector{6, 6, 6},
			},
		},
		{
			test:   "one inside another but one side are overlapping",
			expect: 27 - 9, // 27 are parts of main box minus 9 boxes should gone
			a: Box3{
				Min: Vector{0, 0, 0},
				Max: Vector{4, 4, 4},
			},
			b: Box3{
				Min: Vector{0, 1, 1},
				Max: Vector{3, 3, 3},
			},
		},
	}

	for _, x := range tests {
		t.Run(x.test, func(t *testing.T) {
			result := x.a.Split(&x.b)
			size := len(result)
			if size != x.expect {
				for _, box := range result {
					t.Logf("%v", *box)
				}
				t.Errorf("Wrong split result %d. Expected %d", size, x.expect)
			}
		})
	}
}

func TestBoxJoin(t *testing.T) {
	tests := []struct {
		test   string
		expect int
		a      Box3
		b      Box3
	}{
		{
			test: "one fully inside another",
			a: Box3{
				Min: Vector{0, 0, 0},
				Max: Vector{9, 9, 9},
			},
			b: Box3{
				Min: Vector{1, 1, 1},
				Max: Vector{8, 8, 8},
			},
			expect: 1,
		},
		{
			test: "one intersecting another by one axis",
			a: Box3{
				Min: Vector{0, 0, 0},
				Max: Vector{8, 8, 8},
			},
			b: Box3{
				Min: Vector{1, 1, 1},
				Max: Vector{9, 7, 7},
			},
			expect: 2,
		},
		{
			test: "one intersecting another by two axes",
			a: Box3{
				Min: Vector{0, 0, 0},
				Max: Vector{8, 8, 8},
			},
			b: Box3{
				Min: Vector{1, 1, 1},
				Max: Vector{9, 9, 7},
			},
			expect: 4,
		},
		{
			test: "one intersecting another by thre axes",
			a: Box3{
				Min: Vector{0, 0, 0},
				Max: Vector{8, 8, 8},
			},
			b: Box3{
				Min: Vector{1, 1, 1},
				Max: Vector{9, 9, 9},
			},
			expect: 8,
		},
		{
			test: "two boxes are not overlapping",
			a: Box3{
				Min: Vector{0, 0, 0},
				Max: Vector{1, 1, 1},
			},
			b: Box3{
				Min: Vector{2, 2, 2},
				Max: Vector{3, 3, 3},
			},
			expect: 2,
		},
	}

	for _, x := range tests {
		t.Run(x.test, func(t *testing.T) {
			result := x.a.Join(&x.b)

			for i := 0; i < len(result); i++ {
				for j := i + 1; j < len(result); j++ {
					if result[i].Equal(result[j]) {
						t.Errorf("Boxes in join result are not unique. %d and %d are both equal", i, j)
					}
				}
			}

			size := len(result)
			if size != x.expect {
				for _, box := range result {
					t.Logf("%v", *box)
				}
				t.Errorf("Wrong join result %d. Expected %d", size, x.expect)
			}
		})
	}
}

func TestBoxSubtract(t *testing.T) {
	tests := []struct {
		test   string
		expect int
		a      Box3
		b      Box3
	}{
		{
			test: "test0",
			a: Box3{
				Min: Vector{11, 11, 11},
				Max: Vector{13, 13, 13},
			},
			b: Box3{
				Min: Vector{10, 10, 10},
				Max: Vector{12, 12, 12},
			},
			expect: 7,
		},
	}

	for _, x := range tests {
		t.Run(x.test, func(t *testing.T) {
			result := x.a.Subtract(&x.b)

			for i := 0; i < len(result); i++ {
				if x.b.ContainsBox(result[i]) {
					t.Errorf("Operand B contains subtracted box %v", *result[i])
				}
			}

			size := len(result)
			if size != x.expect {
				for _, box := range result {
					t.Logf("%v", *box)
				}
				t.Errorf("Wrong subtract result %d. Expected %d", size, x.expect)
			}
		})
	}
}
