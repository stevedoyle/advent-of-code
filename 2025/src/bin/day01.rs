use aoc2025::*;

const CIRCLE_SIZE: i32 = 100;
const START_POS: i32 = 50;

fn parse_input(input: &str) -> Vec<(char, i32)> {
    input
        .lines()
        .map(|line| {
            let direction = line.chars().next().expect("Empty line");
            let distance = line[1..].trim().parse::<i32>().expect("Invalid distance");
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
            'L' => (pos - dist).rem_euclid(CIRCLE_SIZE),
            'R' => (pos + dist).rem_euclid(CIRCLE_SIZE),
            _ => panic!("Invalid direction"),
        };
        if pos == 0 {
            at_zero += 1;
        }
    }
    at_zero
}

fn solve_p2(input: &str) -> i32 {
    let rotations = parse_input(input);
    let mut zero_crossings = 0;
    let mut pos = START_POS;

    for (dir, distance) in rotations.iter() {
        let (new_pos, crossings) = match dir {
            'L' => {
                let finish_pos = (pos - distance).rem_euclid(CIRCLE_SIZE);
                let crossings = if pos == 0 {
                    distance / CIRCLE_SIZE
                } else {
                    (distance - pos + CIRCLE_SIZE) / CIRCLE_SIZE
                };
                (finish_pos, crossings)
            }
            'R' => {
                let finish_pos = (pos + distance).rem_euclid(CIRCLE_SIZE);
                let crossings = (pos + distance) / CIRCLE_SIZE;
                (finish_pos, crossings)
            }
            _ => panic!("Invalid direction"),
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
