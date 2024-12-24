use num::Integer;
use std::{
    hash::Hash,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct IntVec2D<T: Integer>(pub T, pub T);

impl<T: Integer> Add<IntVec2D<T>> for IntVec2D<T> {
    type Output = IntVec2D<T>;
    fn add(self, rhs: IntVec2D<T>) -> Self::Output {
        IntVec2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Integer> Sub<IntVec2D<T>> for IntVec2D<T> {
    type Output = IntVec2D<T>;
    fn sub(self, rhs: IntVec2D<T>) -> Self::Output {
        IntVec2D(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: Integer + Copy> Mul<T> for IntVec2D<T> {
    type Output = IntVec2D<T>;

    fn mul(self, rhs: T) -> Self::Output {
        IntVec2D(self.0 * rhs, self.1 * rhs)
    }
}

impl<T: Integer + Copy> Div<T> for IntVec2D<T> {
    type Output = IntVec2D<T>;

    fn div(self, rhs: T) -> Self::Output {
        IntVec2D(self.0 / rhs, self.1 / rhs)
    }
}

impl<T: Integer + Copy> IntVec2D<T> {
    pub fn dot(self, rhs: IntVec2D<T>) -> T {
        self.0 * rhs.0 + self.1 * rhs.1
    }

    pub fn norm_sq(self) -> T {
        self.0 * self.0 + self.1 * self.1
    }
}

impl<T: Integer> From<(T, T)> for IntVec2D<T> {
    fn from((x, y): (T, T)) -> Self {
        IntVec2D(x, y)
    }
}
