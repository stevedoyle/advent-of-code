use aoc2020::*;

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn solve_p1(input: &str) -> usize {
    let _data = parse_input(input);
    // TODO: Implement Part 1
    0
}

fn solve_p2(input: &str) -> usize {
    let _data = parse_input(input);
    // TODO: Implement Part 2
    0
}

fn main() {
    let input = read_input(DAY_NUMBER);

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
        let input = read_test_input(DAY_NUMBER);
        let answer = solve_p1(&input);
        assert_eq!(answer, EXPECTED_P1);
        let answer = solve_p2(&input);
        assert_eq!(answer, EXPECTED_P2);
    }
}
