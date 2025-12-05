#include "part2.h"
#include "input.h"
#include <string>

static size_t neighbours(const InputData &input, size_t row, size_t col) {
  size_t count = 0;

  size_t startRow = row == 0 ? 0 : row - 1;
  size_t startCol = col == 0 ? 0 : col - 1;
  for (int i = startRow; i <= row + 1; i++) {
    for (int j = startCol; j <= col + 1; j++) {
      if (i == row && j == col) {
        continue;
      }

      count += input.getCell(i, j);
    }
  }

  return count;
}

std::string solvePart2(InputData &input) {
  size_t canAccess = 0;

  bool changed = true;
  while (changed) {
    changed = false;

    for (size_t i = 0; i < input.rows; i++) {
      for (size_t j = 0; j < input.cols; j++) {
        if (!input.getCell(i, j)) {
          continue;
        }

        size_t neighbourCount = neighbours(input, i, j);

        if (neighbourCount < 4) {
          canAccess++;
          input.removeCell(i, j);
          changed = true;
        }
      }
    }
  }

  return std::format("{}", canAccess);
}
