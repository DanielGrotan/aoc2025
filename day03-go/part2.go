package main

import (
	"fmt"
	"strings"
)

func processLine2(line string) int {
	largest := [12]int{}

	for i, char := range line {
		digit := int(char - '0')

		remaining := min(12, len(line)-i)
		start := 12 - remaining

		for pos := start; pos < 12; pos++ {
			if digit > largest[pos] {
				largest[pos] = digit
				for j := pos + 1; j < 12; j++ {
					largest[j] = 0
				}
				break
			}
		}
	}

	total := 0
	for _, digit := range largest {
		total = total*10 + digit
	}

	return total
}

func Part2(input string) string {
	lines := strings.Split(input, "\n")
	total := 0

	for _, line := range lines {
		total += processLine2(line)
	}

	return fmt.Sprintf("Part 2: %d", total)
}
