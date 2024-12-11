use std::collections::HashMap;

use count_digits::CountDigits;

fn parse_input(input: &str) -> HashMap<usize, usize> {
    input
        .split_whitespace()
        .map(|s| (s.parse().unwrap(), 1))
        .collect()
}

fn split_stone(stone: usize, size: u32) -> Vec<usize> {
    let p = 10usize.pow(size);
    vec![stone / p, stone % p]
}

fn process_stone(stone: usize) -> Vec<usize> {
    if stone == 0 {
        return vec![1];
    }
    let num_digits = stone.count_digits() as u32;
    if num_digits % 2 == 0 {
        return split_stone(stone, num_digits / 2).to_vec();
    }
    vec![stone * 2024]
}

fn solve(input: &str, blinks: usize) -> usize {
    let mut stones = parse_input(input);

    (0..blinks).for_each(|_| {
        for (stone, n) in stones.drain().collect::<Vec<(usize, usize)>>() {
            process_stone(stone).iter().for_each(|&s| {
                stones.entry(s).and_modify(|s| *s += n).or_insert(n);
            });
        }
    });
    stones.values().sum()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let start = std::time::Instant::now();
    let answer = solve(&input, 25);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:.1?}");

    let start = std::time::Instant::now();
    let answer = solve(&input, 75);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, elapsed: {elapsed:.1?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_with_test_input() {
        let input = "0 1 10 99 999".to_string();
        let answer = solve(&input, 1);
        assert_eq!(answer, 7);
        let input = "125 17".to_string();
        let answer = solve(&input, 25);
        assert_eq!(answer, 55312);
    }
}
