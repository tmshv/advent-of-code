package main

import (
	"math"
)

type Box3 struct {
	Min Vector
	Max Vector
}

func (this *Box3) IsEmpty() bool {
	// this is a more robust check for empty than ( volume <= 0 ) because volume can get positive with two negative axes

	return (this.Max.X < this.Min.X) || (this.Max.Y < this.Min.Y) || (this.Max.Z < this.Min.Z)
}

func (this *Box3) Clone() *Box3 {
	return &Box3{
		Min: *this.Min.Clone(),
		Max: *this.Max.Clone(),
	}
}

func (this *Box3) Equal(box *Box3) bool {
	return this.Min.Equal(&box.Min) && this.Max.Equal(&box.Max)
}

func (this *Box3) MakeEmpty() {
	this.Min.X = math.MaxFloat64
	this.Min.Y = math.MaxFloat64
	this.Min.Z = math.MaxFloat64

	this.Max.X = -math.MaxFloat64
	this.Max.Y = -math.MaxFloat64
	this.Max.Z = -math.MaxFloat64
}

func (this *Box3) Center() *Vector {
	if this.IsEmpty() {
		return &Vector{}
	}

	center := this.Min
	center.Add(&this.Max)
	center.Mult(0.5)

	return &center
}

func (this *Box3) Size() *Vector {
	if this.IsEmpty() {
		return &Vector{}
	}
	size := this.Max
	size.Sub(&this.Min)
	return &size
}

func (this *Box3) Expand(value float64) {
	v := Vector{value, value, value}
	this.Min.Sub(&v)
	this.Max.Add(&v)
}

func (this *Box3) Contains(point *Vector) bool {
	return !(point.X < this.Min.X || point.X > this.Max.X ||
		point.Y < this.Min.Y || point.Y > this.Max.Y ||
		point.Z < this.Min.Z || point.Z > this.Max.Z)
}

func (this *Box3) ContainsBox(box *Box3) bool {
	return this.Min.X <= box.Min.X && box.Max.X <= this.Max.X &&
		this.Min.Y <= box.Min.Y && box.Max.Y <= this.Max.Y &&
		this.Min.Z <= box.Min.Z && box.Max.Z <= this.Max.Z
}

func (this *Box3) Within(box *Box3) bool {
	return this.Min.X > box.Min.X && this.Max.X < box.Max.X &&
		this.Min.Y > box.Min.Y && this.Max.Y < box.Max.Y &&
		this.Min.Z > box.Min.Z && this.Max.Z < box.Max.Z
}

func (this *Box3) IntersectsBox(box *Box3) bool {
	// using 6 splitting planes to rule out intersections.
	return !(box.Max.X < this.Min.X || box.Min.X > this.Max.X ||
		box.Max.Y < this.Min.Y || box.Min.Y > this.Max.Y ||
		box.Max.Z < this.Min.Z || box.Min.Z > this.Max.Z)
}

// clampPoint( point, target ) {
//
//     return target.copy( point ).clamp( this.min, this.max );
//
// }

// distanceToPoint( point ) {
//
//     return this.clampPoint( point, _vector ).distanceTo( point );
//
// }

func (this *Box3) Intersect(box *Box3) {
	this.Min.Max(&box.Min)
	this.Max.Min(&box.Max)

	// ensure that if there is no overlap, the result is fully empty, not slightly empty with non-inf/+inf values that will cause subsequence intersects to erroneously return valid values.
	if this.IsEmpty() {
		this.MakeEmpty()
	}
}

func (this *Box3) Union(box *Box3) {
	this.Min.Min(&box.Min)
	this.Max.Max(&box.Max)
}

func (this *Box3) Translate(offset *Vector) {
	this.Min.Add(offset)
	this.Max.Add(offset)
}

func (this *Box3) Join(box *Box3) []*Box3 {
	var result []*Box3

	result = append(result, this.Clone())
	for _, part := range box.Split(this) {
		if !this.ContainsBox(part) {
			result = append(result, part)
		}
	}
	return result
}

func (this *Box3) Subtract(box *Box3) []*Box3 {
	var result []*Box3
	for _, part := range this.Split(box) {
		if !box.ContainsBox(part) {
			result = append(result, part)
		}
	}
	return result
}

func (this *Box3) Split(box *Box3) []*Box3 {
	var result []*Box3
	var next []*Box3

	// Init
	result = append(result, this.Clone())

	// X
	for _, item := range result {
		for _, part := range item.SplitX(box.Min.X) {
			next = append(next, part)
		}
	}
	result = next
	next = nil

	for _, item := range result {
		for _, part := range item.SplitX(box.Max.X) {
			next = append(next, part)
		}
	}
	result = next
	next = nil

	// Y
	for _, item := range result {
		for _, part := range item.SplitY(box.Min.Y) {
			next = append(next, part)
		}
	}
	result = next
	next = nil

	for _, item := range result {
		for _, part := range item.SplitY(box.Max.Y) {
			next = append(next, part)
		}
	}
	result = next
	next = nil

	// Z
	for _, item := range result {
		for _, b := range item.SplitZ(box.Min.Z) {
			next = append(next, b)
		}
	}
	result = next
	next = nil

	for _, item := range result {
		for _, b := range item.SplitZ(box.Max.Z) {
			next = append(next, b)
		}
	}
	result = next
	next = nil

	return result
}

func (this *Box3) SplitX(val float64) []*Box3 {
	if val <= this.Min.X || val >= this.Max.X {
		return []*Box3{this.Clone()}
	}

	var box *Box3
	var result []*Box3

	box = this.Clone()
	box.Max.X = val
	result = append(result, box)

	box = this.Clone()
	box.Min.X = val
	result = append(result, box)

	return result
}

func (this *Box3) SplitY(val float64) []*Box3 {
	if val <= this.Min.Y || val >= this.Max.Y {
		return []*Box3{this.Clone()}
	}

	var box *Box3
	var result []*Box3

	box = this.Clone()
	box.Max.Y = val
	result = append(result, box)

	box = this.Clone()
	box.Min.Y = val
	result = append(result, box)

	return result
}

func (this *Box3) SplitZ(val float64) []*Box3 {
	if val <= this.Min.Z || val >= this.Max.Z {
		return []*Box3{this.Clone()}
	}

	var box *Box3
	var result []*Box3

	box = this.Clone()
	box.Max.Z = val
	result = append(result, box)

	box = this.Clone()
	box.Min.Z = val
	result = append(result, box)

	return result
}

func NewFromCenterAndSize(center Vector, size Vector) *Box3 {
	halfSize := size
	halfSize.Mult(0.5)

	min := center
	min.Sub(&halfSize)

	max := center
	max.Add(&halfSize)

	return &Box3{min, max}
}

func NewFromMinMax(min, max Vector) *Box3 {
	return &Box3{min, max}
}
