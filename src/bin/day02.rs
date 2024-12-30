use rusty_advent_2024::utils::file_io;

#[derive(Clone, Copy, Debug, PartialEq)]
enum ReportType {
    Unsafe,
    Trivial,
    Increasing,
    Decreasing,
}

impl ReportType {
    pub fn is_safe(&self) -> bool {
        match self {
            ReportType::Unsafe => false,
            _ => true,
        }
    }

    pub fn combined_with(&self, other_type: &ReportType) -> ReportType {
        match (self, other_type) {
            (ReportType::Unsafe, _)
            | (_, ReportType::Unsafe)
            | (ReportType::Decreasing, ReportType::Increasing)
            | (ReportType::Increasing, ReportType::Decreasing) => ReportType::Unsafe,
            (ReportType::Trivial, other_type) => *other_type,
            (my_type, _) => *my_type,
        }
    }
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

fn report_type(report: &[i32]) -> ReportType {
    if report.len() < 2 {
        return ReportType::Trivial;
    }

    let mut differences = report.into_iter().zip(&report[1..]).map(|(v1, v2)| v2 - v1);

    if report[1] > report[0] && differences.all(is_safe_increase) {
        return ReportType::Increasing;
    } else if report[1] < report[0] && differences.all(is_safe_decrease) {
        return ReportType::Decreasing;
    }
    return ReportType::Unsafe;
}

fn is_safe_report(report: &[i32]) -> bool {
    report_type(report).is_safe()
}

fn is_safe_report_with_damper(report: &[i32]) -> bool {
    if report.len() < 3 {
        return true;
    }

    // Deal with special cases first
    if is_safe_report(&report[1..]) || is_safe_report(&report[..report.len() - 1]) {
        return true;
    }

    // Try removing elements individually
    for idx in 1..report.len() - 1 {
        let left = &report[..idx];
        let left_type = report_type(left);
        if !left_type.is_safe() {
            // if the left report is already unsafe, we cannot salvage it
            return false;
        }

        let mid = &vec![report[idx - 1], report[idx + 1]];
        let right_needs_type = report_type(mid).combined_with(&left_type);
        if !right_needs_type.is_safe() {
            continue;
        }

        let right = &report[idx + 1..];
        let right_type = report_type(right);
        if right_type.combined_with(&right_needs_type).is_safe() {
            return true;
        }
    }
    return false;
}

fn part1(path: &str) -> usize {
    let reports = file_io::rows_from_file::<i32>(path);
    reports
        .into_iter()
        .filter(|report: &Vec<i32>| is_safe_report(report))
        .count()
}

fn part2(path: &str) -> usize {
    let reports = file_io::rows_from_file::<i32>(path);
    reports
        .into_iter()
        .filter(|report: &Vec<i32>| is_safe_report_with_damper(report))
        .count()
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input02.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input02.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(is_safe_report(&vec![1, 3, 4, 5, 7]));
        assert!(is_safe_report(&vec![7, 5, 4, 3, 1]));
        assert!(is_safe_report(&vec![7, 4, 3, 2, 1]));
        assert!(is_safe_report(&vec![1, 3, 4, 3, 5]) == false);
        assert!(is_safe_report(&vec![8, 4, 3, 2, 1]) == false);
        assert_eq!(part1("input/input02.txt.test1"), 2);
    }

    #[test]
    fn test_part2() {
        assert!(is_safe_report_with_damper(&vec![1, 3, 4, 5, 7]));
        assert!(is_safe_report_with_damper(&vec![8, 5, 4, 2, 1]));
        assert!(is_safe_report_with_damper(&vec![1, 3, 4, 3, 5]));
        assert!(is_safe_report_with_damper(&vec![7, 8, 4, 3, 1]));
        assert!(is_safe_report_with_damper(&vec![3, 4, 3, 2, 1]));
        assert!(is_safe_report_with_damper(&vec![4, 3, 2, 1, 3]));
        assert!(is_safe_report_with_damper(&vec![4, 3, 4, 3, 4]) == false);
        assert_eq!(part2("input/input02.txt.test1"), 4);
    }
}
