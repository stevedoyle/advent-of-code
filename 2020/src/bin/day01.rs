use aoc2020::*;
use std::collections::HashSet;

const DAY: u8 = 1;

fn parse_input(input: &str) -> Vec<i32> {
    parse_lines(input)
}

fn solve_p1(input: &str) -> i32 {
    let data = parse_input(input);
    let target = 2020;
    let mut seen = HashSet::new();
    for &num in &data {
        let complement = target - num;
        if seen.contains(&complement) {
            return num * complement;
        }
        seen.insert(num);
    }
    0
}

fn solve_p2(input: &str) -> i32 {
    let data = parse_input(input);
    let target = 2020;
    let set: HashSet<i32> = data.iter().copied().collect();

    let len = data.len();
    for i in 0..len {
        for j in i + 1..len {
            let sum = data[i] + data[j];
            let complement = target - sum;
            // Check if complement exists and is different from the two we already picked
            if complement != data[i] && complement != data[j] && set.contains(&complement) {
                return data[i] * data[j] * complement;
            }
        }
    }
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
        assert_eq!(solve_p1(&read_test_input(DAY, 1)), 514579);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_p2(&read_test_input(DAY, 1)), 241861950);
    }
}
