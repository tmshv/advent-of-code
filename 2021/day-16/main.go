package main

import (
	"bufio"
	"encoding/hex"
	"fmt"
	"log"
	"os"
)

func readInput() ([]byte, error) {
	scanner := bufio.NewScanner(os.Stdin)

	scanner.Scan()
	hexString := scanner.Text()
	bytes, err := hex.DecodeString(hexString)
	if err != nil {
		return nil, err
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return bytes, nil
}

func solvePartOne(bits []byte) uint64 {
	reader := BitReader{bits, 0}
	packet := ReadPacket(&reader)
	var version uint64
	for p := range packet.IterAll() {
		version += uint64(p.Version)
	}
	return version
}

func solvePartTwo(bits []byte) uint64 {
	reader := BitReader{bits, 0}
	packet := ReadPacket(&reader)
	return packet.Operate()
}

func main() {
	bits, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	var result uint64
	result = solvePartOne(bits)
	fmt.Printf("Part one: %v\n", result)

	result = solvePartTwo(bits)
	fmt.Printf("Part two: %v\n", result)
}
