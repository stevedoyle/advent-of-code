enum Direction {
    Forward,
    Down,
    Up,
}

struct Command {
    direction: Direction,
    value: i32,
}

fn parse_input(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let direction = match parts.next().unwrap() {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => panic!("Unknown direction"),
            };
            let value = parts.next().unwrap().parse().unwrap();
            Command { direction, value }
        })
        .collect()
}

fn solve_p1(input: &str) -> i32 {
    let commands = parse_input(input);
    let mut horizontal = 0;
    let mut depth = 0;
    for command in commands {
        match command.direction {
            Direction::Forward => horizontal += command.value,
            Direction::Down => depth += command.value,
            Direction::Up => depth -= command.value,
        }
    }
    horizontal * depth
}

fn solve_p2(input: &str) -> i32 {
    let commands = parse_input(input);
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in commands {
        match command.direction {
            Direction::Forward => {
                horizontal += command.value;
                depth += aim * command.value;
            }
            Direction::Down => aim += command.value,
            Direction::Up => aim -= command.value,
        }
    }
    horizontal * depth
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

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

    const TEST_INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(TEST_INPUT);
        assert_eq!(answer, 150);
        let answer = solve_p2(TEST_INPUT);
        assert_eq!(answer, 900);
    }
}
