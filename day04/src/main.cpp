#include "input.h"
#include "part1.h"
#include "part2.h"
#include <print>
#include <string>

int main() {
  InputData input = parseInput("");

  std::string part1Solution = solvePart1(input);
  std::string part2Solution = solvePart2(input);

  std::println("Part 1: {}", part1Solution);
  std::println("Part 2: {}", part2Solution);
}
