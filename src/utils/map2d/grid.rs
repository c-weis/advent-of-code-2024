use crate::utils::map2d::direction::Direction;
use crate::utils::map2d::position::Position;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Bounds(pub usize, pub usize);

#[derive(Debug)]
pub struct Grid<T> {
    pub data: Vec<Vec<T>>,
    pub bounds: Bounds,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ValidPosition(pub usize, pub usize);

impl Into<Position> for ValidPosition {
    fn into(self) -> Position {
        Position(self.0 as i32, self.1 as i32)
    }
}

impl Position {
    pub fn in_bounds(&self, bounds: &Bounds) -> Option<ValidPosition> {
        if self.0 >= 0 && self.1 >= 0 && self.0 < bounds.0 as i32 && self.1 < bounds.1 as i32 {
            Some(ValidPosition(self.0 as usize, self.1 as usize))
        } else {
            None
        }
    }

    pub fn valid_neighbours(&self, bounds: &Bounds) -> HashSet<ValidPosition> {
        self.neighbours()
            .into_iter()
            .filter_map(|neib| neib.in_bounds(bounds))
            .collect()
    }
}

impl ValidPosition {
    pub fn valid_neighbours(&self, bounds: &Bounds) -> HashSet<ValidPosition> {
        let pos: Position = (*self).into();
        pos.valid_neighbours(bounds)
    }

    pub fn try_step(&self, direction: &Direction, bounds: &Bounds) -> Option<Self> {
        let pos: Position = (*self).into();
        pos.step(direction).in_bounds(bounds)
    }
}

impl<T> Grid<T> {
    pub fn position_iter(&self) -> impl Iterator<Item = ValidPosition> {
        (0..self.bounds.0)
            .cartesian_product(0..self.bounds.1)
            .map(|(x, y)| ValidPosition(x, y))
    }

    pub fn value(&self, pos: &ValidPosition) -> &T {
        &self.data[pos.1 as usize][pos.0 as usize]
    }

    pub fn value_mut(&mut self, pos: &ValidPosition) -> &mut T {
        &mut self.data[pos.1 as usize][pos.0 as usize]
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn find(&self, value: &T) -> HashSet<ValidPosition> {
        self.position_iter()
            .filter(|pos| -> bool { self.value(pos) == value })
            .collect()
    }

    pub fn contiguous_region(&self, &pos: &ValidPosition) -> HashSet<ValidPosition> {
        let mut visited: HashSet<ValidPosition> = HashSet::new();
        let mut to_visit: VecDeque<ValidPosition> = VecDeque::new();
        to_visit.push_back(pos);
        let target_value = self.value(&pos);

        while let Some(next_pos) = to_visit.pop_front() {
            if !visited.insert(next_pos.clone()) {
                continue;
            }

            for neib in next_pos.valid_neighbours(&self.bounds) {
                if self.value(&neib) == target_value {
                    to_visit.push_back(neib);
                }
            }
        }

        visited
    }
}

pub trait ToChar {
    fn to_char(self: &Self) -> char;
}

impl ToChar for char {
    fn to_char(self: &Self) -> char {
        *self
    }
}

impl<T: ToChar> Grid<T> {
    pub fn pretty_print_string(&self) -> String {
        self.data
            .iter()
            .map(|vec| vec.iter().map(|c| -> char { c.to_char() }).join(""))
            .join("\n")
    }
}

pub trait Convert<S> {
    fn convert(&self) -> S;
}

impl<S: Clone + Into<T>, T> Convert<Grid<T>> for Grid<S> {
    fn convert(&self) -> Grid<T> {
        let new_data: Vec<Vec<T>> = self
            .data
            .iter()
            .map(|vec| vec.iter().map(|s| s.clone().into()).collect_vec())
            .collect_vec();
        Grid {
            data: new_data,
            bounds: self.bounds,
        }
    }
}
