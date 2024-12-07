use rusty_advent_2024::utils;

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input02.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input02.txt"));
}

fn is_safe_increase(difference: i32) -> bool {
    match difference {
        1 | 2 | 3 => true,
        _ => false,
    }
}

fn is_safe_decrease(difference: i32) -> bool {
    is_safe_increase(-difference)
}

fn is_safe_report(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        return true;
    }

    let mut differences = report.into_iter().zip(&report[1..]).map(|(v1, v2)| v2 - v1);
    match report[1] > report[0] {
        true => differences.all(is_safe_increase),
        false => differences.all(is_safe_decrease),
    }
}

fn part1(path: &str) -> usize {
    let reports = utils::rows_from_file::<i32>(path);
    reports.into_iter().filter(is_safe_report).count()
}

fn part2(_path: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(is_safe_report(&vec![1, 3, 4, 5, 7]) == true);
        assert!(is_safe_report(&vec![1, 3, 4, 3, 5]) == false);
        assert!(is_safe_report(&vec![7, 5, 4, 3, 1]) == true);
        assert!(is_safe_report(&vec![7, 4, 3, 2, 1]) == true);
        assert!(is_safe_report(&vec![8, 4, 3, 2, 1]) == false);
        assert!(part1("input/input02.txt.test1") == 2);
    }

    #[test]
    fn test_part2() {
        assert!(part2("input/input02.txt.test1") == 0);
    }
}
