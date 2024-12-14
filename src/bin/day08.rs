use std::collections::{HashMap, HashSet};

use rusty_advent_2024::utils::lines_from_file;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Position(i32, i32);
#[derive(Debug)]
struct Bounds(usize, usize);

#[derive(Debug)]
struct Antenna {
    frequency: u8,
    pos: Position,
}

#[derive(Debug)]
struct AntennaMap(HashMap<u8, HashSet<Position>>);

impl Position {
    fn mirrored_across(&self, other: &Self) -> Self {
        Position(2 * other.0 - self.0, 2 * other.1 - self.1)
    }

    fn in_bounds(&self, bounds: &Bounds) -> bool {
        self.0 >= 0 && self.1 >= 0 && self.0 < bounds.0 as i32 && self.1 < bounds.1 as i32
    }

    fn plus(self, (x, y): (i32, i32)) -> Position {
        Position(self.0 + x, self.1 + y)
    }
}

impl AntennaMap {
    fn add(&mut self, antenna: Antenna) {
        if let Some(positions) = self.0.get_mut(&antenna.frequency) {
            positions.insert(antenna.pos);
        } else {
            self.0
                .insert(antenna.frequency, HashSet::from([antenna.pos]));
        }
    }

    fn new() -> Self {
        AntennaMap(HashMap::new())
    }
}

#[derive(Debug)]
struct City {
    bounds: Bounds,
    antenna_map: AntennaMap,
}

impl City {
    fn basic_antinodes(self) -> HashSet<Position> {
        let mut antinodes: HashSet<Position> = HashSet::new();

        for position_list in self.antenna_map.0.values() {
            for pos1 in position_list {
                for pos2 in position_list {
                    if pos1 == pos2 {
                        continue;
                    }

                    let antinode = pos1.mirrored_across(pos2);
                    if antinode.in_bounds(&self.bounds) {
                        antinodes.insert(antinode);
                    }
                }
            }
        }

        antinodes
    }

    fn harmonic_antinodes(self) -> HashSet<Position> {
        let mut antinodes: HashSet<Position> = HashSet::new();

        for position_list in self.antenna_map.0.values() {
            for pos1 in position_list {
                for pos2 in position_list {
                    if pos1 == pos2 {
                        continue;
                    }

                    let (dx, dy) = (pos2.0 - pos1.0, pos2.1 - pos1.1);
                    let gcd = gcd(dx.abs() as usize, dy.abs() as usize) as i32;
                    let (dx, dy) = (dx / gcd, dy / gcd);

                    let mut antinode = *pos1; 
                    while antinode.in_bounds(&self.bounds) {
                        antinodes.insert(antinode.clone());
                        antinode = antinode.plus((dx, dy));
                    } 
                }
            }
        }

        antinodes
    }
}

fn gcd(a: usize, b: usize) -> usize {
    match (a, b) {
        (x, 0) | (0, x) => x,
        _ => gcd(b, a % b),
    }
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input08.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input08.txt"));
}

fn scan_city(path: &str) -> City {
    let mut antennae = AntennaMap::new();
    let mut bounds: Bounds = Bounds(0, 0);
    for (y, line) in lines_from_file(path).into_iter().enumerate() {
        for (x, c) in line.unwrap().as_bytes().iter().enumerate() {
            match *c {
                b'.' => {}
                _ => antennae.add(Antenna {
                    frequency: *c,
                    pos: Position(x as i32, y as i32),
                }),
            };

            bounds = Bounds(x + 1, y + 1);
        }
    }

    City {
        bounds,
        antenna_map: antennae,
    }
}

fn part1(path: &str) -> usize {
    let city = scan_city(path);

    city.basic_antinodes().len()
}

fn part2(path: &str) -> usize {
    let city = scan_city(path);

    city.harmonic_antinodes().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mirroring() {
        let pos1 = Position(5, 4);
        let pos2 = Position(7, 4);
        let pos3 = Position(10, 10);
        assert!(pos1.mirrored_across(&pos2) == Position(9, 4));
        assert!(pos2.mirrored_across(&pos1) == Position(3, 4));
        assert!(pos1.mirrored_across(&pos3) == Position(15, 16));
        assert!(pos3.mirrored_across(&pos1) == Position(0, -2));
    }

    #[test]
    fn test_gcd() {
        assert!(gcd(20, 5) == 5);
        assert!(gcd(5, 20) == 5);
        assert!(gcd(0, 8) == 8);
        assert!(gcd(3824, 218) == 2);
        assert!(gcd(91, 26) == 13);
    }

    #[test]
    fn test_part1() {
        assert!(part1("input/input08.txt.test1") == 14);
    }

    #[test]
    fn test_part2() {
        assert!(part2("input/input08.txt.test1") == 34);
    }
}
