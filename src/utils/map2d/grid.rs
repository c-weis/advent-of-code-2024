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
}

impl<T: PartialEq> Grid<T> {
    pub fn position_iter(&self) -> impl Iterator<Item = ValidPosition> {
        (0..self.bounds.0)
            .cartesian_product(0..self.bounds.1)
            .map(|(x, y)| ValidPosition(x, y))
    }

    pub fn value(&self, pos: &ValidPosition) -> &T {
        &self.data[pos.0 as usize][pos.1 as usize]
    }

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
