use itertools::Itertools;
use num::Integer;
use regex::{Captures, Regex};
use rusty_advent_2024::utils::{file_io, math2d::IntVec2D};
use std::cmp::min;

type Coordinate = i128;

#[derive(Debug)]
struct ClawMachine {
    a: IntVec2D<Coordinate>,
    b: IntVec2D<Coordinate>,
    prize: IntVec2D<Coordinate>,
}

trait IntoTuple<T> {
    fn into_tuple(self) -> (T, T);
}

impl IntoTuple<Coordinate> for Captures<'_> {
    fn into_tuple(self) -> (Coordinate, Coordinate) {
        (
            self.get(1)
                .expect("Did not match first group.")
                .as_str()
                .parse()
                .expect("Could not parse group 1."),
            self.get(2)
                .expect("Did not match second group.")
                .as_str()
                .parse()
                .expect("Could not parse group 2."),
        )
    }
}

impl From<&str> for ClawMachine {
    fn from(data_string: &str) -> Self {
        let button_a_pattern: Regex = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
        let button_b_pattern: Regex = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
        let prize_pattern: Regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

        let button_a_match = button_a_pattern
            .captures(data_string)
            .expect("Button A data not found.");
        let button_b_match = button_b_pattern
            .captures(data_string)
            .expect("Button B data not found.");
        let prize_match = prize_pattern
            .captures(data_string)
            .expect("Prize data not found.");

        let button_a_data: (Coordinate, Coordinate) = button_a_match.into_tuple();
        let button_b_data: (Coordinate, Coordinate) = button_b_match.into_tuple();
        let prize_data: (Coordinate, Coordinate) = prize_match.into_tuple();

        ClawMachine {
            a: IntVec2D::from(button_a_data),
            b: IntVec2D::from(button_b_data),
            prize: IntVec2D::from(prize_data),
        }
    }
}

fn cost<T: Integer + From<i32>>(press_a: T, press_b: T) -> T {
    press_a * 3.into() + press_b
}

impl ClawMachine {
    fn cheapest_win(&self) -> Option<Coordinate> {
        let IntVec2D(a_0, a_1) = self.a;
        let IntVec2D(b_0, b_1) = self.b;
        let a_orth = IntVec2D(-a_1, a_0);
        let b_orth = IntVec2D(-b_1, b_0);

        let determinant = b_orth.dot(self.a);
        if determinant != 0 {
            // a & b are not parallel: the solution is unique if it exists
            let numerator = IntVec2D(b_orth.dot(self.prize), -a_orth.dot(self.prize));

            if numerator.0 % determinant == 0 && numerator.1 % determinant == 0 {
                let presses = numerator / determinant;
                if presses.0 >= 0 && presses.1 >= 0 {
                    return Some(cost(presses.0, presses.1));
                }
            }

            None
        } else {
            // thankfully not needed for my inputs :D
            todo!()
        }
    }

    fn cheapest_win_easy(&self) -> Option<Coordinate> {
        let IntVec2D(a_0, a_1) = self.a;
        let IntVec2D(b_0, b_1) = self.b;
        let IntVec2D(p_0, p_1) = self.prize;

        let gcd_0 = a_0.extended_gcd(&b_0);
        let gcd_1 = a_1.extended_gcd(&b_1);

        if p_0 % gcd_0.gcd != 0 || p_1 % gcd_1.gcd != 0 {
            return None;
        }

        let max_a = min(min(p_0 / a_0, p_1 / a_1), 100);

        (0..=max_a)
            .filter_map(|a_presses| -> Option<Coordinate> {
                let remainder = self.prize - self.a * a_presses;
                if remainder.0 % b_0 == 0
                    && remainder.1 % b_1 == 0
                    && remainder.0 / b_0 == remainder.1 / b_1
                {
                    Some(cost(a_presses, remainder.0 / b_0))
                } else {
                    None
                }
            })
            .min()
    }
}

fn claw_machines_from_file(path: &str) -> Vec<ClawMachine> {
    let lines = file_io::lines_from_file(path).map(|line| line.unwrap());
    lines
        .chunks(4)
        .into_iter()
        .map(|mut paragraph| -> String { paragraph.join(" ") })
        .map(|data_string| ClawMachine::from(data_string.as_str()))
        .collect()
}

fn part1(path: &str) -> Coordinate {
    let machines = claw_machines_from_file(path);
    machines
        .iter()
        .filter_map(|machine| machine.cheapest_win_easy())
        .sum()
}

fn part2(path: &str) -> Coordinate {
    let mut machines = claw_machines_from_file(path);
    machines.iter_mut().for_each(|machine| {
        machine.prize = machine.prize + IntVec2D(10000000000000, 10000000000000)
    });

    machines
        .iter()
        .filter_map(|machine| machine.cheapest_win())
        .sum()
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input13.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input13.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1("input/input13.txt.test1") == 480);
    }
}
