package main

import (
	"encoding/hex"
	"testing"
)

func sliceEq(a, b []byte) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}

func TestReadSlice(t *testing.T) {
	var r BitReader
	var part []byte
	var ans []byte
	b := []byte{
		0b00111000, 0b00000000, 0b01101111, 0b01000101, 0b00101001, 0b00010010, 0b00000000,
	}
	// 00111000 00000000 01101111 01000101 00101001 00010010 00000000
	//     1000 00000000 01101
	// 10000000 00000110 10000000
	r = BitReader{b, 4}
	part = r.ReadSlice(17)
	ans = []byte{
		0b10000000, 0b00000110, 0b10000000,
	}
	if !sliceEq(part, ans) {
		PrintBytes(part)
		t.Errorf("%v is not equal %v", part, ans)
	}

	// 00111000 00000000 01101111 01000101 00101001 00010010 00000000
	//                     101111 01000101 00101
	//                   10111101 00010100 10100000
	r = BitReader{b, 18}
	part = r.ReadSlice(19)
	ans = []byte{
		0b10111101, 0b00010100, 0b10100000,
	}
	if !sliceEq(part, ans) {
		PrintBytes(part)
		t.Errorf("%v is not equal %v", part, ans)
	}
}

func TestReadLiteralPacket(t *testing.T) {
	input := "D2FE28"
	bytes, err := hex.DecodeString(input)
	if err != nil {
		t.Error(err)
	}

	reader := BitReader{bytes, 0}
	packet := ReadPacket(&reader)
	if packet.Version != 6 {
		t.Errorf("Wrong Version of Packet %s", input)
	}
	if packet.TypeID != 4 {
		t.Errorf("Wrong Type ID of Packet %s", input)
	}
	if packet.Value != 2021 {
		t.Errorf("Wrong Value of Packet %s", input)
	}
	if packet.Children != nil {
		t.Errorf("Packet %s sould be with no children", input)
	}
}

func TestRead15BitOperatorPacket(t *testing.T) {
	input := "38006F45291200"
	bytes, err := hex.DecodeString(input)
	if err != nil {
		t.Error(err)
	}

	reader := BitReader{bytes, 0}
	packet := ReadPacket(&reader)
	if packet.Version != 1 {
		t.Errorf("Wrong Version of Packet %s", input)
	}
	if packet.TypeID != 6 {
		t.Errorf("Wrong Type ID of Packet %s", input)
	}
	if packet.Value != 0 {
		t.Errorf("Wrong Value of Packet %s", input)
	}
	if packet.Children == nil {
		t.Errorf("Packet %s sould have children", input)
	}
	if len(packet.Children) != 2 {
		t.Errorf("Packet %s sould have 2 children not %d", input, len(packet.Children))
	}
	if packet.Children[0].TypeID != 4 || packet.Children[0].Value != 10 {
		t.Errorf("First subpacket of Packet %s is wrong", input)
	}
	if packet.Children[1].TypeID != 4 || packet.Children[1].Value != 20 {
		t.Errorf("Second subpacket of Packet %s is wrong", input)
	}
}

func TestRead11BitOperatorPacket(t *testing.T) {
	input := "EE00D40C823060"
	bytes, err := hex.DecodeString(input)
	if err != nil {
		t.Error(err)
	}

	reader := BitReader{bytes, 0}
	packet := ReadPacket(&reader)
	if packet.Version != 7 {
		t.Errorf("Wrong Version of Packet %s", input)
	}
	if packet.TypeID != 3 {
		t.Errorf("Wrong Type ID of Packet %s", input)
	}
	if packet.Value != 0 {
		t.Errorf("Wrong Value of Packet %s", input)
	}
	if packet.Children == nil {
		t.Errorf("Packet %s sould have children", input)
	}
	if len(packet.Children) != 3 {
		t.Errorf("Packet %s sould have 3 children not %d", input, len(packet.Children))
	}
	if packet.Children[0].TypeID != 4 || packet.Children[0].Value != 1 {
		t.Errorf("First subpacket of Packet %s is wrong", input)
	}
	if packet.Children[1].TypeID != 4 || packet.Children[1].Value != 2 {
		t.Errorf("Second subpacket of Packet %s is wrong", input)
	}
	if packet.Children[2].TypeID != 4 || packet.Children[2].Value != 3 {
		t.Errorf("Third subpacket of Packet %s is wrong", input)
	}
}

func TestReadPacket1(t *testing.T) {
	input := "8A004A801A8002F478"
	bytes, err := hex.DecodeString(input)
	if err != nil {
		t.Error(err)
	}

	reader := BitReader{bytes, 0}
	packet := ReadPacket(&reader)

	version := 0
	for p := range packet.IterAll() {
		version += p.Version
	}
	if version != 16 {
		t.Errorf("Sum of versions of Packet %s is 16 not %d", input, version)
	}
}

func TestReadPacket2(t *testing.T) {
	// 01100010 00000000 10000000 00000000 00010110 00010001 01010110 00101100 10001000 00000010 00010001 10001110 00110100
	// vvvtttiL LLLLLLLL LL
	// 					   vvvttt iBBBBBBB BBBBBBBB+BBBBBBBB BBBBBBBB BBBBBB
	// 					   							vvvtttVV VVVvvvtt tVVVVV
	// 																		vv vtttiLLL LLLLLLLL
	// 																							 vvvtttVV VVV
	// 																							             vvvtt tVVVVV

	input := "620080001611562C8802118E34"
	bytes, err := hex.DecodeString(input)
	if err != nil {
		t.Error(err)
	}

	reader := BitReader{bytes, 0}
	packet := ReadPacket(&reader)

	version := 0
	for p := range packet.IterAll() {
		version += p.Version
	}
	if version != 12 {
		t.Errorf("Sum of versions of Packet %s is 12 not %d", input, version)
	}
}

// func TestReadPacket3(t *testing.T) {
// 	// 11000000 00000001 01010000 00000000 00000001 01100001 00010101 10100010 11100000 10000000 00101111 00011000 00100011 01000000
// 	input := "C0015000016115A2E0802F182340"
// 	bytes, err := hex.DecodeString(input)
// 	if err != nil {
// 		t.Error(err)
// 	}

// 	reader := BitReader{bytes, 0}
// 	packet := ReadPacket(&reader)

// 	version := 0
// 	for p := range packet.IterAll() {
// 		version += p.Version
// 	}
// 	if version != 23 {
// 		t.Errorf("Sum of versions of Packet %s is 23 not %d", input, version)
// 	}
// }
