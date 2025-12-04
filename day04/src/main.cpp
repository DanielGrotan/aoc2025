#include "input.h"
#include "part1.h"
#include "part2.h"
#include <fstream>
#include <print>
#include <string>

int main() {
  std::ifstream file("input.txt");
  InputData input = parseInput(file);
  file.close();

  std::string part1Solution = solvePart1(input);
  std::string part2Solution = solvePart2(input);

  std::println("Part 1: {}", part1Solution);
  std::println("Part 2: {}", part2Solution);
}
