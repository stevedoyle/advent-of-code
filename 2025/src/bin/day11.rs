use pathfinding::prelude::count_paths;
use std::collections::HashMap;

use aoc2025::*;

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut data = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").map(|s| s.trim()).collect();
        if parts.len() == 2 {
            let key = parts[0].to_string();
            let values: Vec<String> = parts[1].split(' ').map(|s| s.trim().to_string()).collect();
            data.insert(key, values);
        }
    }

    data
}

fn solve_p1(input: &str) -> usize {
    let devices = parse_input(input);
    let start = "you".to_string();
    let target = "out".to_string();

    count_paths(
        &start,
        |from| devices.get(*from).unwrap(),
        |node| **node == target,
    )
}

fn solve_p2(input: &str) -> usize {
    let mut devices = parse_input(input);
    let start = "svr".to_string();
    let target = "out".to_string();
    devices.insert(target.clone(), vec![]); // Ensure target is in the map

    count_paths(
        (&start, false, false),
        |from| {
            let (current, has_dac, has_fft) = *from;
            devices.get(current).unwrap().iter().map(move |neighbor| {
                if neighbor == "dac" {
                    (neighbor, true, has_fft)
                } else if neighbor == "fft" {
                    (neighbor, has_dac, true)
                } else {
                    (neighbor, has_dac, has_fft)
                }
            })
        },
        |node| **node.0 == target && node.1 && node.2,
    )
}

fn main() {
    let input = read_input(11);

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
        let input = read_test_input(11);
        let answer = solve_p1(&input);
        assert_eq!(answer, 5);

        let input = read_test_input_2(11);
        let answer = solve_p2(&input);
        assert_eq!(answer, 2);
    }
}
