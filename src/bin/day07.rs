use itertools::Itertools;
use rusty_advent_2024::utils::lines_from_file;

struct Equation {
    target: usize,
    numbers: Vec<usize>,
}

fn equation_possible(target: usize, numbers: &[usize], concatenation_allowed: bool) -> bool {
    if numbers.len() == 1 {
        return target == numbers[0];
    }

    let number = numbers[numbers.len() - 1];

    target >= number
        && ((number != 0
            && target % number == 0
            && equation_possible(
                target / number,
                &numbers[..numbers.len() - 1],
                concatenation_allowed,
            ))
            || equation_possible(
                target - number,
                &numbers[..numbers.len() - 1],
                concatenation_allowed,
            )
            || (concatenation_allowed && {
                let divisor = match number {
                    0 => 10,
                    x => (10 as usize).pow(x.ilog10() + 1),
                };

                ((target - number) % divisor == 0)
                    && equation_possible(
                        (target - number) / divisor,
                        &numbers[..numbers.len() - 1],
                        concatenation_allowed,
                    )
            }))
}

fn equations_from_file(path: &str) -> Vec<Equation> {
    lines_from_file(path)
        .map(|line| line.unwrap())
        .filter_map(|line: String| -> Option<Equation> {
            line.split_once(": ").map(|(target, numbers)| -> Equation {
                Equation {
                    target: target.trim().parse().expect("Error parsing target number."),
                    numbers: numbers
                        .split_whitespace()
                        .map(|substr| substr.trim().parse().expect("Error parsing numbers."))
                        .collect_vec(),
                }
            })
        })
        .collect_vec()
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input07.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input07.txt"));
}

fn part1(path: &str) -> usize {
    let equations = equations_from_file(path);
    equations
        .iter()
        .filter(|Equation { target, numbers }| -> bool {
            equation_possible(*target, numbers, false)
        })
        .map(|Equation { target, numbers: _ }| target)
        .sum()
}

fn part2(path: &str) -> usize {
    let equations = equations_from_file(path);
    equations
        .iter()
        .filter(|Equation { target, numbers }| -> bool {
            equation_possible(*target, numbers, true)
        })
        .map(|Equation { target, numbers: _ }| target)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(equation_possible(5, &[5], false));
        assert!(equation_possible(50, &[5, 2, 5], false));
        assert!(!equation_possible(111, &[5, 2, 5, 6, 11, 22], false));
        assert!(!equation_possible(0, &[1, 4, 3], false));
        assert!(equation_possible(8, &[1, 4, 3], false));
        assert!(!equation_possible(14, &[1, 4, 3], false));
        assert!(equation_possible(15, &[1, 4, 3], false));
        assert!(part1("input/input07.txt.test1") == 3749);
    }

    #[test]
    fn test_part2() {
        assert!(equation_possible(50, &[5, 0], true));
        assert!(equation_possible(1150, &[10, 1, 50], true));
        assert!(equation_possible(15, &[5, 3], true));
        assert!(equation_possible(3511, &[5, 7, 11], true));
        assert!(equation_possible(5147, &[5, 100, 47], true));
        assert!(!equation_possible(5148, &[5, 100, 47], true));
        assert!(part2("input/input07.txt.test1") == 11387);
    }
}
