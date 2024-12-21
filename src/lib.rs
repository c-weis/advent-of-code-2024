pub mod utils {
    use std::{
        fmt::Debug,
        fs::File,
        io::{BufRead, BufReader, Lines},
        str::FromStr,
    };

    use itertools::Itertools;

    pub fn lines_from_file(path: &str) -> Lines<BufReader<File>> {
        let file = File::open(path).expect("Failed to open file.");
        BufReader::new(file).lines()
    }

    pub fn two_columns_from_file<T: FromStr>(path: &str) -> (Vec<T>, Vec<T>)
    where
        T::Err: Debug,
    {
        lines_from_file(path)
            .map(|line| -> (T, T) {
                line.unwrap()
                    .split_whitespace()
                    .map(|word| word.parse().expect(&format!("Failed to parse: {}.", word)))
                    .collect_tuple()
                    .expect("Each line must contain exactly two elements.")
            })
            .unzip()
    }

    pub fn rows_from_file<T: FromStr>(path: &str) -> Vec<Vec<T>>
    where
        T::Err: Debug,
    {
        lines_from_file(path)
            .map(|line| -> Vec<T> {
                line.unwrap()
                    .split_whitespace()
                    .map(|word: &str| {
                        word.parse::<T>()
                            .expect(&format!("Failed to parse: {}.", word))
                    })
                    .collect_vec()
            })
            .collect_vec()
    }
}

pub mod maps {
    use itertools::Itertools;
    use std::{
        collections::{HashSet, VecDeque},
        hash::Hash,
        io::{BufRead, Lines},
        ops::{Add, Div, Mul, Sub},
    };

    pub trait HasCharConverter {
        fn convert(c: char) -> Self;
    }

    impl HasCharConverter for u32 {
        fn convert(c: char) -> Self {
            c.to_digit(10).expect("Error converting digit.")
        }
    }

    impl HasCharConverter for char {
        fn convert(c: char) -> Self {
            c
        }
    }

    #[derive(Debug)]
    pub struct Map2D<T> {
        pub data: Vec<Vec<T>>,
        pub bounds: Bounds,
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    pub struct IntVec2D(pub i32, pub i32);

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    pub struct Position(pub i32, pub i32);

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    pub struct ValidPosition(pub usize, pub usize);

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    pub struct Bounds(pub usize, pub usize);

    impl From<(usize, usize)> for ValidPosition {
        fn from((x, y): (usize, usize)) -> ValidPosition {
            ValidPosition(x, y)
        }
    }

    impl From<ValidPosition> for Position {
        fn from(ValidPosition(x, y): ValidPosition) -> Position {
            Position(x as i32, y as i32)
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

        pub fn neighbours(&self) -> Vec<Position> {
            vec![
                Position(self.0 + 1, self.1),
                Position(self.0 - 1, self.1),
                Position(self.0, self.1 + 1),
                Position(self.0, self.1 - 1),
            ]
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
            let _pos: &Position = &(*self).into();
            _pos.neighbours()
                .into_iter()
                .filter_map(|neib| neib.in_bounds(bounds))
                .collect()
        }
    }

    impl<T: HasCharConverter, B: BufRead> From<Lines<B>> for Map2D<T> {
        fn from(lines: Lines<B>) -> Self {
            let data = lines
                .map(|line| -> Vec<T> {
                    line.unwrap()
                        .chars()
                        .map(|c| -> T { T::convert(c) })
                        .collect_vec()
                })
                .collect_vec();
            let bounds = Bounds(data[0].len(), data.len());
            Map2D { data, bounds }
        }
    }

    impl<T: PartialEq> Map2D<T> {
        pub fn position_iter(&self) -> impl Iterator<Item = ValidPosition> {
            (0..self.bounds.0)
                .cartesian_product(0..self.bounds.1)
                .map_into()
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

        pub fn mirrored_across(&self, other: &Self) -> Self {
            Position(2 * other.0 - self.0, 2 * other.1 - self.1)
        }

        pub fn add(self, IntVec2D(x, y): &IntVec2D) -> Self {
            Position(self.0 + x, self.1 + y)
        }
    }

    impl Add<IntVec2D> for Position {
        type Output = Position;

        fn add(self, rhs: IntVec2D) -> Self::Output {
            Position(self.0 + rhs.0, self.1 + rhs.1)
        }
    }

    impl Sub<Self> for Position {
        type Output = IntVec2D;
        fn sub(self, rhs: Self) -> Self::Output {
            IntVec2D(self.0 - rhs.0, self.1 - rhs.1)
        }
    }

    impl Sub<IntVec2D> for Position {
        type Output = Position;
        fn sub(self, rhs: IntVec2D) -> Self::Output {
            Position(self.0 - rhs.0, self.1 - rhs.1)
        }
    }

    impl Add<IntVec2D> for IntVec2D {
        type Output = IntVec2D;
        fn add(self, rhs: IntVec2D) -> Self::Output {
            IntVec2D(self.0 + rhs.0, self.1 + rhs.1)
        }
    }

    impl Sub<IntVec2D> for IntVec2D {
        type Output = IntVec2D;
        fn sub(self, rhs: IntVec2D) -> Self::Output {
            IntVec2D(self.0 - rhs.0, self.1 - rhs.1)
        }
    }

    impl Mul<i32> for IntVec2D {
        type Output = IntVec2D;
    
        fn mul(self, rhs: i32) -> Self::Output {
            IntVec2D(self.0 * rhs, self.1 * rhs)
        }
    }

    impl Div<i32> for IntVec2D {
        type Output = IntVec2D;
    
        fn div(self, rhs: i32) -> Self::Output {
            IntVec2D(self.0 / rhs, self.1 / rhs)
        }
    }
}
