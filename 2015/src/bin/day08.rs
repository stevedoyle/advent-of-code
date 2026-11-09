use aoc2015::*;

const DAY: u8 = 8;

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().map(str::trim).collect()
}

fn count_code_chars(s: &str) -> usize {
    s.len()
}

fn count_memory_chars(s: &str) -> usize {
    let mut count = 0;
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(next) = chars.next() {
                    match next {
                        '\\' | '"' => count += 1,
                        'x' => {
                            chars.next();
                            chars.next();
                            count += 1;
                        }
                        _ => {}
                    }
                }
            }
            '"' => {}
            _ => count += 1,
        }
    }
    count
}

fn encode(s: &str) -> String {
    let mut encoded = String::new();
    encoded.push('"');
    for c in s.chars() {
        match c {
            '\\' => encoded.push_str("\\\\"),
            '"' => encoded.push_str("\\\""),
            _ => encoded.push(c),
        }
    }
    encoded.push('"');
    encoded
}

fn solve_p1(input: &str) -> usize {
    let strs = parse_input(input);
    strs.iter()
        .map(|s| count_code_chars(s) - count_memory_chars(s))
        .sum()
}

fn solve_p2(input: &str) -> usize {
    let data = parse_input(input);
    data.iter().map(|s| encode(s).len() - s.len()).sum()
}

fn main() {
    run(DAY, solve_p1, solve_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(count_code_chars(r#""""#), 2);
        assert_eq!(count_memory_chars(r#""""#), 0);
        assert_eq!(solve_p1(r#""""#), 2);
        assert_eq!(solve_p1(&read_test_input(DAY, 1)), 12);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_p2(&read_test_input(DAY, 1)), 19);
    }
}
