use aoc2026::*;

const DAY: u8 = {{DAY}};

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn solve_p1(input: &str) -> usize {
    let _data = parse_input(input);
    0
}

fn solve_p2(input: &str) -> usize {
    let _data = parse_input(input);
    0
}

fn main() {
    run(DAY, solve_p1, solve_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve_p1(&read_test_input(DAY, 1)), 0);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_p2(&read_test_input(DAY, 1)), 0);
    }
}
