use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;
use rusty_advent_2024::{maps::IntVec2D, utils};

type Number = i32;

#[derive(Debug)]
struct Robot {
    pos: IntVec2D<Number>,
    vel: IntVec2D<Number>,
}

struct Torus(Number, Number);

#[derive(PartialEq, Eq, Hash)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Robot {
    fn move_on_torus(&mut self, seconds: Number, torus: &Torus) {
        self.pos = self.pos + self.vel * seconds;
        self.pos.0 = (self.pos.0 % torus.0 + torus.0) % torus.0;
        self.pos.1 = (self.pos.1 % torus.1 + torus.1) % torus.1;
    }
}

fn torus_print(robots: &Vec<Robot>, torus: &Torus) {
    let mut multiplicity: HashMap<IntVec2D<Number>, usize> = HashMap::new();
    for robot in robots {
        *multiplicity.entry(robot.pos).or_insert(0) += 1;
    }

    print!(
        "{}",
        (0..torus.1)
            .map(|y| -> String {
                (0..torus.0)
                    .map(|x| -> String {
                        multiplicity
                            .get(&IntVec2D(x, y))
                            .map_or(String::from("."), |num| num.to_string())
                    })
                    .join("")
            })
            .join("\n")
    );
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
}

fn robots_from_file(path: &str) -> Vec<Robot> {
    let pattern =
        Regex::new(r"p=(.*?),(.*?) v=(.*?),(.*?)$").expect("Creation of regex pattern failed.");

    let lines = utils::lines_from_file(path).map(|line| line.unwrap());

    lines
        .map(|line| -> Robot {
            let captures = pattern
                .captures(line.as_str())
                .expect("Robot data could not be detected.");
            let integer_data: [Number; 4] = captures
                .extract()
                .1
                .map(|capture| -> Number { capture.parse().expect("Could not parse integer.") });

            Robot {
                pos: IntVec2D(integer_data[0], integer_data[1]),
                vel: IntVec2D(integer_data[2], integer_data[3]),
            }
        })
        .collect()
}

fn safety_factor(robots: Vec<Robot>, torus: &Torus) -> Number {
    let mut robots_per_quadrant: HashMap<Quadrant, Number> = HashMap::new();

    for robot in robots {
        let IntVec2D(x, y) = robot.pos;
        if x < torus.0 / 2 {
            if y < torus.1 / 2 {
                *robots_per_quadrant.entry(Quadrant::TopLeft).or_insert(0) += 1;
            } else if y > torus.1 / 2 {
                *robots_per_quadrant.entry(Quadrant::BottomLeft).or_insert(0) += 1;
            }
        } else if x > torus.0 / 2 {
            if y < torus.1 / 2 {
                *robots_per_quadrant.entry(Quadrant::TopRight).or_insert(0) += 1;
            } else if y > torus.1 / 2 {
                *robots_per_quadrant
                    .entry(Quadrant::BottomRight)
                    .or_insert(0) += 1;
            }
        }
    }

    robots_per_quadrant.values().product()
}

fn advance_pack(robots: &mut [Robot], seconds: Number, torus: &Torus) {
    for robot in robots {
        robot.move_on_torus(seconds, &torus);
    }
}

fn part1(path: &str, torus: Torus) -> Number {
    let mut robots = robots_from_file(path);
    advance_pack(&mut robots, 100, &torus);
    safety_factor(robots, &torus)
}

fn part2(path: &str, torus: Torus) -> String {
    let mut robots = robots_from_file(path);
    let not_the_answer = 6900;
    advance_pack(&mut robots, not_the_answer, &torus);
    for i in 1..=200 {
        println!("{}:", i + 6900);
        advance_pack(&mut robots, 1, &torus);
        torus_print(&robots, &torus);
    }

    String::from("Look for the ||s and =s")
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input14.txt", Torus(101, 103)));
    println!("Good luck with part 2!");
    println!("{}", part2("input/input14.txt", Torus(101, 103)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1("input/input14.txt.test1", Torus(11, 7)) == 12);
    }
}
