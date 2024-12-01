use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    input.lines().for_each(|line| {
        let mut iter = line.split_whitespace();
        left.push(iter.next().unwrap().parse().unwrap());
        right.push(iter.next().unwrap().parse().unwrap());
    });
    (left, right)
}

fn solve_p1(input: &str) -> i32 {
    let (mut left, mut right) = parse_input(input);
    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn solve_p2(input: &str) -> usize {
    let (left, mut right) = parse_input(input);
    right.sort_unstable();

    let mut counts = HashMap::new();

    left.iter().for_each(|&x| {
        let count = counts.entry(x).or_insert(0);
        *count += right.iter().filter(|&&r| r == x).count();
    });

    counts
        .iter()
        .map(|(num, count)| *count * *num as usize)
        .sum()
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

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 11);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 31);
    }
}
