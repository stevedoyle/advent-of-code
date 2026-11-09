use aoc2015::*;

const DAY: u8 = 3;

fn parse_input(input: &str) -> String {
    input.lines().flat_map(str::chars).collect()
}

fn solve_p1(input: &str) -> usize {
    let data = parse_input(input);
    let mut visited = std::collections::HashSet::new();
    let mut santa = Point::ORIGIN;
    visited.insert(santa);
    for c in data.chars() {
        match c {
            '>' => santa += Point::RIGHT,
            '<' => santa += Point::LEFT,
            '^' => santa += Point::UP,
            'v' => santa += Point::DOWN,
            _ => {}
        }
        visited.insert(santa);
    }
    visited.len()
}

fn solve_p2(input: &str) -> usize {
    let data = parse_input(input);
    let mut visited = std::collections::HashSet::new();
    let mut santa = Point::ORIGIN;
    let mut robo = Point::ORIGIN;
    visited.insert(santa);
    for (i, c) in data.chars().enumerate() {
        let current = if i % 2 == 0 { &mut santa } else { &mut robo };
        match c {
            '>' => *current += Point::RIGHT,
            '<' => *current += Point::LEFT,
            '^' => *current += Point::UP,
            'v' => *current += Point::DOWN,
            _ => {}
        }
        visited.insert(*current);
    }
    visited.len()
}

fn main() {
    run(DAY, solve_p1, solve_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve_p1(">"), 2);
        assert_eq!(solve_p1("^>v<"), 4);
        assert_eq!(solve_p1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_p2("^v"), 3);
        assert_eq!(solve_p2("^>v<"), 3);
        assert_eq!(solve_p2("^v^v^v^v^v"), 11);
    }
}
