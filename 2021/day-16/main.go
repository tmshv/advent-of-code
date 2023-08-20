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

func solvePartOne(bits []byte) int {
	PrintBytes(bits)
	reader := BitReader{bits, 0}
	packet := ReadPacket(&reader)
	log.Printf("Got Packet: %v (%v)", packet, reader.Done())

	version := 0
	for p := range packet.IterAll() {
		log.Println("Interating packet", p, version)
		version += p.Version
	}
	return version
}

func solvePartTwo(bits []byte) int {
	return 0
}

func main() {
	bits, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	var result int
	result = solvePartOne(bits)
	fmt.Printf("Part one: %v\n", result)

	result = solvePartTwo(bits)
	fmt.Printf("Part two: %v\n", result)
}
