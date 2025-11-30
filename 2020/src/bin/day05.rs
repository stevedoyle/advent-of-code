use std::str::FromStr;

use aoc2020::*;

#[allow(dead_code)]
struct Seat {
    id: u32,
}

impl FromStr for Seat {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Seat, Self::Err> {
        let row = s[..7]
            .chars()
            .fold(0u32, |acc, c| (acc << 1) | if c == 'B' { 1 } else { 0 });
        let col = s[7..]
            .chars()
            .fold(0u32, |acc, c| (acc << 1) | if c == 'R' { 1 } else { 0 });
        Ok(Seat { id: row * 8 + col })
    }
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn solve_p1(input: &str) -> usize {
    let seats = parse_input(input);
    seats
        .into_iter()
        .map(|s| s.parse::<Seat>().unwrap().id as usize)
        .max()
        .unwrap()
}

fn solve_p2(input: &str) -> usize {
    let seat_ids = parse_input(input)
        .into_iter()
        .map(|s| s.parse::<Seat>().unwrap().id)
        .collect::<Vec<_>>();

    let mut seats = [false; 1024]; // max seat ID is 127*8+7 = 1023
    seat_ids.iter().for_each(|&id| seats[id as usize] = true);

    // Find first missing seat with occupied neighbors
    for i in 1..1023 {
        if !seats[i] && seats[i - 1] && seats[i + 1] {
            return i;
        }
    }
    0
}

fn main() {
    let input = read_input(5);

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
        let input = read_test_input(5);
        let answer = solve_p1(&input);
        assert_eq!(answer, 820);
    }

    #[test]
    fn test_seat_parsing() {
        let seat: Seat = "FBFBBFFRLR".parse().unwrap();
        assert_eq!(seat.id, 357);
    }
}
