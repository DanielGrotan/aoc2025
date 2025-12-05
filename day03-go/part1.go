package main

import (
	"fmt"
	"strings"
)

func processLine(line string) int {
	first := 0
	second := 0

	for i, char := range line {
		digit := int(char - '0')

		if digit > first && i < len(line)-1 {
			first = digit
			second = 0
			continue
		}

		if digit > second {
			second = digit
		}
	}

	return first*10 + second
}

func Part1(input string) string {
	lines := strings.Split(input, "\n")
	total := 0

	for _, line := range lines {
		total += processLine(line)
	}

	return fmt.Sprintf("Part 1: %d", total)
}
