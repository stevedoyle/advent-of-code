use anyhow::Result;
use regex::Regex;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Plan {
    dir: Direction,
    distance: usize,
}

impl From<&str> for Plan {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"([RLUD]) (\d+) ").unwrap();
        let captures = re.captures(value).unwrap();
        Plan {
            dir: Direction::from(captures.get(1).unwrap().as_str()),
            distance: captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Default, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("Invalid direction {value}"),
        }
    }
}

fn parse_input_p1(input: &str) -> Vec<Plan> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Plan::from(line.trim()))
        .collect()
}

fn parse_plan_p2(value: &str) -> Plan {
    let re = Regex::new(r"#([\dabcdefABCDEF]{6})").unwrap();
    let captures = re.captures(value).unwrap();
    parse_hexcode(captures.get(1).unwrap().as_str())
}

fn parse_input_p2(input: &str) -> Vec<Plan> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_plan_p2(line.trim()))
        .collect()
}

const fn dig(curr: (isize, isize), dir: Direction, steps: usize) -> (isize, isize) {
    match dir {
        Direction::Up => (curr.0, curr.1 - steps as isize),
        Direction::Down => (curr.0, curr.1 + steps as isize),
        Direction::Right => (curr.0 + steps as isize, curr.1),
        Direction::Left => (curr.0 - steps as isize, curr.1),
    }
}

fn dig_and_calc_area(plans: &[Plan]) -> isize {
    let mut curr = (0, 0);
    let mut vertices = vec![curr];
    let mut boundary: isize = 1;
    for plan in plans {
        let steps = plan.distance;
        curr = dig(curr, plan.dir, plan.distance);
        vertices.push(curr);
        boundary += steps as isize;
    }

    // Shoelace formula
    // https://www.themathdoctors.org/polygon-coordinates-and-areas/
    let area: isize = vertices
        .iter()
        .zip(vertices.iter().cycle().skip(1))
        .map(|(a, b)| a.0 * b.1 - a.1 * b.0)
        .sum();
    area.abs() / 2 + boundary / 2 + 1
}

fn parse_hexcode(code: &str) -> Plan {
    let direction = <usize>::from_str_radix(&code[5..6], 16).unwrap();
    let distance = <usize>::from_str_radix(&code[0..5], 16).unwrap();
    Plan {
        dir: match direction {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => unreachable!(),
        },
        distance,
    }
}

fn solve_p1(input: &str) -> isize {
    let plans = parse_input_p1(input);
    dig_and_calc_area(&plans)
}

fn solve_p2(input: &str) -> isize {
    let plans = parse_input_p2(input);
    dig_and_calc_area(&plans)
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let answer = solve_p1(input);
    println!("Part 1: {answer}");

    let answer = solve_p2(input);
    println!("Part 2: {answer}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)";

    #[test]
    fn test_parse_input() {
        let plans = parse_input_p1(INPUT);
        assert_eq!(
            plans[0],
            Plan {
                dir: Direction::Right,
                distance: 6,
            }
        );
        assert_eq!(
            plans[plans.len() - 1],
            Plan {
                dir: Direction::Up,
                distance: 2,
            }
        );
    }

    #[test]
    fn test_parse_hexcode() {
        let code = "70c710";
        let plan = parse_hexcode(code);
        assert_eq!(plan.dir, Direction::Right);
        assert_eq!(plan.distance, 461937);
    }

    #[test]
    fn test_solve_with_test_input() {
        let plans = parse_input_p1(INPUT);
        let answer = dig_and_calc_area(&plans);
        assert_eq!(answer, 62);

        let plans = parse_input_p2(INPUT);
        let answer = dig_and_calc_area(&plans);
        assert_eq!(answer, 952408144115);
    }

    #[test]
    fn test_solve() {
        let input = include_str!("../input.txt");
        let answer = solve_p1(input);
        assert_eq!(answer, 36725);

        let answer = solve_p2(input);
        assert_eq!(answer, 97874103749720);
    }
}
