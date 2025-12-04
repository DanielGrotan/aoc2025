#pragma once
#include <cstddef>
#include <fstream>
#include <vector>

struct InputData {
  size_t rows;
  size_t cols;
  std::vector<bool> grid;

  bool getCell(size_t row, size_t col) const;
  void removeCell(size_t row, size_t col);
};

InputData parseInput(std::ifstream &file);
