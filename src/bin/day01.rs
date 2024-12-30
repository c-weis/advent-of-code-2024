use itertools::Itertools;
use rusty_advent_2024::utils::file_io;

fn part1(path: &str) -> i32 {
    let (mut v1, mut v2) = file_io::two_columns_from_file::<i32>(path);
    v1.sort();
    v2.sort();
    v1.into_iter()
        .zip(v2)
        .map(|(a, b)| -> i32 { (a - b).abs() })
        .sum::<i32>()
}

fn part2(path: &str) -> i32 {
    let (v1, v2) = file_io::two_columns_from_file::<i32>(path);
    let freq1 = v1.into_iter().counts();
    let freq2 = v2.into_iter().counts();
    freq1
        .iter()
        .map(|(number, occurrences1)| -> i32 {
            number * *occurrences1 as i32 * *freq2.get(number).unwrap_or(&0) as i32
        })
        .sum()
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input01.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input01.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input/input01.txt.test1"), 0);
        assert_eq!(part1("input/input01.txt.test2"), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/input01.txt.test1"), 6);
        assert_eq!(part2("input/input01.txt.test2"), 60);
    }
}
