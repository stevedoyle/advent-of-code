use aoc2020::*;

struct PasswordPolicy {
    min: usize,
    max: usize,
    letter: char,
}

fn parse_input(input: &str) -> Vec<(PasswordPolicy, String)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let policy_part = parts[0].trim();
            let password = parts[1].trim().to_string();

            let policy_parts: Vec<&str> = policy_part.split_whitespace().collect();
            let range_part = policy_parts[0];
            let letter_part = policy_parts[1];

            let range_bounds: Vec<&str> = range_part.split('-').collect();
            let min = range_bounds[0].parse::<usize>().unwrap();
            let max = range_bounds[1].parse::<usize>().unwrap();
            let letter = letter_part.chars().next().unwrap();

            (PasswordPolicy { min, max, letter }, password)
        })
        .collect()
}

fn solve_p1(input: &str) -> i32 {
    let passwords = parse_input(input);
    let mut valid_count = 0;
    for (policy, password) in passwords {
        let letter_count = password.chars().filter(|&c| c == policy.letter).count();
        if letter_count >= policy.min && letter_count <= policy.max {
            valid_count += 1;
        }
    }
    valid_count
}

fn solve_p2(input: &str) -> i32 {
    let passwords = parse_input(input);
    let mut valid_count = 0;
    for (policy, password) in passwords {
        let first_pos = policy.min - 1;
        let second_pos = policy.max - 1;
        let first_match = password.chars().nth(first_pos) == Some(policy.letter);
        let second_match = password.chars().nth(second_pos) == Some(policy.letter);
        if first_match ^ second_match {
            valid_count += 1;
        }
    }
    valid_count
}

fn main() {
    let input = read_input(2);

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
        let input = read_test_input(2);
        let answer = solve_p1(&input);
        assert_eq!(answer, 2);
        let answer = solve_p2(&input);
        assert_eq!(answer, 1);
    }
}
