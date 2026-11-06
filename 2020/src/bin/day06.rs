use std::collections::HashSet;

use aoc2020::*;

const DAY: u8 = 6;

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
    run(DAY, solve_p1, solve_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve_p1(&read_test_input(DAY, 1)), 11);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_p2(&read_test_input(DAY, 1)), 6);
    }
}
