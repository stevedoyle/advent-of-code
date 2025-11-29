use std::fs;

/// Read input file for a given day
pub fn read_input(day: u8) -> String {
    let path = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", path))
}

/// Read test input file for a given day
pub fn read_test_input(day: u8) -> String {
    let path = format!("inputs/day{:02}_test.txt", day);
    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read test input file: {}", path))
}

/// Parse lines into a vector of type T
pub fn parse_lines<T>(input: &str) -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(|line| line.parse().expect("Failed to parse line"))
        .collect()
}

/// Parse a grid of characters
pub fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

/// Parse a grid of digits
pub fn parse_digit_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Invalid digit"))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let input = "1\n2\n3";
        let result: Vec<i32> = parse_lines(input);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_parse_grid() {
        let input = "abc\ndef";
        let result = parse_grid(input);
        assert_eq!(result, vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]);
    }
}
