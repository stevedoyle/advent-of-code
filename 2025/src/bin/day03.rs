use aoc2025::*;

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input.lines().map(digit_str_to_vec).collect()
}

fn digit_str_to_vec(s: &str) -> Vec<usize> {
    s.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn max_digit_and_index(bank: &[usize]) -> (usize, usize) {
    let mut max_digit = 0;
    let mut max_digit_index = 0;
    for (i, &digit) in bank.iter().enumerate() {
        // max_by_key doesn't work since we need the index of the first max digit in case of ties
        if digit > max_digit {
            max_digit = digit;
            max_digit_index = i;
        }
    }
    (max_digit, max_digit_index)
}

fn max_pairs(bank: &[usize]) -> usize {
    max_n_digits(bank, 2)
}

fn max_n_digits(bank: &[usize], n: usize) -> usize {
    let mut digits: Vec<usize> = Vec::new();
    let mut curr_digit_idx = 0;

    for i in 0..n {
        let (max_digit, max_digit_index) =
            max_digit_and_index(&bank[curr_digit_idx..bank.len() - (n - i - 1)]);
        digits.push(max_digit);
        curr_digit_idx = curr_digit_idx + max_digit_index + 1;
    }
    digits.iter().fold(0, |acc, &d| acc * 10 + d)
}

fn solve_p1(input: &str) -> usize {
    let banks = parse_input(input);
    banks.iter().map(|bank| max_pairs(bank)).sum()
}

fn solve_p2(input: &str) -> usize {
    let banks = parse_input(input);
    banks.iter().map(|bank| max_n_digits(bank, 12)).sum()
}

fn main() {
    let input = read_input(3);

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
        let input = read_test_input(3);
        let answer = solve_p1(&input);
        assert_eq!(answer, 357);
        let answer = solve_p2(&input);
        assert_eq!(answer, 3121910778619);
    }

    #[test]
    fn test_max_pairs() {
        let data = digit_str_to_vec("987654321111111");
        assert_eq!(max_pairs(&data), 98);
        assert_eq!(max_n_digits(&data, 2), 98);
        assert_eq!(max_n_digits(&data, 12), 987654321111);
    }
}
