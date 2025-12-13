use aoc2020::*;

struct Instruction {
    action: char,
    value: i32,
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let action = line.chars().next().unwrap();
            let value = line[1..].parse().unwrap();
            Instruction { action, value }
        })
        .collect()
}

fn solve_p1(input: &str) -> usize {
    let instructions = parse_input(input);
    let start = (0, 0);
    let mut position = start;
    let mut direction = Direction::East;

    instructions.iter().for_each(|inst| match inst.action {
        'N' => position.1 += inst.value,
        'S' => position.1 -= inst.value,
        'E' => position.0 += inst.value,
        'W' => position.0 -= inst.value,
        'L' => {
            if inst.value % 90 != 0 {
                panic!("Invalid turn value: {}", inst.value);
            }
            let turns = (inst.value / 90) % 4;
            for _ in 0..turns {
                direction = match direction {
                    Direction::North => Direction::West,
                    Direction::West => Direction::South,
                    Direction::South => Direction::East,
                    Direction::East => Direction::North,
                }
            }
        }
        'R' => {
            let turns = (inst.value / 90) % 4;
            for _ in 0..turns {
                direction = match direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                }
            }
        }
        'F' => match direction {
            Direction::North => position.1 += inst.value,
            Direction::South => position.1 -= inst.value,
            Direction::East => position.0 += inst.value,
            Direction::West => position.0 -= inst.value,
        },
        _ => {}
    });

    (position.0.abs() + position.1.abs()) as usize
}

fn solve_p2(input: &str) -> usize {
    let instructions = parse_input(input);
    let mut ship = (0, 0);
    let mut waypoint = (10, 1);

    instructions.iter().for_each(|inst| match inst.action {
        'N' => waypoint.1 += inst.value,
        'S' => waypoint.1 -= inst.value,
        'E' => waypoint.0 += inst.value,
        'W' => waypoint.0 -= inst.value,
        'L' => {
            let turns = (inst.value / 90) % 4;
            for _ in 0..turns {
                waypoint = (-waypoint.1, waypoint.0);
            }
        }
        'R' => {
            let turns = (inst.value / 90) % 4;
            for _ in 0..turns {
                waypoint = (waypoint.1, -waypoint.0);
            }
        }
        'F' => {
            ship.0 += waypoint.0 * inst.value;
            ship.1 += waypoint.1 * inst.value;
        }
        _ => {}
    });

    (ship.0.abs() + ship.1.abs()) as usize
}

fn main() {
    let input = read_input(12);

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
        let input = read_test_input(12);
        let answer = solve_p1(&input);
        assert_eq!(answer, 25);
        let answer = solve_p2(&input);
        assert_eq!(answer, 286);
    }
}
