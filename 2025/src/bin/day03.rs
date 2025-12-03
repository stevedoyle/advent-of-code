use aoc2025::*;

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input.lines().map(digit_str_to_vec).collect()
}

fn digit_str_to_vec(s: &str) -> Vec<usize> {
    s.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn max_pairs(bank: &[usize]) -> usize {
    let mut first_digit = bank[0];
    let mut first_digit_index = 0;

    for (i, digit) in bank.iter().enumerate().take(bank.len() - 1).skip(1) {
        if *digit > first_digit {
            first_digit = *digit;
            first_digit_index = i;
        }
    }

    let mut second_digit = bank[first_digit_index + 1];
    for &digit in &bank[first_digit_index + 2..] {
        if digit > second_digit {
            second_digit = digit;
        }
    }
    first_digit * 10 + second_digit
}

fn max_n_digits(bank: &[usize], n: usize) -> usize {
    let mut digits: Vec<usize> = Vec::new();
    let mut digit_indices: Vec<usize> = Vec::new();
    let mut curr_digit_idx = 0;

    (0..n).for_each(|i| {
        let mut max_digit = 0;
        let mut max_digit_index = 0;
        for (j, digit) in bank
            .iter()
            .enumerate()
            .take(bank.len() - (n - i - 1))
            .skip(curr_digit_idx)
        {
            if *digit > max_digit {
                max_digit = *digit;
                max_digit_index = j;
            }
        }
        digits.push(max_digit);
        digit_indices.push(max_digit_index);
        curr_digit_idx = max_digit_index + 1;
    });
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
