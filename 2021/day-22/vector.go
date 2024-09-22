package main

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

func (p *vector) Equal(v *vector) bool {
	return p.x == v.x && p.y == v.y && p.z == v.z
}

func (p *vector) Clone() *vector {
	return &vector{
		x: p.x,
		y: p.y,
		z: p.z,
	}
}

func (p *vector) Min(v *vector) {
	p.x = math.Min(p.x, v.x)
	p.y = math.Min(p.y, v.y)
	p.z = math.Min(p.z, v.z)
}

func (p *vector) Max(v *vector) {
	p.x = math.Max(p.x, v.x)
	p.y = math.Max(p.y, v.y)
	p.z = math.Max(p.z, v.z)
}

func (p *vector) Mult(val float64) {
	p.x *= val
	p.y *= val
	p.z *= val
}

func (p *vector) Add(other *vector) {
	p.x += other.x
	p.y += other.y
	p.z += other.z
}

func (p *vector) Sub(other *vector) {
	p.x -= other.x
	p.y -= other.y
	p.z -= other.z
}

func (p *vector) Clamp(min, max *vector) {
	p.x = clamp(p.x, min.x, max.x)
	p.y = clamp(p.y, min.y, max.y)
	p.z = clamp(p.z, min.z, max.z)
}

func (p *vector) Adjacents(ok func(p *vector) bool) <-chan vector {
	ch := make(chan vector)

	go func() {
		defer close(ch)

		dirs := []vector{
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

func NewFromInt(x, y, z int) *vector {
	return &vector{float64(x), float64(y), float64(z)}
}
