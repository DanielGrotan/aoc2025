mod part1;
mod part2;

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part1::solve(input));
    println!("{}", part2::solve(input));
}
