use std::time::Instant;

fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn solve_p1(input: &str) -> i32 {
    let _data = parse_input(input);
    0
}

fn solve_p2(input: &str) -> i32 {
    let _data = parse_input(input);
    0
}

fn main() {
    let input = include_str!("../input.txt");

    let start = Instant::now();
    let answer = solve_p1(input);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, {elapsed:.2?}");

    let start = Instant::now();
    let answer = solve_p2(input);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, {elapsed:.2?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 0);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 0);
    }
}
