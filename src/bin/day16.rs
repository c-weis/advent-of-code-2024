use std::{
    cmp::{Ordering, Reverse},
    collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet},
};

use itertools::Itertools;
use rusty_advent_2024::utils::{
    file_io,
    map2d::{
        direction::Direction,
        grid::{Convert, Grid, ValidPosition},
    },
};

#[derive(Debug, Eq, PartialEq)]
enum Field {
    Empty,
    Wall,
}

impl From<char> for Field {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '.' | 'S' | 'E' => Self::Empty,
            _ => panic!("Invalid character for maze field."),
        }
    }
}

#[derive(Debug)]
struct Maze {
    field: Grid<Field>,
    start: ValidPosition,
    end: ValidPosition,
}

#[derive(Debug)]
struct Reindeer {
    pos: ValidPosition,
    dir: Direction,
    score: usize,
    past: HashSet<ValidPosition>,
}

impl PartialEq for Reindeer {
    fn eq(&self, other: &Self) -> bool {
        self.score.eq(&other.score)
    }
}

impl Eq for Reindeer {}

impl PartialOrd for Reindeer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for Reindeer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl Maze {
    fn next_steps(&self, reindeer: Reindeer) -> Vec<Reindeer> {
        let mut reindeers = vec![
            Reindeer {
                pos: reindeer.pos,
                dir: reindeer.dir.turned_right(),
                score: reindeer.score + 1000, // make it A* by adding heuristic?
                past: reindeer.past.clone(),
            },
            Reindeer {
                pos: reindeer.pos,
                dir: reindeer.dir.turned_left(),
                score: reindeer.score + 1000, // make it A* by adding heuristic?
                past: reindeer.past.clone(),
            },
        ];
        if let Some(pos) = reindeer.pos.try_step(&reindeer.dir, &self.field.bounds) {
            if self.field.value(&pos) == &Field::Empty {
                let mut new_past = reindeer.past.clone();
                new_past.insert(pos);
                reindeers.push(Reindeer {
                    pos,
                    dir: reindeer.dir,
                    score: reindeer.score + 1,
                    past: new_past,
                });
            }
        }
        reindeers
    }

    fn score_and_best_seats(&self) -> (usize, usize) {
        let mut reindeers: BinaryHeap<Reverse<Reindeer>> = BinaryHeap::new();
        let mut min_score_map: HashMap<(ValidPosition, Direction), usize> = HashMap::new();

        let mut min_total: Option<usize> = None;
        let mut best_seats: HashSet<ValidPosition> = HashSet::new();

        reindeers.push(Reverse(Reindeer {
            pos: self.start,
            dir: Direction::RIGHT,
            score: 0, // TODO: make it A* by adding heuristic?
            past: HashSet::from([self.start]),
        }));

        while let Some(Reverse(reindeer)) = reindeers.pop() {
            // 1. check if we found the end - if its a 'best' case, store its past
            if reindeer.pos == self.end {
                if let Some(min_total) = min_total {
                    if min_total < reindeer.score {
                        break;
                    }
                } else {
                    min_total = Some(reindeer.score);
                }
                best_seats.extend(reindeer.past.iter().by_ref());
            }

            // 2. check in minimal score hashmap
            match min_score_map.entry((reindeer.pos, reindeer.dir)) {
                Entry::Occupied(mut min_score_entry) => {
                    if *min_score_entry.get() < reindeer.score {
                        continue;
                    }
                    min_score_entry.insert(reindeer.score);
                }
                Entry::Vacant(empty_entry) => {
                    empty_entry.insert(reindeer.score);
                }
            }

            for next_reindeer in self.next_steps(reindeer) {
                reindeers.push(Reverse(next_reindeer));
            }
        }

        if let Some(min_total) = min_total {
            (min_total, best_seats.len())
        } else {
            panic!("No path found!");
        }
    }
}

fn load_maze(path: &str) -> Maze {
    let char_grid: Grid<char> = file_io::strings_from_file(path).collect_vec().into();
    let start = *char_grid
        .find(&'S')
        .iter()
        .exactly_one()
        .expect("There should be exactly one S in the input.");
    let end = *char_grid
        .find(&'E')
        .iter()
        .exactly_one()
        .expect("There should be exactly one E in the input.");
    Maze {
        field: char_grid.convert(),
        start,
        end,
    }
}

fn part1(path: &str) -> usize {
    let maze = load_maze(path);
    maze.score_and_best_seats().0
}

fn part2(path: &str) -> usize {
    let maze = load_maze(path);
    maze.score_and_best_seats().1
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input16.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input16.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1("input/input16.txt.test1") == 7036);
        assert!(part1("input/input16.txt.test2") == 11048);
    }

    #[test]
    fn test_part2() {
        assert!(part2("input/input16.txt.test1") == 45);
        assert!(part2("input/input16.txt.test2") == 64);
    }
}
