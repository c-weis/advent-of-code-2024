use itertools::Itertools;
use rusty_advent_2024::utils::{
    file_io,
    map2d::{
        direction::Direction,
        grid::{Convert, Grid, ToChar, ValidPosition},
    },
};
use std::collections::HashSet;

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    Box,
    Wall,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            'O' => Self::Box,
            _ => Self::Empty,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum HalfTile {
    Empty,
    BoxHalfLeft,
    BoxHalfRight,
    Wall,
}

impl ToChar for HalfTile {
    fn to_char(self: &Self) -> char {
        match self {
            Self::Wall => '#',
            Self::BoxHalfLeft => '[',
            Self::BoxHalfRight => ']',
            Self::Empty => '.',
        }
    }
}

impl From<char> for HalfTile {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '[' => Self::BoxHalfLeft,
            ']' => Self::BoxHalfRight,
            _ => Self::Empty,
        }
    }
}

trait IsTile {
    fn process_input_line(line: &str) -> String;
    fn adds_to_gps(&self) -> bool;
}
impl IsTile for Tile {
    fn process_input_line(line: &str) -> String {
        line.into()
    }

    fn adds_to_gps(&self) -> bool {
        *self == Self::Box
    }
}
impl IsTile for HalfTile {
    fn process_input_line(line: &str) -> String {
        line.replace(".", "..")
            .replace("O", "[]")
            .replace("#", "##")
            .replace("@", "@.")
    }

    fn adds_to_gps(&self) -> bool {
        *self == Self::BoxHalfLeft
    }
}

struct Warehouse<T: IsTile> {
    room: Grid<T>,
    robot: ValidPosition,
}

impl Warehouse<Tile> {
    fn try_step(&mut self, direction: Direction) -> bool {
        self.try_move(self.robot, direction)
            .then(|| {
                self.robot = self
                    .robot
                    .try_step(&direction, &self.room.bounds)
                    .expect("Error executing robot step.")
            })
            .is_some()
    }

    fn try_move(&mut self, start_pos: ValidPosition, direction: Direction) -> bool {
        let start_value = *self.room.value(&start_pos);
        start_pos
            .try_step(&direction, &self.room.bounds)
            .and_then(|next_pos| {
                let next_value = *self.room.value(&next_pos);
                match next_value {
                    Tile::Empty => Some(next_pos),
                    Tile::Box if self.try_move(next_pos, direction) => Some(next_pos),
                    _ => None,
                }
            })
            .map(|next_pos| {
                *self.room.value_mut(&next_pos) = start_value;
            })
            .is_some()
    }
}

impl Warehouse<HalfTile> {
    fn try_step(&mut self, direction: Direction) -> bool {
        match direction {
            Direction::RIGHT | Direction::LEFT => self.try_move_horizontally(self.robot, direction),
            Direction::UP | Direction::DOWN => {
                self.try_move_vertically([self.robot].into(), direction)
            }
        }
        .then(|| {
            self.robot = self
                .robot
                .try_step(&direction, &self.room.bounds)
                .expect("Error executing robot step.")
        })
        .is_some()
    }

    fn try_move_horizontally(&mut self, start_pos: ValidPosition, direction: Direction) -> bool {
        let start_value = *self.room.value(&start_pos);
        start_pos
            .try_step(&direction, &self.room.bounds)
            .and_then(|next_pos| {
                let next_value = *self.room.value(&next_pos);
                match next_value {
                    HalfTile::Empty => Some(next_pos),
                    HalfTile::BoxHalfLeft | HalfTile::BoxHalfRight
                        if self.try_move_horizontally(next_pos, direction) =>
                    {
                        Some(next_pos)
                    }
                    _ => None,
                }
            })
            .map(|next_pos| {
                *self.room.value_mut(&next_pos) = start_value;
            })
            .is_some()
    }

    fn try_move_vertically(
        &mut self,
        start_positions: HashSet<ValidPosition>,
        direction: Direction,
    ) -> bool {
        if start_positions.is_empty() {
            return true;
        }

        // we go row-by-row here. nothing can move unless everything moves, so must check first, then move
        // 1. collect obstacles in next row
        let mut obstacles: HashSet<ValidPosition> = HashSet::new();
        for start_pos in &start_positions {
            let next_pos = start_pos
                .try_step(&direction, &self.room.bounds)
                .expect("Stepped out of bounds - invalid state.");
            let next_value = *self.room.value(&next_pos);
            match next_value {
                HalfTile::Wall => return false,
                HalfTile::BoxHalfLeft => {
                    obstacles.insert(next_pos);
                    obstacles.insert(
                        next_pos
                            .try_step(&Direction::RIGHT, &self.room.bounds)
                            .expect("Box did not have right half - invalid state."),
                    );
                }
                HalfTile::BoxHalfRight => {
                    obstacles.insert(next_pos);
                    obstacles.insert(
                        next_pos
                            .try_step(&Direction::LEFT, &self.room.bounds)
                            .expect("Box did not have right half - invalid state."),
                    );
                }
                _ => (),
            };
        }

        // 2. try move obstacles, move if possible
        // TODO: refactor - have already computed next_pos
        self.try_move_vertically(obstacles, direction)
            .then(|| {
                for start_pos in start_positions {
                    let next_pos = start_pos
                        .try_step(&direction, &self.room.bounds)
                        .expect("Stepped out of bounds - invalid state.");
                    let start_value = *self.room.value(&start_pos);
                    *self.room.value_mut(&next_pos) = start_value;
                    *self.room.value_mut(&start_pos) = HalfTile::Empty;
                }
            })
            .is_some()
    }
}

impl<T: IsTile> Warehouse<T> {
    fn gps(self) -> usize {
        self.room
            .position_iter()
            .filter(|pos| T::adds_to_gps(self.room.value(pos)))
            .map(|ValidPosition(x, y)| x + 100 * y)
            .sum()
    }
}

impl<T: IsTile + ToChar> Warehouse<T> {
    fn pretty_print(&self) {
        let ValidPosition(robo_x, robo_y) = &self.robot;
        for y in 0..self.room.bounds.1 {
            for x in 0..self.room.bounds.0 {
                if (x, y) == (*robo_x, *robo_y) {
                    print!("@");
                } else {
                    print!("{}", (*self.room.value(&ValidPosition(x, y))).to_char());
                }
            }
            print!("\n");
        }
    }
}

fn load_input<T: IsTile + From<char>>(path: &str) -> (Warehouse<T>, Vec<Direction>) {
    let mut lines = file_io::strings_from_file(path);

    let map: Grid<char> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| T::process_input_line(&line))
        .collect_vec()
        .into();

    let instructions: Vec<Direction> = lines
        .join("")
        .chars()
        .map(|c| -> Direction { c.into() })
        .collect();

    let robot: ValidPosition = map
        .find(&'@')
        .drain()
        .exactly_one()
        .expect("Could not find unique robot position.");

    let warehouse = Warehouse {
        robot,
        room: map.convert(),
    };

    (warehouse, instructions)
}

fn part1(path: &str) -> usize {
    let (mut warehouse, instructions): (Warehouse<Tile>, _) = load_input(path);

    for direction in instructions {
        warehouse.try_step(direction);
    }

    warehouse.gps()
}

fn part2(path: &str, debug: bool) -> usize {
    let (mut warehouse, instructions): (Warehouse<HalfTile>, _) = load_input(path);

    if debug {
        println!("Initial:");
        warehouse.pretty_print();
    }
    for direction in instructions {
        warehouse.try_step(direction);
        if debug {
            println!("Step: {:?}", direction);
            warehouse.pretty_print();
        }
    }

    warehouse.gps()
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input15.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input15.txt", false));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/input15.txt.test1"), 2028);
        assert_eq!(part1("input/input15.txt.test2"), 10092);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/input15.txt.test2", false), 9021);
    }
}
