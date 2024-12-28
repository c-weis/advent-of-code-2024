use itertools::Itertools;
use num::abs;
use rusty_advent_2024::utils::{
    file_io,
    map2d::{
        grid::{Convert, Grid, ValidPosition},
        position::Position,
    },
};
use std::collections::{HashMap, HashSet};

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
            _ => panic!("Invalid character for racetrack field."),
        }
    }
}

#[derive(Debug)]
struct RaceTrack {
    field: Grid<Field>,
    start: ValidPosition,
    end: ValidPosition,
}

#[derive(Eq, PartialEq, Hash)]
struct Cheat {
    start: ValidPosition,
    end: ValidPosition,
}

impl Cheat {
    fn min_duration(&self) -> usize {
        (abs(self.start.0 as i32 - self.end.0 as i32)
            + abs(self.start.1 as i32 - self.end.1 as i32)) as usize
    }
}

impl RaceTrack {
    fn single_path(&self) -> Vec<ValidPosition> {
        let mut prev_pos: Option<ValidPosition> = None;
        let mut pos = self.start;
        let mut path: Vec<ValidPosition> = vec![pos];
        while pos != self.end {
            (prev_pos, pos) = (Some(pos),
            *pos
                .valid_neighbours(&self.field.bounds)
                .iter()
                .filter(|&&next_pos| {
                    *self.field.value(&next_pos) == Field::Empty
                        && prev_pos.is_none_or(|prev_pos| next_pos != prev_pos)
                })
                .exactly_one()
                .expect(
                    "Racetrack should have a unique step forward at each point except at the end.",
                )
            );

            path.push(pos);
        }

        path
    }

    fn timestamp_map(&self) -> HashMap<ValidPosition, usize> {
        self.single_path()
            .iter()
            .enumerate()
            .map(|(timestamp, &pos)| (pos, timestamp))
            .collect()
    }

    fn valid_neighbours_2(&self, pos: ValidPosition) -> Vec<ValidPosition> {
        [
            (2, 0),
            (1, 1),
            (0, 2),
            (-1, 1),
            (-2, 0),
            (-1, -1),
            (0, -2),
            (1, -1),
        ]
        .iter()
        .map(|(dx, dy)| Position(pos.0 as i32 + dx, pos.1 as i32 + dy))
        .filter_map(|pos| pos.in_bounds(&self.field.bounds))
        .collect()
    }

    fn valid_neighbours_20(&self, pos: ValidPosition) -> Vec<ValidPosition> {
        (-20..=20)
            .cartesian_product(-20..=20)
            .filter(|&(dx, dy)| abs(dx) + abs(dy) <= 20)
            .map(|(dx, dy)| Position(pos.0 as i32 + dx, pos.1 as i32 + dy))
            .filter_map(|pos| pos.in_bounds(&self.field.bounds))
            .collect()
    }

    fn cheats(&self) -> HashMap<usize, HashSet<Cheat>> {
        let timestamps = self.timestamp_map();
        let mut cheats: HashMap<usize, HashSet<Cheat>> = HashMap::new();
        for (start_pos, start_time) in &timestamps {
            self.valid_neighbours_2(*start_pos)
                .iter()
                .filter_map(|end_pos| -> Option<(ValidPosition, usize)> {
                    timestamps
                        .get(end_pos)
                        .and_then(|&time| Some((*end_pos, time)))
                })
                .filter_map(|(end_pos, end_time)| -> Option<(usize, Cheat)> {
                    if end_time > start_time + 2 {
                        Some((
                            end_time - (start_time + 2),
                            Cheat {
                                start: *start_pos,
                                end: end_pos,
                            },
                        ))
                    } else {
                        None
                    }
                })
                .for_each(|(time_save, cheat)| {
                    cheats
                        .entry(time_save)
                        .or_insert(HashSet::new())
                        .insert(cheat);
                })
        }
        cheats
    }

    fn big_cheats(&self) -> HashMap<usize, HashSet<Cheat>> {
        let timestamps = self.timestamp_map();
        let mut big_cheats: HashMap<usize, HashSet<Cheat>> = HashMap::new();
        for (start_pos, start_time) in &timestamps {
            self.valid_neighbours_20(*start_pos)
                .iter()
                .filter_map(|end_pos| -> Option<(ValidPosition, usize)> {
                    timestamps
                        .get(end_pos)
                        .and_then(|&time| Some((*end_pos, time)))
                })
                .filter_map(|(end_pos, end_time)| -> Option<(usize, Cheat)> {
                    let cheat = Cheat {
                        start: *start_pos,
                        end: end_pos,
                    };
                    if end_time > start_time + cheat.min_duration() {
                        Some((end_time - (start_time + cheat.min_duration()), cheat))
                    } else {
                        None
                    }
                })
                .for_each(|(time_save, cheat)| {
                    big_cheats
                        .entry(time_save)
                        .or_insert(HashSet::new())
                        .insert(cheat);
                })
        }
        big_cheats
    }
}

fn load_track(path: &str) -> RaceTrack {
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
    RaceTrack {
        field: char_grid.convert(),
        start,
        end,
    }
}

fn part1(path: &str, min_time_save: usize) -> usize {
    let race_track = load_track(path);
    let cheats = race_track.cheats();
    cheats
        .iter()
        .filter(|(&time_save, _)| time_save >= min_time_save)
        .map(|(_, cheat_set)| cheat_set.len())
        .sum()
}

fn part2(path: &str, min_time_save: usize) -> usize {
    let race_track = load_track(path);
    let cheats = race_track.big_cheats();
    cheats
        .iter()
        .filter(|(&time_save, _)| time_save >= min_time_save)
        .map(|(_, cheat_set)| cheat_set.len())
        .sum()
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input20.txt", 100));
    println!("Answer to part 2:");
    println!("{}", part2("input/input20.txt", 100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let race_track = load_track("input/input20.txt.test1");
        let cheats = race_track.cheats();
        let cheat_nrs: HashMap<usize, usize> = cheats
            .iter()
            .map(|(&time_save, cheat_set)| (time_save, cheat_set.len()))
            .collect();

        assert_eq!(cheat_nrs.get(&2), Some(&14));
        assert_eq!(cheat_nrs.get(&4), Some(&14));
        assert_eq!(cheat_nrs.get(&6), Some(&2));
        assert_eq!(cheat_nrs.get(&8), Some(&4));
        assert_eq!(cheat_nrs.get(&10), Some(&2));
        assert_eq!(cheat_nrs.get(&12), Some(&3));
        assert_eq!(cheat_nrs.get(&20), Some(&1));
        assert_eq!(cheat_nrs.get(&36), Some(&1));
        assert_eq!(cheat_nrs.get(&38), Some(&1));
        assert_eq!(cheat_nrs.get(&40), Some(&1));
        assert_eq!(cheat_nrs.get(&64), Some(&1));

        assert_eq!(cheat_nrs.values().sum::<usize>(), 44);
    }

    #[test]
    fn test_part2() {
        let race_track = load_track("input/input20.txt.test1");
        let cheats = race_track.big_cheats();
        let cheat_nrs: HashMap<usize, usize> = cheats
            .iter()
            .map(|(&time_save, cheat_set)| (time_save, cheat_set.len()))
            .collect();
        assert_eq!(cheat_nrs.get(&50), Some(&32));
        assert_eq!(cheat_nrs.get(&52), Some(&31));
        assert_eq!(cheat_nrs.get(&54), Some(&29));
        assert_eq!(cheat_nrs.get(&56), Some(&39));
        assert_eq!(cheat_nrs.get(&58), Some(&25));
        assert_eq!(cheat_nrs.get(&60), Some(&23));
        assert_eq!(cheat_nrs.get(&62), Some(&20));
        assert_eq!(cheat_nrs.get(&64), Some(&19));
        assert_eq!(cheat_nrs.get(&66), Some(&12));
        assert_eq!(cheat_nrs.get(&68), Some(&14));
        assert_eq!(cheat_nrs.get(&70), Some(&12));
        assert_eq!(cheat_nrs.get(&72), Some(&22));
        assert_eq!(cheat_nrs.get(&74), Some(&4));
        assert_eq!(cheat_nrs.get(&76), Some(&3));
        assert_eq!(
            cheat_nrs
                .iter()
                .filter_map(|(time_save, nr_cheats)| {
                    match time_save {
                        x if x < &50 => None,
                        _ => Some(nr_cheats),
                    }
                })
                .sum::<usize>(),
            285
        );
    }
}
