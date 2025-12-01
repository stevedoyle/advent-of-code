use aoc2025::*;

const CIRCLE_SIZE: i32 = 100;
const START_POS: i32 = 50;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        }
    }
}

fn parse_input(input: &str) -> Vec<(Direction, i32)> {
    input
        .lines()
        .map(|line| {
            let (dir, dist) = line.split_at(1);
            let direction = Direction::from_char(dir.chars().next().expect("Empty line"))
                .expect("Invalid direction");
            let distance = dist.trim().parse::<i32>().expect("Invalid distance");
            (direction, distance)
        })
        .collect()
}

fn solve_p1(input: &str) -> i32 {
    let rotations = parse_input(input);
    let mut at_zero = 0;
    let mut pos = START_POS;

    for (dir, dist) in rotations.iter() {
        pos = match dir {
            Direction::Left => (pos - dist).rem_euclid(CIRCLE_SIZE),
            Direction::Right => (pos + dist).rem_euclid(CIRCLE_SIZE),
        };
        if pos == 0 {
            at_zero += 1;
        }
    }
    at_zero
}

fn count_zero_crossings(start_pos: i32, distance: i32, is_left: bool) -> (i32, i32) {
    let finish_pos;
    let mut crossings = 0;

    if is_left {
        finish_pos = (start_pos - distance).rem_euclid(CIRCLE_SIZE);
        if start_pos == 0 {
            crossings = distance / CIRCLE_SIZE;
        } else {
            crossings = (distance - start_pos + CIRCLE_SIZE) / CIRCLE_SIZE;
        }
    } else {
        finish_pos = (start_pos + distance).rem_euclid(CIRCLE_SIZE);
        crossings += (start_pos + distance) / CIRCLE_SIZE;
    };

    (finish_pos, crossings)
}

fn solve_p2(input: &str) -> i32 {
    let rotations = parse_input(input);
    let mut zero_crossings = 0;
    let mut pos = START_POS;

    for (dir, dist) in rotations.iter() {
        let (new_pos, crossings) = match dir {
            Direction::Left => count_zero_crossings(pos, *dist, true),
            Direction::Right => count_zero_crossings(pos, *dist, false),
        };
        pos = new_pos;
        zero_crossings += crossings;
    }
    zero_crossings
}

fn main() {
    let input = read_input(1);

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
        let input = read_test_input(1);
        let answer = solve_p1(&input);
        assert_eq!(answer, 3);
        let answer = solve_p2(&input);
        assert_eq!(answer, 6);
    }
}
