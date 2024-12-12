use itertools::Itertools;
use rusty_advent_2024::utils::lines_from_file;

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input04.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input04.txt"));
}

fn matches_word(haystack: &Vec<Vec<u8>>, indices: &Vec<(i32, i32)>, subword: &[u8]) -> bool {
    let w = haystack[0].len() as i32;
    let h = haystack.len() as i32;
    indices.iter().enumerate().all(|(subword_idx, (x, y))| {
        *x >= 0
            && *x <= w - 1
            && *y >= 0
            && *y <= h - 1
            && haystack[*y as usize][*x as usize] == subword[subword_idx]
    })
}

fn find_x_mas(haystack: &Vec<Vec<u8>>, a_coord: (i32, i32)) -> bool {
    let (a_x, a_y) = a_coord;
    let diag1 = vec![(a_x - 1, a_y - 1), (a_x + 1, a_y + 1)];
    let diag2 = vec![(a_x - 1, a_y + 1), (a_x + 1, a_y - 1)];

    haystack[a_y as usize][a_x as usize] == b'A'
        && (matches_word(haystack, &diag1, "MS".as_bytes())
            || matches_word(haystack, &diag1, "SM".as_bytes()))
        && (matches_word(haystack, &diag2, "MS".as_bytes())
            || matches_word(haystack, &diag2, "SM".as_bytes()))
}

fn part1(path: &str) -> usize {
    let puzzle = lines_from_file(path)
        .map(|line| line.unwrap().as_bytes().to_vec())
        .collect_vec();
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

    let w = puzzle[0].len() as i32;
    let h = puzzle[1].len() as i32;

    (0..w)
        .map(|x| -> usize {
            (0..h)
                .map(|y| -> usize {
                    directions
                        .iter()
                        .filter(|&&(dx, dy)| {
                            let indices = vec![
                                (x, y),
                                (x + dx, y + dy),
                                (x + 2 * dx, y + 2 * dy),
                                (x + 3 * dx, y + 3 * dy),
                            ];
                            matches_word(&puzzle, &indices, "XMAS".as_bytes())
                        })
                        .count()
                })
                .sum()
        })
        .sum()
}

fn part2(path: &str) -> usize {
    let puzzle = lines_from_file(path)
        .map(|line| line.unwrap().as_bytes().to_vec())
        .collect_vec();

    let w = puzzle[0].len() as i32;
    let h = puzzle[1].len() as i32;
    (1..w - 1)
        .map(|x| -> usize {
            (1..h - 1)
                .filter(|&y| -> bool { find_x_mas(&puzzle, (x, y)) })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1("input/input04.txt.test1") == 18);
    }

    #[test]
    fn test_part2() {
        assert!(part2("input/input04.txt.test1") == 9);
    }
}
