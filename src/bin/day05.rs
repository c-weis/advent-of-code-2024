use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use rusty_advent_2024::utils::lines_from_file;

type RuleSet = HashMap<usize, HashSet<usize>>;
type Update = Vec<usize>;

fn update_rule(rules: &mut RuleSet, key: usize, value: usize) {
    if let Some(values) = rules.get_mut(&key) {
        values.insert(value);
    } else {
        rules.insert(key, HashSet::from([value]));
    }
}

fn middle_page(update: &Vec<usize>) -> usize {
    update[update.len() / 2]
}

fn is_valid(update: &Update, rules: &RuleSet) -> bool {
    if update.len() < 3 {
        return true;
    }

    let mut previous_pages: HashSet<usize> = HashSet::new();
    for page in update {
        if let Some(successors) = rules.get(page) {
            if !previous_pages.is_disjoint(successors) {
                return false;
            }
        }
        previous_pages.insert(*page);
    }

    true
}

fn read_in_file(path: &str) -> (RuleSet, Vec<Update>) {
    let lines = lines_from_file(path);

    let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut updates: Vec<Update> = Vec::new();

    let mut reading_rules: bool = true;
    for line in lines {
        let row = line.unwrap();
        if row.len() == 0 {
            reading_rules = false;
            continue;
        }

        if reading_rules {
            let (key, value): (usize, usize) = row
                .split("|")
                .map(|number| -> usize { number.parse().expect("Parsing {number} failed.") })
                .collect_tuple()
                .expect("Error collecting tuple.");

            update_rule(&mut rules, key, value);
        } else {
            let update: Update = row
                .split(r",")
                .map(|number| -> usize { number.parse().expect("Parsing {number} failed.") })
                .collect_vec();
            updates.push(update);
        }
    }

    (rules, updates)
}

fn fix_update(update: &mut Update, rules: &RuleSet) {
    let mut needs_sorting = true;

    // put numbers in correct order
    while needs_sorting {
        needs_sorting = false;
        for left in 0..update.len() - 1 {
            for right in left..update.len() {
                let (left_page, right_page) = (update[left], update[right]);
                if let Some(successors) = rules.get(&right_page) {
                    if successors.contains(&left_page) {
                        update.swap(left, right);
                        needs_sorting = true;
                    }
                }
            }
        }
    }
}

fn part1(path: &str) -> usize {
    let (rules, updates) = read_in_file(path);

    updates
        .iter()
        .filter(|update| is_valid(update, &rules))
        .map(middle_page)
        .sum()
}

fn part2(path: &str) -> usize {
    let (rules, mut updates) = read_in_file(path);

    let invalid_updates = updates
        .iter_mut()
        .filter(|update| !is_valid(update, &rules));

    invalid_updates
        .map(|update| -> usize {
            fix_update(update, &rules);
            middle_page(update)
        })
        .sum()
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input05.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input05.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1("input/input05.txt.test1") == 143);
    }

    #[test]
    fn test_part2() {
        assert!(part2("input/input05.txt.test1") == 123);
    }
}
