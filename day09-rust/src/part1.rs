use itertools::Itertools;

#[derive(Clone)]
struct Position {
    x: isize,
    y: isize,
}

pub fn solve(input: &str) -> String {
    let max_area = input
        .lines()
        .map(|line| {
            let mut coords_iter = line.split(",").map(|coord| coord.parse().unwrap());
            Position {
                x: coords_iter.next().unwrap(),
                y: coords_iter.next().unwrap(),
            }
        })
        .tuple_combinations()
        .map(|(a, b)| (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1))
        .max()
        .unwrap();

    format!("Max area: {max_area}")
}
