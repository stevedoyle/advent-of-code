use std::{collections::HashMap, time::Instant};

fn parse_input(input: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let mut rules = HashMap::new();

    let (rule_input, page_input) = input.split_once("\n\n").unwrap();
    for line in rule_input.lines() {
        if let Some((key, value)) = line.split_once("|") {
            let key = key.parse().unwrap();
            let value = value.parse().unwrap();
            rules.entry(key).or_insert(vec![]).push(value);
        }
    }
    let updates = page_input
        .lines()
        .map(|line| line.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();
    (rules, updates)
}

fn sort_by_rules(rules: &HashMap<usize, Vec<usize>>, data: &mut [usize]) {
    data.sort_by(|a, b| match rules.get(a) {
        None => std::cmp::Ordering::Greater,
        Some(v) => {
            if v.contains(b) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        }
    });
}

fn solve_p1(rules: &HashMap<usize, Vec<usize>>, updates: &[Vec<usize>]) -> usize {
    let mut in_order = Vec::new();
    for item in updates {
        let mut adjusted = item.clone();
        sort_by_rules(rules, &mut adjusted);
        if *item == adjusted {
            in_order.push(adjusted);
        }
    }
    in_order.iter().map(|u| u[u.len() / 2]).sum()
}

fn solve_p2(rules: &HashMap<usize, Vec<usize>>, updates: &[Vec<usize>]) -> usize {
    let mut reordered = Vec::new();
    for item in updates {
        let mut adjusted = item.clone();
        sort_by_rules(rules, &mut adjusted);
        if *item != adjusted {
            reordered.push(adjusted);
        }
    }
    reordered.iter().map(|u| u[u.len() / 2]).sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let (rules, updates) = parse_input(input);

    let start = Instant::now();
    let answer = solve_p1(&rules, &updates);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:0.1?}");

    let start = Instant::now();
    let answer = solve_p2(&rules, &updates);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, elapsed: {elapsed:0.1?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_solve_with_test_input() {
        let (rules, updates) = parse_input(INPUT);
        let answer = solve_p1(&rules, &updates);
        assert_eq!(answer, 143);
        let answer = solve_p2(&rules, &updates);
        assert_eq!(answer, 123);
    }
}
