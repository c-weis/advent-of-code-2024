use itertools::{Either, Itertools};
use rusty_advent_2024::utils::file_io;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

const PINS: usize = 5;
const LOCK_HEIGHT: u8 = 5;
type PinSet = [u8; PINS];
type Lock = PinSet;
type Key = PinSet;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Pin {
    index: usize,
    height: u8,
}

impl Pin {
    fn new() -> Self {
        Self {
            index: 0,
            height: 0,
        }
    }

    fn fitting_opposites(self) -> Vec<Self> {
        (0..=LOCK_HEIGHT - self.height)
            .map(|complementary_height| Pin {
                index: self.index,
                height: complementary_height,
            })
            .collect()
    }
}

fn pins(pinset: &PinSet) -> [Pin; PINS] {
    let mut pins = [Pin::new(); PINS];
    for pin_idx in 0..PINS {
        pins[pin_idx] = Pin {
            index: pin_idx,
            height: pinset[pin_idx],
        };
    }
    pins
}

#[derive(Debug)]
struct LockSmith {
    locks: Vec<Lock>,
    keys: Vec<Key>,

    locks_with_pin: HashMap<Pin, HashSet<Lock>>,
    locks_that_fit_pin: HashMap<Pin, HashSet<Lock>>,
}

impl LockSmith {
    fn from_file(path: &str) -> Self {
        let (locks, keys) = file_io::strings_from_file(path)
            .chunk_by(|line| line.is_empty())
            .into_iter()
            .filter_map(|(is_empty, chunk)| {
                if is_empty {
                    None
                } else {
                    Some(chunk.collect_vec())
                }
            })
            .partition_map(|block| {
                if LockSmith::is_lock(&block) {
                    Either::Left(LockSmith::get_counts(&block))
                } else {
                    Either::Right(LockSmith::get_counts(&block))
                }
            });

        LockSmith::new(locks, keys)
    }

    fn new(locks: Vec<Lock>, keys: Vec<Key>) -> Self {
        let mut new = LockSmith {
            locks,
            keys,
            locks_with_pin: HashMap::new(),
            locks_that_fit_pin: HashMap::new(),
        };
        new.cache_locks();

        new
    }

    fn cache_locks(&mut self) {
        for lock in &self.locks {
            for pin in pins(lock) {
                self.locks_with_pin
                    .entry(pin)
                    .or_insert(HashSet::new())
                    .insert(*lock);
                for opposite_pin in pin.fitting_opposites() {
                    self.locks_that_fit_pin
                        .entry(opposite_pin)
                        .or_insert(HashSet::new())
                        .insert(*lock);
                }
            }
        }
    }

    fn is_lock(block: &[String]) -> bool {
        match block.first().unwrap().as_str() {
            "#####" => true,
            "....." => false,
            _ => panic!("Each block should start with an empty or a full line."),
        }
    }

    fn get_counts(block: &[String]) -> PinSet {
        let mut counts = [0; PINS];

        // ignore first and last line of each block
        for line in &block[1..block.len() - 1] {
            for (column, c) in line.char_indices() {
                if c == '#' {
                    counts[column] += 1;
                }
            }
        }

        counts
    }

    fn matching_locks(&self, key: &Key) -> usize {
        let mut sorted_lock_sets = pins(key)
            .iter()
            .map(|pin| self.locks_that_fit_pin.get(&pin))
            .sorted_by_key(|opt_set| -> usize { opt_set.map_or(0, |set| set.len()) });

        let mut fitting_locks: HashSet<Lock> = sorted_lock_sets
            .by_ref()
            .next()
            .unwrap()
            .map_or(HashSet::new(), |set| set.clone());

        for lock_set in sorted_lock_sets {
            if let Some(lock_set) = lock_set {
                fitting_locks.retain(|lock| lock_set.contains(lock));
            }
        }

        fitting_locks.len()
    }

    fn fitting_combinations(&mut self) -> usize {
        self.keys.iter().map(|key| self.matching_locks(key)).sum()
    }
}

fn part1(path: &str) -> usize {
    let mut locksmith = LockSmith::from_file(path);

    locksmith.fitting_combinations()
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input25.txt"));
    println!("Answer to part 2:");
    println!("{}", "Deliver the chronicle!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/input25.txt.test1"), 3);
    }
}
