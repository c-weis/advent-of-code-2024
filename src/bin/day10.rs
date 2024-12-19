use std::{collections::HashSet, hash::Hash};

use itertools::Itertools;
use rusty_advent_2024::utils;

#[derive(Debug)]
struct Topography {
    map: Vec<Vec<u8>>,
    bounds: Bounds,
}

#[derive(PartialEq, Eq, Hash)]
struct Position(i32, i32);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct ValidPosition(usize, usize);
#[derive(Debug)]
struct Bounds(usize, usize);

impl From<(usize, usize)> for ValidPosition {
    fn from((x, y): (usize, usize)) -> ValidPosition {
        ValidPosition(x, y)
    }
}

impl From<ValidPosition> for Position {
    fn from(ValidPosition(x, y): ValidPosition) -> Position {
        Position(x as i32, y as i32)
    }
}

impl Position {
    fn in_bounds(&self, bounds: &Bounds) -> Option<ValidPosition> {
        if self.0 >= 0 && self.1 >= 0 && self.0 < bounds.0 as i32 && self.1 < bounds.1 as i32 {
            Some(ValidPosition(self.0 as usize, self.1 as usize))
        } else {
            None
        }
    }
}

impl Topography {
    fn position_iter(&self) -> impl Iterator<Item = ValidPosition> {
        (0..self.bounds.0)
            .cartesian_product(0..self.bounds.1)
            .map_into()
    }

    fn value(&self, pos: &ValidPosition) -> u8 {
        self.map[pos.0 as usize][pos.1 as usize]
    }

    fn neighbours(&self, pos: Position) -> HashSet<ValidPosition> {
        [
            Position(pos.0 + 1, pos.1),
            Position(pos.0 - 1, pos.1),
            Position(pos.0, pos.1 + 1),
            Position(pos.0, pos.1 - 1),
        ]
        .into_iter()
        .filter_map(|neib| neib.in_bounds(&self.bounds))
        .collect()
    }

    fn find(&self, character: u8) -> HashSet<ValidPosition> {
        self.position_iter()
            .filter(|ValidPosition(x, y)| self.map[*x][*y] == character)
            .collect()
    }

    fn targets_reachable_by_trail(
        &self,
        start: ValidPosition,
        target_value: u8,
    ) -> HashSet<ValidPosition> {
        let start_value = self.value(&start);
        if start_value == target_value {
            return HashSet::from([start]);
        }

        self.neighbours(start.into())
            .iter()
            .filter(|&next_pos| -> bool {
                if target_value > start_value {
                    self.value(next_pos) == self.value(&start) + 1
                } else {
                    self.value(next_pos) == self.value(&start) - 1
                }
            })
            .map(|next_pos| -> HashSet<ValidPosition> {
                self.targets_reachable_by_trail(*next_pos, target_value)
            })
            .flatten()
            .collect()
    }

    fn trail_score(&self) -> usize {
        self.find(0)
            .iter()
            .map(|&zero| -> usize { self.targets_reachable_by_trail(zero, 9).len() })
            .sum()
    }

    fn partial_trail_rating(&self, start: ValidPosition, target_value: u8) -> usize {
        let start_value = self.value(&start);
        if start_value == target_value {
            return 1;
        }

        self.neighbours(start.into())
            .iter()
            .filter(|&next_pos| -> bool {
                if target_value > start_value {
                    self.value(next_pos) == self.value(&start) + 1
                } else {
                    self.value(next_pos) == self.value(&start) - 1
                }
            })
            .map(|next_pos| -> usize { self.partial_trail_rating(*next_pos, target_value) })
            .sum()
    }

    fn trail_rating(&self) -> usize {
        self.find(0)
            .iter()
            .map(|&zero| -> usize { self.partial_trail_rating(zero, 9) })
            .sum()
    }
}

fn topography_from_file(path: &str) -> Topography {
    let map = utils::lines_from_file(path)
        .map(|line| -> Vec<u8> {
            line.unwrap()
                .split("")
                .filter(|c| !c.is_empty())
                .map(|c| -> u8 { c.parse().expect("Error parsing height.") })
                .collect_vec()
        })
        .collect_vec();
    let bounds = Bounds(map[0].len(), map.len());

    Topography { map, bounds }
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input10.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input10.txt"));
}

fn part1(path: &str) -> usize {
    let topography = topography_from_file(path);
    topography.trail_score()
}

fn part2(path: &str) -> usize {
    let topography = topography_from_file(path);
    topography.trail_rating()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1("input/input10.txt.test1") == 36);
    }

    #[test]
    fn test_part2() {
        assert!(part2("input/input10.txt.test1") == 81);
    }
}
