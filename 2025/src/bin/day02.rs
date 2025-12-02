use core::str;

use aoc2025::*;

fn parse_input(input: &str) -> Vec<(String, String)> {
    // Input is in the form of lines with multiple ranges like "2-4,6-8"
    input
        .lines()
        .flat_map(|line| {
            line.split(',').map(|r| {
                let (start, end) = r.trim().split_once('-').expect("Invalid range");
                (start.to_string(), end.to_string())
            })
        })
        .collect()
}

fn get_simple_invalid_ids(start: &str, end: &str) -> Vec<usize> {
    // An ID is invalid if it is of the form ABCABC (i.e. the first half equals the second half)
    // The range is inclusive and the IDs are strings of digits

    // Compute the possible invalid IDs in the range
    let start_num: usize = start.parse().expect("Invalid start ID");
    let end_num: usize = end.parse().expect("Invalid end ID");
    let mut invalid_ids = Vec::new();
    for id in start_num..=end_num {
        let id_str = id.to_string();
        let len = id_str.len();
        if len % 2 == 0 {
            let (first_half, second_half) = id_str.split_at(len / 2);
            if first_half == second_half {
                invalid_ids.push(id);
            }
        }
    }
    invalid_ids
}

fn get_invalid_ids(start: &str, end: &str) -> Vec<usize> {
    // An ID is invalid if it is constructed from any repeated sequence of digits
    // The range is inclusive and the IDs are strings of digits

    // Compute the possible invalid IDs in the range
    let start_num: usize = start.parse().expect("Invalid start ID");
    let end_num: usize = end.parse().expect("Invalid end ID");
    let mut invalid_ids = Vec::new();

    for id in start_num..=end_num {
        let id_str = id.to_string();
        let len = id_str.len();

        // Check all possible pattern lengths that divide evenly into the ID length
        for pattern_len in 1..len {
            if len % pattern_len == 0 {
                let pattern = &id_str[0..pattern_len];
                let repetitions = len / pattern_len;

                // Check if the entire ID is made of this pattern repeated
                let repeated = pattern.repeat(repetitions);
                if repeated == id_str {
                    invalid_ids.push(id);
                    break; // Found a valid pattern, no need to check further
                }
            }
        }
    }
    invalid_ids
}

fn solve_p1(input: &str) -> usize {
    let ranges = parse_input(input);
    ranges
        .iter()
        .map(|(start, end)| get_simple_invalid_ids(start, end))
        .flatten()
        .sum()
}

fn solve_p2(input: &str) -> usize {
    let ranges = parse_input(input);
    ranges
        .iter()
        .map(|(start, end)| get_invalid_ids(start, end))
        .flatten()
        .sum()
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
        assert_eq!(answer, 1227775554);
        let answer = solve_p2(&input);
        assert_eq!(answer, 4174379265);
    }

    #[test]
    fn test_simple_invalid_ids() {
        assert_eq!(get_simple_invalid_ids("1212", "1212").len(), 1);
        assert_eq!(get_simple_invalid_ids("1234", "1234").len(), 0);
        assert_eq!(get_simple_invalid_ids("1111", "2222").len(), 12); // 1111, 1212, 1313, ..., 1919, 2020, 2121
        assert_eq!(get_simple_invalid_ids("11", "22").len(), 2);
        assert_eq!(get_simple_invalid_ids("95", "115").len(), 1);
        assert_eq!(get_simple_invalid_ids("1188511880", "1188511890").len(), 1);
        assert_eq!(get_simple_invalid_ids("222220", "222224").len(), 1);
        assert_eq!(get_simple_invalid_ids("1698522", "1698528").len(), 0);
    }

    #[test]
    fn test_invalid_ids() {
        assert_eq!(get_invalid_ids("11", "22").len(), 2);
        assert_eq!(get_invalid_ids("95", "115").len(), 2);
        assert_eq!(get_invalid_ids("1188511880", "1188511890").len(), 1);
        assert_eq!(get_invalid_ids("222220", "222224").len(), 1);
        assert_eq!(get_invalid_ids("1698522", "1698528").len(), 0);
        assert_eq!(get_invalid_ids("2121212118", "2121212124").len(), 1);
    }
}
