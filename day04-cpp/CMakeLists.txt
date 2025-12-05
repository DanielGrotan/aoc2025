cmake_minimum_required(VERSION 3.16)
project(aoc_day04 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(aoc-day04
  src/main.cpp
  src/input.cpp
  src/part1.cpp
  src/part2.cpp
)

target_include_directories(aoc-day04 PRIVATE include)

if (CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
  target_compile_options(aoc-day04 PRIVATE -stdlib=libc++)
  target_link_options(aoc-day04 PRIVATE -stdlib=libc++)
endif()
