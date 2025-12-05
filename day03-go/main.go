package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	data, err := os.ReadFile("input.txt")
	if err != nil {
		return
	}
	trimmed := strings.TrimSpace(string(data))

	fmt.Println(Part1(trimmed))
	fmt.Println(Part2(trimmed))
}
