use std::collections::HashMap;

use aoc2020::*;

const DAY: u8 = 15;

fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn play_game(starting_numbers: &[usize], target_turn: usize) -> usize {
    let mut last_seen: HashMap<usize, usize> = HashMap::new();

    for (i, &num) in starting_numbers
        .iter()
        .enumerate()
        .take(starting_numbers.len() - 1)
    {
        last_seen.insert(num, i + 1);
    }

    let mut current = *starting_numbers.last().unwrap();

    for turn in starting_numbers.len()..target_turn {
        let next = if let Some(&prev_turn) = last_seen.get(&current) {
            turn - prev_turn
        } else {
            0
        };

        last_seen.insert(current, turn);
        current = next;
    }

    current
}

fn solve_p1(input: &str) -> usize {
    let starting_numbers = parse_input(input);
    let target_turn = 2020;
    play_game(&starting_numbers, target_turn)
}

fn solve_p2(input: &str) -> usize {
    let starting_numbers = parse_input(input);
    let target_turn = 30_000_000;
    play_game(&starting_numbers, target_turn)
}

fn main() {
    run(DAY, solve_p1, solve_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve_p1("0,3,6"), 436);
        assert_eq!(solve_p1("1,3,2"), 1);
        assert_eq!(solve_p1("2,1,3"), 10);
        assert_eq!(solve_p1("1,2,3"), 27);
        assert_eq!(solve_p1("2,3,1"), 78);
        assert_eq!(solve_p1("3,2,1"), 438);
        assert_eq!(solve_p1("3,1,2"), 1836);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_p2("0,3,6"), 175594);
        assert_eq!(solve_p2("1,3,2"), 2578);
        assert_eq!(solve_p2("2,1,3"), 3544142);
        assert_eq!(solve_p2("1,2,3"), 261214);
        assert_eq!(solve_p2("2,3,1"), 6895259);
        assert_eq!(solve_p2("3,2,1"), 18);
        assert_eq!(solve_p2("3,1,2"), 362);
    }
}
