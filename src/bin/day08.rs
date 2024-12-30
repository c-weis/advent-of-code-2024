use itertools::Itertools;
use rusty_advent_2024::utils::{
    file_io,
    map2d::{
        grid::{Bounds, Grid, ValidPosition},
        position::Position,
    },
};
use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
};

struct Antenna {
    frequency: char,
    pos: Position,
}

struct AntennaMap(HashMap<char, HashSet<Position>>);

// implemented bc I want AntennaMap to *be* a HashMap
impl Deref for AntennaMap {
    type Target = HashMap<char, HashSet<Position>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// implemented bc I want AntennaMap to *be* a HashMap
impl DerefMut for AntennaMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AntennaMap {
    fn add(&mut self, antenna: Antenna) {
        if let Some(positions) = self.get_mut(&antenna.frequency) {
            positions.insert(antenna.pos);
        } else {
            self.insert(antenna.frequency, HashSet::from([antenna.pos]));
        }
    }

    fn new() -> Self {
        AntennaMap(HashMap::new())
    }
}

struct City {
    bounds: Bounds,
    antenna_map: AntennaMap,
}

impl City {
    fn basic_antinodes(self) -> HashSet<ValidPosition> {
        let mut antinodes: HashSet<ValidPosition> = HashSet::new();

        for position_list in self.antenna_map.values() {
            for pos1 in position_list {
                for pos2 in position_list {
                    if pos1 == pos2 {
                        continue;
                    }

                    let antinode = pos1.mirrored_across(pos2);
                    if let Some(pos) = antinode.in_bounds(&self.bounds) {
                        antinodes.insert(pos);
                    }
                }
            }
        }

        antinodes
    }

    fn harmonic_antinodes(self) -> HashSet<ValidPosition> {
        let mut antinodes: HashSet<ValidPosition> = HashSet::new();

        for position_list in self.antenna_map.values() {
            let position_iter = position_list.iter();
            for (pos1, pos2) in position_iter.clone().cartesian_product(position_iter) {
                if pos1 == pos2 {
                    continue;
                }

                let distance = *pos2 - *pos1;
                let gcd = gcd(distance.0.abs() as usize, distance.1.abs() as usize) as i32;
                let delta = distance / gcd;

                let mut antinode = pos1.clone();
                while let Some(pos) = antinode.in_bounds(&self.bounds) {
                    antinodes.insert(pos.clone());
                    antinode = antinode + delta;
                }
            }
        }

        antinodes
    }
}

impl From<Grid<char>> for City {
    fn from(map: Grid<char>) -> Self {
        let mut antenna_map = AntennaMap::new();
        for pos in map.position_iter() {
            match map.value(&pos) {
                '.' => (),
                c => antenna_map.add(Antenna {
                    frequency: *c,
                    pos: pos.into(),
                }),
            };
        }

        City {
            bounds: map.bounds,
            antenna_map,
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    match (a, b) {
        (x, 0) | (0, x) => x,
        _ => gcd(b, a % b),
    }
}

fn scan_city(path: &str) -> City {
    let map: Grid<char> = file_io::strings_from_file(path).collect_vec().into();
    City::from(map)
}

fn part1(path: &str) -> usize {
    let city = scan_city(path);
    city.basic_antinodes().len()
}

fn part2(path: &str) -> usize {
    let city = scan_city(path);
    city.harmonic_antinodes().len()
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input08.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input08.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mirroring() {
        let pos1 = Position(5, 4);
        let pos2 = Position(7, 4);
        let pos3 = Position(10, 10);
        assert_eq!(pos1.mirrored_across(&pos2), Position(9, 4));
        assert_eq!(pos2.mirrored_across(&pos1), Position(3, 4));
        assert_eq!(pos1.mirrored_across(&pos3), Position(15, 16));
        assert_eq!(pos3.mirrored_across(&pos1), Position(0, -2));
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(20, 5), 5);
        assert_eq!(gcd(5, 20), 5);
        assert_eq!(gcd(0, 8), 8);
        assert_eq!(gcd(3824, 218), 2);
        assert_eq!(gcd(91, 26), 13);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/input08.txt.test1"), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/input08.txt.test1"), 34);
    }
}
