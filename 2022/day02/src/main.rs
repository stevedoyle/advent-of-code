use std::collections::HashMap;

fn solve_p1(input: &str) -> usize {
    let strategy = HashMap::from([
        ("A X", 1 + 3),
        ("A Y", 2 + 6),
        ("A Z", 3 + 0),
        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 1 + 6),
        ("C Y", 2 + 0),
        ("C Z", 3 + 3),
    ]);

    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| strategy.get(line.trim()).unwrap())
        .sum()
}

fn solve_p2(input: &str) -> usize {
    let strategy = HashMap::from([
        ("A X", 3 + 0),
        ("A Y", 1 + 3),
        ("A Z", 2 + 6),
        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 2 + 0),
        ("C Y", 3 + 3),
        ("C Z", 1 + 6),
    ]);

    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| strategy.get(line.trim()).unwrap())
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

    const INPUT: &str = "A Y
    B X
    C Z";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 15);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 12);
    }
}
