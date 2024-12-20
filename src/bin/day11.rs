use std::collections::HashMap;

use itertools::Itertools;
use rusty_advent_2024::utils;

type BigNumber = u64;
type StoneList = Vec<BigNumber>;
type StoneMap = HashMap<BigNumber, usize>;

fn stone_list_from_file(path: &str) -> StoneList {
    utils::lines_from_file(path)
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|word| -> BigNumber { word.parse().expect("Error parsing word {word}.") })
                .collect_vec()
        })
        .flatten()
        .collect()
}

fn stone_map_from_file(path: &str) -> StoneMap {
    utils::lines_from_file(path)
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|word| -> BigNumber { word.parse().expect("Error parsing word {word}.") })
                .collect_vec()
        })
        .flatten()
        .counts()
}

fn even_number_of_digits(value: &BigNumber) -> bool {
    value.ilog10() % 2 == 1
}

fn split_digits_vec(value: &BigNumber) -> Vec<BigNumber> {
    let half_digits = (value.ilog10() + 1) / 2;
    let factor = (10 as BigNumber).pow(half_digits);

    vec![value / factor, value % factor]
}

fn split_digits(value: &BigNumber) -> (BigNumber, BigNumber) {
    let half_digits = (value.ilog10() + 1) / 2;
    let factor = (10 as BigNumber).pow(half_digits);

    (value / factor, value % factor)
}

fn blink_map(stone_map: StoneMap) -> StoneMap {
    let mut next_map: StoneMap = HashMap::new();
    for (stone, count) in stone_map {
        match stone {
            0 => {
                *next_map.entry(1).or_insert(0) += count;
            }
            x if even_number_of_digits(&x) => {
                let (left, right) = split_digits(&x);
                *next_map.entry(left).or_insert(0) += count;
                *next_map.entry(right).or_insert(0) += count;
            },
            y => 
            {
                *next_map.entry(y * 2024).or_insert(0) += count;
            }
        }
    };

    next_map
}

fn blink_list(stone_list: StoneList) -> StoneList {
    stone_list
        .iter()
        .flat_map(|stone| -> Vec<BigNumber> {
            match stone {
                0 => {
                    vec![1]
                }
                x if even_number_of_digits(x) => split_digits_vec(x),
                y => vec![y * 2024],
            }
        })
        .collect()
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input11.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input11.txt"));
}

fn part1(path: &str) -> usize {
    let mut stone_list: StoneList = stone_list_from_file(path);
    for _ in 1..=25 {
        stone_list = blink_list(stone_list);
    }
    stone_list.len()
}

fn part2(path: &str) -> usize {
    let mut stone_map: StoneMap = stone_map_from_file(path);

    for _ in 1..=75 {
        stone_map = blink_map(stone_map);
    }

    stone_map.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blink() {
        assert!(blink_list(vec![0]) == vec![1]);
        assert!(blink_list(vec![1234]) == vec![12, 34]);
        assert!(blink_list(vec![1]) == vec![2024]);
        assert!(blink_list(vec![10, 3, 0]) == vec![1, 0, 6072, 1]);
    }

    #[test]
    fn test_part1() {
        assert!(part1("input/input11.txt.test1") == 55312);
    }
}
