use aoc2020::*;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    parse_grid(input)
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
    let input = read_input(3);

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
        let input = read_test_input(3);
        let answer = solve_p1(&input);
        assert_eq!(answer, 0);
        let answer = solve_p2(&input);
        assert_eq!(answer, 0);
    }
}
