package main

import (
	"math"
)

type Box3 struct {
	Min vector
	Max vector
}

func (this *Box3) IsEmpty() bool {
	// this is a more robust check for empty than ( volume <= 0 ) because volume can get positive with two negative axes

	return (this.Max.x < this.Min.x) || (this.Max.y < this.Min.y) || (this.Max.z < this.Min.z)
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
	this.Min.x = math.MaxFloat64
	this.Min.y = math.MaxFloat64
	this.Min.z = math.MaxFloat64

	this.Max.x = -math.MaxFloat64
	this.Max.y = -math.MaxFloat64
	this.Max.z = -math.MaxFloat64
}

func (this *Box3) Center() *vector {
	if this.IsEmpty() {
		return &vector{}
	}

	center := this.Min
	center.Add(&this.Max)
	center.Mult(0.5)

	return &center
}

func (this *Box3) Size() *vector {
	if this.IsEmpty() {
		return &vector{}
	}
	size := this.Max
	size.Sub(&this.Min)
	return &size
}

func (this *Box3) Expand(value float64) {
	v := vector{value, value, value}
	this.Min.Sub(&v)
	this.Max.Add(&v)
}

func (this *Box3) Contains(point *vector) bool {
	return !(point.x < this.Min.x || point.x > this.Max.x ||
		point.y < this.Min.y || point.y > this.Max.y ||
		point.z < this.Min.z || point.z > this.Max.z)
}

func (this *Box3) ContainsBox(box *Box3) bool {
	return this.Min.x <= box.Min.x && box.Max.x <= this.Max.x &&
		this.Min.y <= box.Min.y && box.Max.y <= this.Max.y &&
		this.Min.z <= box.Min.z && box.Max.z <= this.Max.z
}

func (this *Box3) Within(box *Box3) bool {
	return this.Min.x > box.Min.x && this.Max.x < box.Max.x &&
		this.Min.y > box.Min.y && this.Max.y < box.Max.y &&
		this.Min.z > box.Min.z && this.Max.z < box.Max.z
}

func (this *Box3) IntersectsBox(box *Box3) bool {
	// using 6 splitting planes to rule out intersections.
	return !(box.Max.x < this.Min.x || box.Min.x > this.Max.x ||
		box.Max.y < this.Min.y || box.Min.y > this.Max.y ||
		box.Max.z < this.Min.z || box.Min.z > this.Max.z)
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

func (this *Box3) Translate(offset *vector) {
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
		if !box.IntersectsBox(part) {
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
		for _, part := range item.SplitX(box.Min.x) {
			next = append(next, part)
		}
	}
	result = next
	next = nil

	for _, item := range result {
		for _, part := range item.SplitX(box.Max.x) {
			next = append(next, part)
		}
	}
	result = next
	next = nil

	// Y
	for _, item := range result {
		for _, part := range item.SplitY(box.Min.y) {
			next = append(next, part)
		}
	}
	result = next
	next = nil

	for _, item := range result {
		for _, part := range item.SplitY(box.Max.y) {
			next = append(next, part)
		}
	}
	result = next
	next = nil

	// Z
	for _, item := range result {
		for _, b := range item.SplitZ(box.Min.z) {
			next = append(next, b)
		}
	}
	result = next
	next = nil

	for _, item := range result {
		for _, b := range item.SplitZ(box.Max.z) {
			next = append(next, b)
		}
	}
	result = next
	next = nil

	return result
}

func (this *Box3) SplitX(val float64) []*Box3 {
	if val <= this.Min.x || val >= this.Max.x {
		return []*Box3{this.Clone()}
	}

	var box *Box3
	var result []*Box3

	box = this.Clone()
	box.Max.x = val
	result = append(result, box)

	box = this.Clone()
	box.Min.x = val
	result = append(result, box)

	return result
}

func (this *Box3) SplitY(val float64) []*Box3 {
	if val <= this.Min.y || val >= this.Max.y {
		return []*Box3{this.Clone()}
	}

	var box *Box3
	var result []*Box3

	box = this.Clone()
	box.Max.y = val
	result = append(result, box)

	box = this.Clone()
	box.Min.y = val
	result = append(result, box)

	return result
}

func (this *Box3) SplitZ(val float64) []*Box3 {
	if val <= this.Min.z || val >= this.Max.z {
		return []*Box3{this.Clone()}
	}

	var box *Box3
	var result []*Box3

	box = this.Clone()
	box.Max.z = val
	result = append(result, box)

	box = this.Clone()
	box.Min.z = val
	result = append(result, box)

	return result
}

func NewFromCenterAndSize(center vector, size vector) *Box3 {
	halfSize := size
	halfSize.Mult(0.5)

	min := center
	min.Sub(&halfSize)

	max := center
	max.Add(&halfSize)

	return &Box3{min, max}
}

func NewFromMinMax(min, max vector) *Box3 {
	return &Box3{min, max}
}
