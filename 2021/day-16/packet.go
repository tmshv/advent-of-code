package main

import (
	"encoding/binary"
	"fmt"
	"math"
)

func pow(x, y int) int {
	if y == 0 {
		return 1
	}
	val := x // x ^ 1
	for i := 2; i <= y; i++ {
		val = val * x
	}
	return val
}

func floor(a, b int) int {
	x := math.Floor(float64(a) / float64(b))
	return int(x)
}

func boolToUint64(val bool) uint64 {
	if val {
		return 1
	}
	return 0
}

func Uint64ToBytes(num uint64) []byte {
	b := make([]byte, 8)
	binary.BigEndian.PutUint64(b, num)
	return b
}

func BytesToUint64(bytes []byte) uint64 {
	var num uint64
	for i, x := range bytes {
		shift := 8 * (len(bytes) - 1 - i)
		num |= uint64(x) << shift
	}
	num <<= 64 - len(bytes)*8
	return num
}

type BitReader struct {
	Bytes    []byte
	Position int
}

func (r *BitReader) Done() bool {
	lastBit := len(r.Bytes)*8 - 1
	xs := r.GetSlice(lastBit - r.Position)

	for _, b := range xs {
		if b != 0 {
			return false
		}
	}
	return true
}

func (r *BitReader) ReadUint64(size int) uint64 {
	bits := r.ReadSlice(size)
	num := BytesToUint64(bits)
	num >>= 64 - size
	return num
}

func (r *BitReader) ReadSlice(size int) []byte {
	part := r.GetSlice(size)
	r.Position += size
	return part
}

func (r *BitReader) GetSlice(size int) []byte {
	i := floor(r.Position, 8)
	j := floor(r.Position+size, 8)

	frag := r.Bytes[i : j+1]
	part := make([]byte, len(frag))
	copy(part, frag)

	skip := r.Position % 8

	// Trim lower bits in last byte
	last := (r.Position + size) % 8
	mask := byte(pow(2, 8-last) - 1)
	mask = ^mask
	part[len(part)-1] &= mask

	for i = 0; i < len(part); i++ {
		// Move all bytes right
		part[i] <<= skip

		// Add big bits from next bytes for all except last byte
		if i < len(part)-1 {
			mask := pow(2, 8-skip) - 1
			mask = ^mask
			part[i] |= (part[i+1] & byte(mask)) >> byte(8-skip)
		}
	}

	return part
}

type Packet struct {
	Version  int
	TypeID   int
	Value    uint64
	Children []Packet
}

func (p *Packet) IsLiteral() bool {
	return p.TypeID == 4
}

func (p Packet) IterAll() <-chan Packet {
	ch := make(chan Packet)
	go func() {
		q := []Packet{p}
		for len(q) > 0 {
			packet := q[0]
			ch <- packet
			q = q[1:]
			if !packet.IsLiteral() {
				for _, subpacket := range packet.Children {
					q = append(q, subpacket)
				}
			}
		}
		defer close(ch)
	}()
	return ch
}

func (p *Packet) Operate() uint64 {
	var result uint64
	if p.TypeID == 0 {
		result = 0
		for _, packet := range p.Children {
			x := packet.Operate()
			result += x
		}
		return result
	}
	if p.TypeID == 1 {
		result = 1
		for _, packet := range p.Children {
			x := packet.Operate()
			result *= x
		}
		return result
	}
	if p.TypeID == 2 {
		result = math.MaxUint64
		for _, packet := range p.Children {
			x := packet.Operate()
			if x < result {
				result = x
			}
		}
		return result
	}
	if p.TypeID == 3 {
		result = 0
		for _, packet := range p.Children {
			x := packet.Operate()
			if x > result {
				result = x
			}
		}
		return result
	}
	if p.TypeID == 4 {
		return uint64(p.Value)
	}
	if p.TypeID == 5 {
		a := p.Children[0].Operate()
		b := p.Children[1].Operate()
		return boolToUint64(a > b)
	}
	if p.TypeID == 6 {
		a := p.Children[0].Operate()
		b := p.Children[1].Operate()
		return boolToUint64(a < b)
	}
	if p.TypeID == 7 {
		a := p.Children[0].Operate()
		b := p.Children[1].Operate()
		return boolToUint64(a == b)
	}
	return 0
}

func PrintPacket(packet *Packet, pad int) {
	ops := map[int]string{
		0: "+",
		1: "*",
		2: "min",
		3: "max",
		4: "val",
		5: ">",
		6: "<",
		7: "=",
	}
	op := ops[packet.TypeID]

	if packet.IsLiteral() {
		fmt.Printf("%*dPacket %s=%d", pad*4, packet.Version, op, packet.Value)
	} else {
		fmt.Printf("%*dPacket %s", pad*4, packet.Version, op)
	}

	fmt.Println()
	if !packet.IsLiteral() {
		pad += 1
		for _, sub := range packet.Children {
			PrintPacket(&sub, pad)
		}
	}
}

func PrintBytes(bytes []byte) {
	// Print each byte in binary format
	for _, b := range bytes {
		fmt.Printf("%8X ", b)
	}
	fmt.Println()
	for _, b := range bytes {
		fmt.Printf("%08b ", b)
	}
	fmt.Println()
}

func ReadPacket(r *BitReader) Packet {
	packetVersion := r.ReadUint64(3)
	packetType := r.ReadUint64(3)

	// Parse literal value packet
	if packetType == 4 {
		parts := []uint64{}
		for {
			group := r.ReadUint64(5)
			val := group & 0b1111
			parts = append(parts, val)
			groupStopMarker := group >> 4
			if groupStopMarker == 0 {
				break
			}
		}
		var value uint64
		for i, part := range parts {
			j := len(parts) - 1 - i
			part <<= j * 4
			value |= part
		}

		return Packet{
			Version: int(packetVersion),
			TypeID:  int(packetType),
			Value:   value,
		}
	}

	lengthTypeID := r.ReadUint64(1)

	// Parse 15bit
	if lengthTypeID == 0 {
		bodySize := int(r.ReadUint64(15))
		body := r.ReadSlice(bodySize)
		children := []Packet{}
		bodyReader := BitReader{body, 0}
		for !bodyReader.Done() {
			subpacket := ReadPacket(&bodyReader)
			children = append(children, subpacket)
		}
		return Packet{
			Version:  int(packetVersion),
			TypeID:   int(packetType),
			Children: children,
		}
	}

	// Parse 11bit
	subpackets := int(r.ReadUint64(11))
	children := make([]Packet, subpackets)
	for i := 0; i < subpackets; i++ {
		children[i] = ReadPacket(r)
	}
	return Packet{
		Version:  int(packetVersion),
		TypeID:   int(packetType),
		Children: children,
	}
}
