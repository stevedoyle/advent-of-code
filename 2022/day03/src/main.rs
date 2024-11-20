use itertools::Itertools;
use std::collections::HashSet;

fn solve_p1(input: &str) -> i32 {
    let mut misplaced = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let (first, second) = line.split_at(line.len() / 2);
        let first_set: HashSet<char> = HashSet::from_iter(first.chars());
        let second_set: HashSet<char> = HashSet::from_iter(second.chars());
        misplaced.extend(first_set.intersection(&second_set));
    }

    misplaced.iter().map(|c| priority(*c)).sum()
}

fn solve_p2(input: &str) -> i32 {
    let mut badges = Vec::new();

    for (l1, l2, l3) in input.lines().tuples() {
        let l1_set: HashSet<char> = HashSet::from_iter(l1.chars());
        let l2_set: HashSet<char> = HashSet::from_iter(l2.chars());
        let l3_set: HashSet<char> = HashSet::from_iter(l3.chars());
        badges.extend(
            l1_set
                .intersection(&l2_set)
                .cloned()
                .collect::<HashSet<_>>()
                .intersection(&l3_set)
                .cloned(),
        );
    }

    badges.iter().map(|c| priority(*c)).sum()
}

fn priority(c: char) -> i32 {
    match c {
        'A'..='Z' => 27 + (c as i32) - ('A' as i32),
        'a'..='z' => 1 + (c as i32) - ('a' as i32),
        _ => 0,
    }
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

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
A Y";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 157);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 70);
    }
}
