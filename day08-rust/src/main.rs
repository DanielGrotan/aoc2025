use std::fs;

mod part1;
mod part2;

fn main() {
    let input = fs::read_to_string("test.txt").expect("Failed to read input");

    println!("Part 1: {}", part1::solve(&input));
    println!("Part 2: {}", part2::solve(&input));
}
