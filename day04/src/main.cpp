#include "input.h"
#include "part1.h"
#include "part2.h"
#include <fstream>
#include <print>

int main() {
  std::ifstream file("input.txt");
  InputData input = parseInput(file);

  std::println("Part 1: {}", solvePart1(input));
  std::println("Part 2: {}", solvePart2(input));
}
