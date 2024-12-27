use std::{
    cmp::{Ordering, Reverse},
    collections::{hash_map::Entry, BinaryHeap, HashMap},
};

use itertools::Itertools;
use num::abs;
use rusty_advent_2024::utils::{
    file_io,
    map2d::grid::{Bounds, Grid, ValidPosition},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Memory {
    Working,
    Corrupted,
}

#[derive(Debug)]
struct MemorySpace {
    field: Grid<Memory>,
    start: ValidPosition,
    end: ValidPosition,
}

#[derive(Debug)]
struct Runner {
    pos: ValidPosition,
    time_elapsed: usize,
    time_expected: usize,
}

impl Runner {
    fn score(&self) -> usize {
        self.time_elapsed + self.time_expected
    }
}

impl PartialEq for Runner {
    fn eq(&self, other: &Self) -> bool {
        self.score().eq(&other.score())
    }
}

impl Eq for Runner {}

impl PartialOrd for Runner {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score().partial_cmp(&other.score())
    }
}

impl Ord for Runner {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score().cmp(&other.score())
    }
}

impl MemorySpace {
    fn new(width: usize, height: usize) -> Self {
        let field = Grid::new(Bounds(width, height), Memory::Working);
        MemorySpace {
            field,
            start: ValidPosition(0, 0),
            end: ValidPosition(width - 1, height - 1),
        }
    }

    fn corrupt(&mut self, pos: &ValidPosition) {
        *self.field.value_mut(pos) = Memory::Corrupted;
    }

    fn heuristic(&self, pos: ValidPosition) -> usize {
        (abs(pos.0 as isize - self.end.0 as isize) + abs(pos.1 as isize - self.end.1 as isize))
            as usize
    }

    fn next_steps(&self, runner: Runner) -> Vec<Runner> {
        runner
            .pos
            .valid_neighbours(&self.field.bounds)
            .iter()
            .filter_map(|&pos| match self.field.value(&pos) {
                Memory::Working => Some(Runner {
                    pos: pos.clone(),
                    time_elapsed: runner.time_elapsed + 1,
                    time_expected: self.heuristic(pos),
                }),
                _ => None,
            })
            .collect()
    }

    fn shortest_path(&self) -> Option<usize> {
        let mut runners: BinaryHeap<Reverse<Runner>> = BinaryHeap::new();
        let mut fastest_arrival_map: HashMap<ValidPosition, usize> = HashMap::new();

        runners.push(Reverse(Runner {
            pos: self.start,
            time_elapsed: 0,
            time_expected: self.heuristic(self.start),
        }));

        while let Some(Reverse(runner)) = runners.pop() {
            //dbg!(&runner);
            if runner.pos == self.end {
                return Some(runner.time_elapsed);
            }

            // 2. check in minimal score hashmap
            match fastest_arrival_map.entry(runner.pos) {
                Entry::Occupied(mut min_time_entry) => {
                    if *min_time_entry.get() <= runner.time_elapsed {
                        continue;
                    }
                    min_time_entry.insert(runner.time_elapsed);
                }
                Entry::Vacant(empty_entry) => {
                    empty_entry.insert(runner.time_elapsed);
                }
            }

            for next_runner in self.next_steps(runner) {
                runners.push(Reverse(next_runner));
            }
        }

        None
    }

    fn bulk_corrupt(&mut self, corruptions: &[(usize, usize)]) {
        for cor in corruptions {
            self.corrupt(&ValidPosition(cor.0, cor.1));
        }
    }
}

fn find_blocking_byte((width, height): (usize, usize), corruptions: &[(usize, usize)]) -> usize {
    let mut left = 0;
    let mut right = corruptions.len() - 1;

    while left < right {
        let mid = (left + right) / 2;
        let mut memory = MemorySpace::new(width, height);
        memory.bulk_corrupt(&corruptions[0..=mid]);

        if memory.shortest_path().is_some() {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    right
}

fn load_corruptions(path: &str) -> Vec<(usize, usize)> {
    file_io::strings_from_file(path)
        .map(|s| -> (usize, usize) {
            s.split(",")
                .map(|num| num.parse().expect("Number values should be parsable."))
                .collect_tuple()
                .expect("Each line should contain a pair of comma-separated numbers.")
        })
        .collect_vec()
}

fn part1(path: &str, (width, height): (usize, usize), fallen_bytes: usize) -> usize {
    let mut memory = MemorySpace::new(width, height);
    let corruptions = load_corruptions(path);
    memory.bulk_corrupt(&corruptions[0..fallen_bytes]);
    memory.shortest_path().expect("No shortest path found!")
}

fn part2(path: &str, (width, height): (usize, usize)) -> (usize, usize) {
    let corruptions = load_corruptions(path);
    let byte_idx = find_blocking_byte((width, height), &corruptions);
    corruptions[byte_idx]
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input18.txt", (71, 71), 1024));
    println!("Answer to part 2:");
    println!("{:?}", part2("input/input18.txt", (71, 71)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/input18.txt.test1", (7, 7), 12), 22);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/input18.txt.test1", (7, 7)), (6, 1));
    }
}
