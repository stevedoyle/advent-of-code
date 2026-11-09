use aoc2015::*;

const DAY: u8 = 3;

fn parse_input(input: &str) -> String {
    input.lines().flat_map(str::chars).collect()
}

fn solve_p1(input: &str) -> usize {
    let data = parse_input(input);
    let mut visited = std::collections::HashSet::new();
    let mut x = 0;
    let mut y = 0;
    visited.insert((x, y));
    for c in data.chars() {
        match c {
            '>' => x += 1,
            '<' => x -= 1,
            '^' => y += 1,
            'v' => y -= 1,
            _ => {}
        }
        visited.insert((x, y));
    }
    visited.len()
}

fn solve_p2(input: &str) -> usize {
    let data = parse_input(input);
    let mut visited = std::collections::HashSet::new();
    let mut x_santa = 0;
    let mut y_santa = 0;
    let mut x_robo = 0;
    let mut y_robo = 0;
    visited.insert((0, 0));
    for (i, c) in data.chars().enumerate() {
        let (x, y) = if i % 2 == 0 {
            (&mut x_santa, &mut y_santa)
        } else {
            (&mut x_robo, &mut y_robo)
        };
        match c {
            '>' => *x += 1,
            '<' => *x -= 1,
            '^' => *y += 1,
            'v' => *y -= 1,
            _ => {}
        }
        visited.insert((*x, *y));
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
