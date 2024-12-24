use crate::utils::map2d::position::Position;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    pub fn turned_right(self) -> Self {
        match self {
            Self::UP => Self::RIGHT,
            Self::RIGHT => Self::DOWN,
            Self::DOWN => Self::LEFT,
            Self::LEFT => Self::UP,
        }
    }

    pub fn turned_left(self) -> Self {
        match self {
            Self::UP => Self::LEFT,
            Self::LEFT => Self::DOWN,
            Self::DOWN => Self::RIGHT,
            Self::RIGHT => Self::UP,
        }
    }

    pub fn turned_around(self) -> Self {
        match self {
            Self::UP => Self::DOWN,
            Self::RIGHT => Self::LEFT,
            Self::DOWN => Self::UP,
            Self::LEFT => Self::RIGHT,
        }
    }

    pub fn turn_right(self: &mut Self) {
        *self = self.turned_right();
    }

    pub fn turn_left(self: &mut Self) {
        *self = self.turned_left();
    }

    pub fn turn_around(self: &mut Self) {
        *self = self.turned_around();
    }

    pub fn iter_all() -> impl Iterator<Item = Direction> {
        [
            Direction::UP,
            Direction::RIGHT,
            Direction::DOWN,
            Direction::LEFT,
        ]
        .iter()
        .copied()
    }
}

impl From<char> for Direction {
    fn from(character: char) -> Self {
        match character {
            '^' => Direction::UP,
            '>' => Direction::RIGHT,
            'v' => Direction::DOWN,
            '<' => Direction::LEFT,
            _ => panic!("Invalid character {character} specified to create Direction."),
        }
    }
}

impl Position {
    pub fn step(&self, direction: &Direction) -> Position {
        let Position(x, y) = self;

        match direction {
            Direction::UP => Position(*x, *y - 1),
            Direction::RIGHT => Position(*x + 1, *y),
            Direction::DOWN => Position(*x, *y + 1),
            Direction::LEFT => Position(*x - 1, *y),
        }
    }
}
