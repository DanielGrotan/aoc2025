#include "input.h"
#include <string>
#include <vector>

bool InputData::getCell(size_t row, size_t col) const {
  if (row < 0 || row >= rows || col < 0 || col >= cols) {
    return false;
  }

  size_t index = col + row * cols;
  return grid[index];
}

InputData parseInput(std::ifstream &file) {
  std::vector<bool> grid;

  std::string line;
  size_t rows = 0;
  size_t cols = 0;

  while (std::getline(file, line)) {
    cols = line.length();
    rows++;

    for (auto c : line) {
      grid.push_back(c == '@');
    }
  }

  InputData inputData = {rows, cols, grid};
  return inputData;
}
