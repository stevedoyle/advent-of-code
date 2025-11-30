use std::collections::HashSet;

use aoc2020::*;

fn parse_input(input: &str) -> Vec<HashSet<char>> {
    let group: Vec<&str> = input.split("\n\n").collect();
    group
        .into_iter()
        .map(|g| g.lines().flat_map(|line| line.trim().chars()).collect())
        .collect()
}

fn parse_groups(input: &str) -> Vec<HashSet<&str>> {
    let group: Vec<&str> = input.split("\n\n").collect();
    group
        .into_iter()
        .map(|g| g.lines().flat_map(|line| line.split_whitespace()).collect())
        .collect()
}

fn solve_p1(input: &str) -> usize {
    let data = parse_input(input);
    data.iter().map(|s| s.len()).sum()
}

fn solve_p2(input: &str) -> usize {
    let groups = parse_groups(input);
    groups
        .iter()
        .map(|group| {
            let mut iter = group.iter();
            if let Some(first) = iter.next() {
                iter.fold(first.chars().collect::<HashSet<char>>(), |acc, person| {
                    acc.intersection(&person.chars().collect())
                        .cloned()
                        .collect()
                })
                .len()
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let input = read_input(6);

    let start = std::time::Instant::now();
    let answer = solve_p1(&input);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:.1?}");

    let start = std::time::Instant::now();
    let answer = solve_p2(&input);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, elapsed: {elapsed:.1?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_with_test_input() {
        let input = read_test_input(6);
        let answer = solve_p1(&input);
        assert_eq!(answer, 11);
        let answer = solve_p2(&input);
        assert_eq!(answer, 6);
    }
}
