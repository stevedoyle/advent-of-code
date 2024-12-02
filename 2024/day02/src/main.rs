use std::time::Instant;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn solve_p1(input: &str) -> i32 {
    let reports = parse_input(input);
    reports.iter().filter(|report| is_safe(report)).count() as i32
}

fn solve_p2(input: &str) -> i32 {
    let reports = parse_input(input);
    reports
        .iter()
        .filter(|report| is_safe_with_dampener(report))
        .count() as i32
}

fn is_safe(report: &[i32]) -> bool {
    let ascending = report[0] < report[1];
    let rlen = report.len();
    for i in 1..rlen {
        if (ascending && report[i - 1] > report[i]) || (!ascending && report[i - 1] < report[i]) {
            return false;
        }
        let diff = (report[i] - report[i - 1]).abs();
        if !(1..=3).contains(&diff) {
            return false;
        }
    }
    true
}

fn is_safe_with_dampener(report: &[i32]) -> bool {
    if is_safe(report) {
        return true;
    }

    let rlen = report.len();
    for i in 0..rlen {
        let new_report: Vec<i32> = report[..i]
            .iter()
            .chain(report[i + 1..].iter())
            .cloned()
            .collect();
        if is_safe(&new_report) {
            return true;
        }
    }
    false
}

fn main() {
    let input = include_str!("../input.txt");

    let before = Instant::now();
    let answer = solve_p1(input);
    println!("Part 1: {answer}, elapsed: {:.2?}", before.elapsed());

    let before = Instant::now();
    let answer = solve_p2(input);
    println!("Part 2: {answer}, elapsed: {:.2?}", before.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 2);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 4);
    }
}
