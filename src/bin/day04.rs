use itertools::Itertools;
use rusty_advent_2024::utils::file_io;
use rusty_advent_2024::utils::map2d::grid::{Grid, ValidPosition};
use rusty_advent_2024::utils::map2d::position::Position;
use std::str::Chars;

type Puzzle = Grid<char>;

#[derive(Clone, Copy)]
struct StraightLine {
    start_pos: Position,
    dir: (i32, i32),
    len: usize,
}

impl Iterator for StraightLine {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        let pos = self.start_pos;
        self.start_pos = Position(pos.0 + self.dir.0, pos.1 + self.dir.1);
        self.len -= 1;
        Some(pos)
    }
}

fn matches_word(
    puzzle: &Puzzle,
    positions: impl Iterator<Item = Position>,
    subword: Chars,
) -> bool {
    positions.zip(subword).all(|(pos, c)| -> bool {
        pos.in_bounds(&puzzle.bounds)
            .is_some_and(|valid_pos| *puzzle.value(&valid_pos) == c)
    })
}

fn find_x_mas(puzzle: &Puzzle, &pos_a: &ValidPosition) -> bool {
    let Position(a_x, a_y) = pos_a.into();
    let diag1 = vec![Position(a_x - 1, a_y - 1), Position(a_x + 1, a_y + 1)];
    let diag2 = vec![Position(a_x - 1, a_y + 1), Position(a_x + 1, a_y - 1)];

    *(puzzle.value(&pos_a)) == 'A'
        && (matches_word(&puzzle, diag1.clone().into_iter(), "MS".chars())
            || matches_word(&puzzle, diag1.into_iter(), "SM".chars()))
        && (matches_word(&puzzle, diag2.clone().into_iter(), "MS".chars())
            || matches_word(&puzzle, diag2.into_iter(), "SM".chars()))
}

fn part1(path: &str) -> usize {
    let puzzle: Puzzle = file_io::strings_from_file(path).collect_vec().into();
    let directions: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    puzzle
        .position_iter()
        .map(Into::into)
        .cartesian_product(directions)
        .map(|(pos, dir)| -> StraightLine {
            // search all straight lines of length 4
            StraightLine {
                start_pos: pos,
                dir,
                len: 4,
            }
        })
        .filter(|line| matches_word(&puzzle, line.into_iter(), "XMAS".chars()))
        .count()
}

fn part2(path: &str) -> usize {
    let puzzle: Puzzle = file_io::strings_from_file(path).collect_vec().into();
    puzzle
        .position_iter()
        .filter(|pos| -> bool { find_x_mas(&puzzle, pos) })
        .count()
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input04.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input04.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/input04.txt.test1"), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/input04.txt.test1"), 9);
    }
}
