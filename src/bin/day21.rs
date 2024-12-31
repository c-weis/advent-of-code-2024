use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use itertools::Itertools;
use num::abs;
use rusty_advent_2024::utils::{file_io, math2d::IntVec2D};
use std::hash::Hash;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum NumericKey {
    Number(u8),
    A,
}

impl From<NumericKey> for char {
    fn from(k: NumericKey) -> Self {
        match k {
            NumericKey::A => 'A',
            NumericKey::Number(x) => char::from_digit(x.into(), 10)
                .expect("NumericKey::Number(x) should have x between 0-9."),
        }
    }
}

impl From<char> for NumericKey {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::A,
            _ => Self::Number(
                c.to_digit(10)
                    .expect("Characters on numeric keypad must be 0-9 or A.") as u8,
            ),
        }
    }
}

impl From<NumericKey> for IntVec2D<i32> {
    fn from(k: NumericKey) -> Self {
        match k {
            NumericKey::A => IntVec2D(2, 0),
            NumericKey::Number(0) => IntVec2D(1, 0),
            NumericKey::Number(x) if x <= 9 => IntVec2D((x as i32 - 1) % 3, (x as i32 - 1) / 3 + 1),
            _ => panic!("Integer stored in NumericKey::Number should be 0-9."),
        }
    }
}

#[derive(Debug)]
struct InvalidKeypadPositionError(i32, i32);
impl TryFrom<IntVec2D<i32>> for NumericKey {
    type Error = InvalidKeypadPositionError;

    fn try_from(pos: IntVec2D<i32>) -> Result<Self, Self::Error> {
        match pos {
            IntVec2D(2, 0) => Ok(Self::A),
            IntVec2D(1, 0) => Ok(Self::Number(0)),
            IntVec2D(x, y) if x >= 0 && y >= 1 && x <= 2 && y <= 3 => {
                Ok(Self::Number((3 * y + x - 2) as u8))
            }
            IntVec2D(x, y) => Err(InvalidKeypadPositionError(x, y)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum DirectionalKey {
    Up,
    A,
    Left,
    Down,
    Right,
}

impl From<DirectionalKey> for char {
    fn from(k: DirectionalKey) -> Self {
        match k {
            DirectionalKey::A => 'A',
            DirectionalKey::Right => '>',
            DirectionalKey::Up => '^',
            DirectionalKey::Left => '<',
            DirectionalKey::Down => 'v',
        }
    }
}

impl From<DirectionalKey> for IntVec2D<i32> {
    fn from(k: DirectionalKey) -> Self {
        match k {
            DirectionalKey::Up => IntVec2D(1, 1),
            DirectionalKey::A => IntVec2D(2, 1),
            DirectionalKey::Left => IntVec2D(0, 0),
            DirectionalKey::Down => IntVec2D(1, 0),
            DirectionalKey::Right => IntVec2D(2, 0),
        }
    }
}

impl TryFrom<IntVec2D<i32>> for DirectionalKey {
    type Error = InvalidKeypadPositionError;

    fn try_from(pos: IntVec2D<i32>) -> Result<Self, Self::Error> {
        match pos {
            IntVec2D(1, 1) => Ok(Self::Up),
            IntVec2D(2, 1) => Ok(Self::A),
            IntVec2D(0, 0) => Ok(Self::Left),
            IntVec2D(1, 0) => Ok(Self::Down),
            IntVec2D(2, 0) => Ok(Self::Right),
            IntVec2D(x, y) => Err(InvalidKeypadPositionError(x, y)),
        }
    }
}

impl From<char> for DirectionalKey {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::A,
            '>' => Self::Right,
            '^' => Self::Up,
            '<' => Self::Left,
            'v' => Self::Down,
            _ => panic!("Characters on directional keypad must be <,^,>,v or A."),
        }
    }
}

impl DirectionalKey {
    fn step(&self, pos: IntVec2D<i32>) -> IntVec2D<i32> {
        match self {
            DirectionalKey::A => pos,
            DirectionalKey::Right => IntVec2D(pos.0 + 1, pos.1),
            DirectionalKey::Left => IntVec2D(pos.0 - 1, pos.1),
            DirectionalKey::Up => IntVec2D(pos.0, pos.1 + 1),
            DirectionalKey::Down => IntVec2D(pos.0, pos.1 - 1),
        }
    }
}

trait KeypadKey:
    TryFrom<IntVec2D<i32>> + Into<IntVec2D<i32>> + Copy + Eq + PartialEq + Hash + From<char> + Debug
{
    fn compute_key_sequences((start, end): &(Self, Self)) -> HashSet<Sequence<DirectionalKey>> {
        let start_pos: IntVec2D<i32> = start.clone().into();
        let end_pos: IntVec2D<i32> = end.clone().into();

        let IntVec2D(dx, dy) = end_pos - start_pos;

        if dy >= 0 {
            if dx >= 0 {
                // dx >= 0, dy >= 0 - move right then up
                [
                    [DirectionalKey::Right].repeat(dx as usize),
                    [DirectionalKey::Up].repeat(dy as usize),
                ]
            } else {
                // if legal, move left then up
                [
                    [DirectionalKey::Left].repeat(-dx as usize),
                    [DirectionalKey::Up].repeat(dy as usize),
                ]
            }
        } else {
            if dx >= 0 {
                // dx >= 0, dy < 0 - move right then down
                [
                    [DirectionalKey::Right].repeat(dx as usize),
                    [DirectionalKey::Down].repeat(-dy as usize),
                ]
            } else {
                // dx < 0, dy < 0 - move down then left
                [
                    [DirectionalKey::Down].repeat(-dy as usize),
                    [DirectionalKey::Left].repeat(-dx as usize),
                ]
            }
        }
        .concat()
        .into_iter()
        .permutations(abs(dx) as usize + abs(dy) as usize)
        .filter(|seq| Self::is_valid_sequence(start_pos, seq))
        .map(|seq| [seq, vec![DirectionalKey::A]].concat())
        .collect()
    }

    fn is_valid_sequence(start_pos: IntVec2D<i32>, seq: &Sequence<DirectionalKey>) -> bool {
        let mut pos = start_pos;

        for key in seq {
            if !Self::is_valid(pos) {
                return false;
            }

            pos = key.step(pos);
        }

        Self::is_valid(pos)
    }

    fn start_key() -> Self;
    fn is_valid(pos: IntVec2D<i32>) -> bool;

    fn to_directional_key(self) -> DirectionalKey {
        panic!("Cannot convert key {:?} to DirectionalKey.", self)
    }

    fn sequence_from_string(s: &str) -> Sequence<Self> {
        s.chars().map(|c| c.into()).collect()
    }
}

impl KeypadKey for NumericKey {
    fn start_key() -> Self {
        Self::A
    }

    fn is_valid(pos: IntVec2D<i32>) -> bool {
        match (pos.0, pos.1) {
            (0, 0) => false,
            (x, y) if x >= 0 && y >= 0 && x <= 2 && y <= 3 => true,
            _ => false,
        }
    }
}

impl KeypadKey for DirectionalKey {
    fn start_key() -> Self {
        Self::A
    }

    fn is_valid(pos: IntVec2D<i32>) -> bool {
        match (pos.0, pos.1) {
            (0, 1) => false,
            (x, y) if x >= 0 && y >= 0 && x <= 2 && y <= 1 => true,
            _ => false,
        }
    }

    fn to_directional_key(self) -> DirectionalKey {
        self
    }
}

type Sequence<T> = Vec<T>;
type Transition<T> = (T, T);

struct Keypad<T: KeypadKey> {
    cached_sequences: HashMap<Transition<T>, Sequence<DirectionalKey>>,
    cached_lengths: HashMap<Transition<T>, usize>,
    controller: Option<Box<Keypad<DirectionalKey>>>,
}

impl<T: KeypadKey> Keypad<T> {
    fn new() -> Self {
        Keypad {
            cached_sequences: HashMap::new(),
            cached_lengths: HashMap::new(),
            controller: None,
        }
    }

    fn with_controller(mut self, controller: Keypad<DirectionalKey>) -> Self {
        self.controller = Some(Box::new(controller));
        self
    }

    fn min_for_sequence(&mut self, seq: Sequence<T>) -> Sequence<DirectionalKey> {
        let transitions: Vec<Transition<T>> = [vec![T::start_key()], seq]
            .iter()
            .flatten()
            .cloned()
            .tuple_windows()
            .collect();

        transitions
            .into_iter()
            .flat_map(|t| self.min_for_transition(t))
            .collect()
    }

    fn min_for_transition(&mut self, t: Transition<T>) -> Sequence<DirectionalKey> {
        if let Some(sequence) = self.cached_sequences.get(&t) {
            return sequence.clone();
        }

        let min_seq = match &mut self.controller {
            Some(controller) => T::compute_key_sequences(&t)
                .into_iter()
                .map(|seq| controller.min_for_sequence(seq))
                .min_by_key(|seq| seq.len()),
            None => Some(vec![t.1.to_directional_key()]),
        }
        .expect("No transition should be impossible");

        self.cached_sequences.insert(t, min_seq.clone());
        min_seq
    }

    fn min_len_for_sequence(&mut self, seq: Sequence<T>) -> usize {
        let transitions: Vec<Transition<T>> = [vec![T::start_key()], seq]
            .iter()
            .flatten()
            .cloned()
            .tuple_windows()
            .collect();

        transitions
            .into_iter()
            .map(|t| self.min_len_for_transition(t))
            .sum()
    }

    fn min_len_for_transition(&mut self, t: Transition<T>) -> usize {
        if let Some(length) = self.cached_lengths.get(&t) {
            return *length;
        }

        let min_len: usize = match &mut self.controller {
            Some(controller) => T::compute_key_sequences(&t)
                .into_iter()
                .map(|seq| controller.min_len_for_sequence(seq))
                .min()
                .expect("No transition should be impossible."),
            None => 1,
        };

        self.cached_lengths.insert(t, min_len);
        min_len
    }
}

fn load_data(path: &str) -> (Vec<Sequence<NumericKey>>, Vec<usize>) {
    let strings = file_io::strings_from_file(path).collect_vec();
    let codes: Vec<Sequence<NumericKey>> = strings
        .clone()
        .iter()
        .map(|string| NumericKey::sequence_from_string(string.as_str()))
        .collect();

    let numeric_parts = strings
        .iter()
        .map(|code| -> usize {
            code.chars()
                .take(3)
                .join("")
                .parse()
                .expect("First three characters of code must parse to number.")
        })
        .collect_vec();
    (codes, numeric_parts)
}

fn complexity(
    control_sequences: Vec<Sequence<DirectionalKey>>,
    numeric_parts: Vec<usize>,
) -> usize {
    control_sequences
        .iter()
        .zip(numeric_parts)
        .map(|(sequence, numeric_part)| sequence.len() * numeric_part)
        .sum()
}

fn pretty_print(control_sequence: &Sequence<DirectionalKey>) {
    println!(
        "{}, len: {}",
        control_sequence
            .iter()
            .cloned()
            .map(|key| -> char { key.into() })
            .join(""),
        control_sequence.len()
    );
}

fn part1(path: &str) -> usize {
    let (codes, numeric_parts) = load_data(path);

    let handheld_keypad: Keypad<DirectionalKey> = Keypad::new();
    let freezing_keypad: Keypad<DirectionalKey> = Keypad::new().with_controller(handheld_keypad);
    let radiated_keypad: Keypad<DirectionalKey> = Keypad::new().with_controller(freezing_keypad);
    let mut depressurised_keypad: Keypad<NumericKey> =
        Keypad::new().with_controller(radiated_keypad);

    let control_sequences: Vec<Sequence<DirectionalKey>> = codes
        .into_iter()
        .map(|code| depressurised_keypad.min_for_sequence(code))
        .collect();

    complexity(control_sequences, numeric_parts)
}

fn part2(path: &str) -> usize {
    let (codes, numeric_parts) = load_data(path);

    let handheld_keypad: Keypad<DirectionalKey> = Keypad::new();
    let mut previous_keypad = handheld_keypad;

    for _ in 0..25 {
        previous_keypad = Keypad::new().with_controller(previous_keypad);
    }

    let mut number_pad: Keypad<NumericKey> = Keypad::new().with_controller(previous_keypad);

    let sequence_lengths: Vec<usize> = codes
        .into_iter()
        .map(|code| number_pad.min_len_for_sequence(code))
        .collect();

    sequence_lengths
        .iter()
        .zip(numeric_parts)
        .map(|(length, number)| length * number)
        .sum()
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input21.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input21.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2_keypads() {
        let handheld: Keypad<DirectionalKey> = Keypad::new();
        let mut number_pad: Keypad<NumericKey> = Keypad::new().with_controller(handheld);

        let code: Sequence<NumericKey> = NumericKey::sequence_from_string("023A");

        assert_eq!(
            number_pad.min_for_sequence(code),
            DirectionalKey::sequence_from_string("<A^A>AvA")
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/input21.txt.test1"), 126384);
    }
}
