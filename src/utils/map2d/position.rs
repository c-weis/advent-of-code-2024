use std::ops::{Add, Sub};

use crate::utils::math2d::IntVec2D;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Position(pub i32, pub i32);

impl Position {
    pub fn neighbours(&self) -> Vec<Position> {
        vec![
            Position(self.0 + 1, self.1),
            Position(self.0 - 1, self.1),
            Position(self.0, self.1 + 1),
            Position(self.0, self.1 - 1),
        ]
    }

    pub fn mirrored_across(&self, other: &Self) -> Self {
        Position(2 * other.0 - self.0, 2 * other.1 - self.1)
    }
}

impl Add<IntVec2D<i32>> for Position {
    type Output = Position;

    fn add(self, rhs: IntVec2D<i32>) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Position> for Position {
    type Output = IntVec2D<i32>;

    fn sub(self, rhs: Position) -> Self::Output {
        IntVec2D(self.0 - rhs.0, self.1 - rhs.1)
    }
}
