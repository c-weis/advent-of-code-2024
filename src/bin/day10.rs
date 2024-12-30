use itertools::Itertools;
use rusty_advent_2024::utils::file_io;
use rusty_advent_2024::utils::map2d::grid::{Grid, ValidPosition};
use std::collections::HashSet;
use std::ops::Deref;

type Height = u32;
struct Topography(Grid<Height>);

impl Deref for Topography {
    type Target = Grid<Height>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Topography {
    fn from_file(path: &str) -> Self {
        Topography(file_io::strings_from_file(path).collect_vec().into())
    }

    fn targets_reachable_by_trail(
        &self,
        start: ValidPosition,
        target_value: Height,
    ) -> HashSet<ValidPosition> {
        let start_value = *self.value(&start);
        if start_value == target_value {
            return HashSet::from([start]);
        }

        start
            .valid_neighbours(&self.bounds)
            .iter()
            .filter(|&next_pos| -> bool {
                if target_value > start_value {
                    *self.value(next_pos) == start_value + 1
                } else {
                    *self.value(next_pos) == start_value - 1
                }
            })
            .map(|next_pos| -> HashSet<ValidPosition> {
                self.targets_reachable_by_trail(*next_pos, target_value)
            })
            .flatten()
            .collect()
    }

    fn trail_score(&self) -> usize {
        self.find(&0)
            .iter()
            .map(|&zero| -> usize { self.targets_reachable_by_trail(zero, 9).len() })
            .sum()
    }

    fn partial_trail_rating(&self, start: ValidPosition, target_value: Height) -> usize {
        let start_value = *self.value(&start);
        if start_value == target_value {
            return 1;
        }

        start
            .valid_neighbours(&self.bounds)
            .iter()
            .filter(|&next_pos| -> bool {
                if target_value > start_value {
                    *self.value(next_pos) == start_value + 1
                } else {
                    *self.value(next_pos) == start_value - 1
                }
            })
            .map(|next_pos| -> usize { self.partial_trail_rating(*next_pos, target_value) })
            .sum()
    }

    fn trail_rating(&self) -> usize {
        self.find(&0)
            .iter()
            .map(|&zero| -> usize { self.partial_trail_rating(zero, 9) })
            .sum()
    }
}

fn part1(path: &str) -> usize {
    Topography::from_file(path).trail_score()
}

fn part2(path: &str) -> usize {
    Topography::from_file(path).trail_rating()
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input10.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input10.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/input10.txt.test1"), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/input10.txt.test1"), 81);
    }
}
