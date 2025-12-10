use std::collections::HashSet;

use good_lp::{Solution, SolverModel, constraint, default_solver, variable};

struct Machine {
    buttons: Vec<Button>,
    target_joltage: Vec<i32>,
}

struct Button {
    toggled_lights: HashSet<i32>,
}

fn parse_numbers<'a>(string: &'a str, open: char, close: char) -> impl Iterator<Item = i32> + 'a {
    string
        .strip_prefix(open)
        .unwrap()
        .strip_suffix(close)
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
}

fn solve_matrix(rows: &[Vec<i32>], target: &[i32]) -> Option<Vec<i32>> {
    let m = rows.len();
    let d = target.len();

    let mut vars = good_lp::ProblemVariables::new();
    let xs: Vec<_> = (0..m)
        .map(|_| vars.add(variable().min(0).integer()))
        .collect();

    let solution = vars
        .minimise(xs.iter().sum::<good_lp::Expression>())
        .using(default_solver)
        .with_all((0..d).map(|j| {
            let lhs = xs
                .iter()
                .enumerate()
                .map(|(i, &x)| x * (rows[i][j]))
                .sum::<good_lp::Expression>();

            constraint!(lhs == target[j])
        }))
        .solve()
        .ok()?;

    Some(xs.iter().map(|&x| solution.value(x) as i32).collect())
}

pub fn solve(input: &str) -> String {
    let total_presses = input
        .lines()
        .map(|line| {
            let mut tokens = line.split(" ");

            tokens.next();

            let parts: Vec<_> = tokens.collect();

            let buttons = parts
                .iter()
                .take_while(|button| button.chars().nth(0).unwrap() == '(')
                .map(|button| {
                    let toggled_lights = parse_numbers(button, '(', ')').collect();

                    Button { toggled_lights }
                })
                .collect();

            let target_joltage = parse_numbers(parts.last().unwrap(), '{', '}').collect();

            Machine {
                buttons,
                target_joltage,
            }
        })
        .map(|machine| {
            let n = machine.target_joltage.len();
            let rows: Vec<_> = machine
                .buttons
                .iter()
                .map(|buttons| {
                    (0..n)
                        .map(|index| buttons.toggled_lights.contains(&(index as i32)) as i32)
                        .collect()
                })
                .collect();

            let solution = solve_matrix(&rows, &machine.target_joltage).unwrap();

            solution.iter().sum::<i32>()
        })
        .sum::<i32>();

    format!("Minimum total presses: {total_presses}")
}
