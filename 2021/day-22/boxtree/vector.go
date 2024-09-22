package boxtree

import "math"

func clamp(value, min, max float64) float64 {
	if value < min {
		return min
	}
	if value > max {
		return max
	}
	return value
}

type Vector struct {
	X float64
	Y float64
	Z float64
}

func (p *Vector) Equal(v *Vector) bool {
	return p.X == v.X && p.Y == v.Y && p.Z == v.Z
}

func (p *Vector) Clone() *Vector {
	return &Vector{
		X: p.X,
		Y: p.Y,
		Z: p.Z,
	}
}

func (p *Vector) Min(v *Vector) {
	p.X = math.Min(p.X, v.X)
	p.Y = math.Min(p.Y, v.Y)
	p.Z = math.Min(p.Z, v.Z)
}

func (p *Vector) Max(v *Vector) {
	p.X = math.Max(p.X, v.X)
	p.Y = math.Max(p.Y, v.Y)
	p.Z = math.Max(p.Z, v.Z)
}

func (p *Vector) Mult(val float64) {
	p.X *= val
	p.Y *= val
	p.Z *= val
}

func (p *Vector) Add(other *Vector) {
	p.X += other.X
	p.Y += other.Y
	p.Z += other.Z
}

func (p *Vector) Sub(other *Vector) {
	p.X -= other.X
	p.Y -= other.Y
	p.Z -= other.Z
}

func (p *Vector) Clamp(min, max *Vector) {
	p.X = clamp(p.X, min.X, max.X)
	p.Y = clamp(p.Y, min.Y, max.Y)
	p.Z = clamp(p.Z, min.Z, max.Z)
}

func (p *Vector) Adjacents(ok func(p *Vector) bool) <-chan Vector {
	ch := make(chan Vector)

	go func() {
		defer close(ch)

		dirs := []Vector{
			{0, -1, 0},
			{0, 1, 0},
			{-1, 0, 0},
			{1, 0, 0},
		}

		for _, v := range dirs {
			v.Add(p)
			if ok(&v) {
				ch <- v
			}
		}
	}()

	return ch
}

func NewFromInt(x, y, z int) *Vector {
	return &Vector{float64(x), float64(y), float64(z)}
}
