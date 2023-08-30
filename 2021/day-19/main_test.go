package main

import (
	"log"
	"math"
	"testing"

	"gonum.org/v1/gonum/mat"
)

// A comparison that treats NaNs as equal, for testing.
func same(m, b *mat.Dense) bool {
	mr, mc := b.Dims()
	br, bc := b.Dims()
	if br != mr || bc != mr {
		return false
	}
	for r := 0; r < mr; r++ {
		for c := 0; c < mc; c++ {
			if av, bv := m.At(r, c), b.At(r, c); av != bv && !(math.IsNaN(av) && math.IsNaN(bv)) {
				return false
			}
		}
	}
	return true
}

func f64(val float64) float64 {
  if val == -0 {
    return 0
  }
  return val
}

func equalF64(a, b []float64) bool {
	for i := range a {
    if f64(a[i]) != f64(b[i]) {
      return false
    }
	}
  return true
}

func TestMatrixInv(t *testing.T) {
	m := mat.NewDense(4, 4, []float64{
		0, 0, -1, 0,
		-1, 0, 0, 0,
		0, -1, 0, 0,
		0, 0, 0, 1,
	})

	i := mat.NewDense(4, 4, []float64{
		0, -1, 0, 0,
		0, 0, -1, 0,
		-1, 0, 0, 0,
		0, 0, 0, 1,
	})

	var inv mat.Dense
	err := inv.Inverse(m)
	if err != nil {
		t.Errorf("Matrix %v should be inversible", m)
	}

	if !same(i, &inv) {
		t.Errorf("Inv of Matrix %v is %v not %v", m, i, &inv)
	}
}

func TestVariants24(t *testing.T) {
	answers := [][]float64{
		{1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1},
		{1, 0, 0, 0, 0, 1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1},
		{1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1},
		{1, 0, 0, 0, 0, -1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1},
		{-1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1},
		{-1, 0, 0, 0, 0, 1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1},
		{-1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1},
		{-1, 0, 0, 0, 0, -1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1},
		{0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1},
		{0, 1, 0, 0, 1, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1},
		{0, 1, 0, 0, -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1},
		{0, 1, 0, 0, -1, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1},
		{0, -1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1},
		{0, -1, 0, 0, 1, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1},
		{0, -1, 0, 0, -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1},
		{0, -1, 0, 0, -1, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1},
		{0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1},
		{0, 0, 1, 0, 1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 1},
		{0, 0, 1, 0, -1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1},
		{0, 0, 1, 0, -1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 1},
		{0, 0, -1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1},
		{0, 0, -1, 0, 1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 1},
		{0, 0, -1, 0, -1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1},
		{0, 0, -1, 0, -1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 1},
	}

	variants := GetSensorVariants()
  for _, matrix := range variants {
    matrixRaw := matrix.RawMatrix().Data
    log.Println(matrixRaw)
  }

	for _, answer := range answers {
		found := false
		for _, matrix := range variants {
			matrixRaw := matrix.RawMatrix().Data
      if equalF64(matrixRaw, answer) {
        found = true
        break
      }
		}
		if !found {
			t.Errorf("Not found variant %v", answer)
		}
	}
}
