use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use rusty_advent_2024::utils::file_io;

const PRUNE_MASK: u32 = 0b111111111111111111111111;

#[inline(always)]
fn next_secret(secret: u32) -> u32 {
    let mut secret = (secret ^ secret << 6) & PRUNE_MASK;
    secret ^= secret >> 5; // prune unnecessary
    (secret ^ secret << 11) & PRUNE_MASK
}

fn next_2000_prices(secret: u32) -> [i8; 2001] {
    let mut prices: [i8; 2001] = [0; 2001];
    let mut secret = secret;
    for i in 0..=2000 {
        prices[i] = (secret % 10) as i8;
        secret = next_secret(secret);
    }
    prices
}

fn sequence_scores(prices: &[i8]) -> HashMap<(i8, i8, i8, i8), u32> {
    let mut sequence = (
        0,
        prices[1] - prices[0],
        prices[2] - prices[1],
        prices[3] - prices[2],
    );
    let mut scores = HashMap::new();
    for i in 4..prices.len() {
        sequence = (
            sequence.1,
            sequence.2,
            sequence.3,
            prices[i] - prices[i - 1],
        );
        scores.entry(sequence).or_insert(prices[i] as u32);
    }
    scores
}

fn load_secrets(path: &str) -> Vec<u32> {
    file_io::lines_from_file(path)
        .map(|word| -> u32 {
            word.unwrap()
                .parse()
                .expect("Each line should be a number.")
        })
        .collect()
}

fn part1(path: &str) -> u128 {
    let mut secrets = load_secrets(path);

    for _ in 0..2000 {
        secrets.iter_mut().for_each(|secret| {
            *secret = next_secret(*secret);
        });
    }

    secrets.into_iter().map_into::<u128>().sum()
}

fn part2(path: &str) -> u32 {
    let secrets = load_secrets(path);
    let price_lists = secrets
        .iter()
        .map(|&secret| next_2000_prices(secret))
        .collect_vec();

    let score_maps = price_lists
        .iter()
        .map(|price_list: &[i8; 2001]| sequence_scores(price_list))
        .collect_vec();

    let keys: HashSet<(i8, i8, i8, i8)> = score_maps
        .iter()
        .by_ref()
        .map(|map| -> HashSet<(i8, i8, i8, i8)> { map.keys().cloned().collect() })
        .flatten()
        .collect();

    keys.iter()
        .map(|key| -> u32 {
            score_maps
                .iter()
                .filter_map(|score_map| score_map.get(key))
                .sum()
        })
        .max()
        .unwrap()
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input22.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input22.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/input22.txt.test1"), 37327623);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/input22.txt.test2"), 23);
    }
}
