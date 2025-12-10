use std::{collections::HashSet, rc::Rc};

use itertools::Itertools;

struct Machine {
    target_lights: HashSet<usize>,
    buttons: Rc<[Button]>,
}

#[derive(Debug)]
struct Button {
    toggled_lights: HashSet<usize>,
}

impl Button {
    pub fn blank() -> Self {
        Self {
            toggled_lights: Default::default(),
        }
    }

    pub fn xor(&mut self, other: &Self) {
        other.toggled_lights.iter().for_each(|&light| {
            if !self.toggled_lights.insert(light) {
                self.toggled_lights.remove(&light);
            }
        });
    }
}

pub fn solve(input: &str) -> String {
    let total_presses = input
        .lines()
        .map(|line| {
            let mut split_iter = line.split(" ");

            let target_lights = split_iter
                .next()
                .unwrap()
                .strip_prefix("[")
                .unwrap()
                .strip_suffix("]")
                .unwrap()
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(index, _)| index)
                .collect();

            let buttons = split_iter
                .take_while(|button| button.chars().nth(0).unwrap() == '(')
                .map(|button| {
                    let toggled_lights = button
                        .strip_prefix("(")
                        .unwrap()
                        .strip_suffix(")")
                        .unwrap()
                        .split(",")
                        .map(|number| number.parse().unwrap())
                        .collect();

                    Button { toggled_lights }
                })
                .collect();

            Machine {
                target_lights,
                buttons,
            }
        })
        .map(|machine| {
            let buttons = machine
                .buttons
                .iter()
                .powerset()
                .find(|buttons| {
                    let mut result = Button::blank();

                    buttons.iter().for_each(|button| {
                        result.xor(button);
                    });

                    result.toggled_lights == machine.target_lights
                })
                .unwrap();

            buttons.len()
        })
        .sum::<usize>();

    format!("Minimum total presses: {total_presses}")
}
