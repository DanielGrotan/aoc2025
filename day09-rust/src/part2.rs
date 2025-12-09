use itertools::Itertools;

#[derive(Clone)]
struct Position {
    x: isize,
    y: isize,
}

pub fn solve(input: &str) -> String {
    let red_tiles: Vec<_> = input
        .lines()
        .map(|line| {
            let mut coords_iter = line.split(",").map(|coord| coord.parse().unwrap());
            Position {
                x: coords_iter.next().unwrap(),
                y: coords_iter.next().unwrap(),
            }
        })
        .collect();
    let lines: Vec<_> = red_tiles.iter().circular_tuple_windows().collect();

    let max_box = red_tiles
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let area = (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1);
            (a, b, area)
        })
        .sorted_by_key(|v| v.2)
        .rev()
        .find(|(a, b, _area)| {
            lines.iter().all(|(line_start, line_end)| {
                let left_of_rect = a.x.max(b.x) <= line_start.x.min(line_end.x);
                let right_of_rect = a.x.min(b.x) >= line_start.x.max(line_end.x);
                let above = a.y.max(b.y) <= line_start.y.min(line_end.y);
                let below = a.y.min(b.y) >= line_start.y.max(line_end.y);
                left_of_rect || right_of_rect || above || below
            })
        });
    let max_area = max_box.unwrap().2;

    format!("Max area: {max_area}")
}
