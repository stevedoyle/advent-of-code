use std::ops::RangeInclusive;

fn solve_p1(input: &str) -> i32 {
    parse_input(input)
        .iter()
        .filter(|(a, b)| a.start() == b.start() || b.end() <= a.end())
        .count() as i32
}

fn solve_p2(input: &str) -> i32 {
    parse_input(input)
        .iter()
        .filter(|(a, b)| b.start() <= a.end())
        .count() as i32
}

fn parse_input(input: &str) -> Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> {
    let mut ranges = Vec::new();
    for line in input.lines() {
        let (left, right) = line.split_once(',').unwrap();

        let (start, end) = left.split_once('-').unwrap();
        let r1: RangeInclusive<i32> = start.parse().unwrap()..=end.parse().unwrap();

        let (start, end) = right.split_once('-').unwrap();
        let r2: RangeInclusive<i32> = start.parse().unwrap()..=end.parse().unwrap();

        if r1.start() < r2.start() {
            ranges.push((r1, r2));
        } else {
            ranges.push((r2, r1));
        }
    }
    ranges
}

fn main() {
    let input = include_str!("../input.txt");
    let answer = solve_p1(input);
    println!("Part 1: {answer}");
    let answer = solve_p2(input);
    println!("Part 2: {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 2);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 4);
    }
}
