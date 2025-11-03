#[derive(Debug)]
struct Entry {
    patterns: Vec<String>,
    output: Vec<String>,
}

fn parse_input(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" | ").collect();
            let patterns = parts[0]
                .split_whitespace()
                .map(|s| {
                    let mut chars: Vec<char> = s.chars().collect();
                    chars.sort();
                    chars.into_iter().collect()
                })
                .collect();
            let output = parts[1]
                .split_whitespace()
                .map(|s| {
                    let mut chars: Vec<char> = s.chars().collect();
                    chars.sort();
                    chars.into_iter().collect()
                })
                .collect();
            Entry { patterns, output }
        })
        .collect()
}

fn diff(a: &str, b: &str) -> String {
    a.chars().filter(|c| !b.contains(*c)).collect()
}

fn decode(patterns: &[String]) -> std::collections::HashMap<String, i32> {
    let mut pattern_map = std::collections::HashMap::new();
    let mut reverse_map = std::collections::HashMap::new();

    // Step 1: Identify digits with unique segment counts
    identify_unique_digits(patterns, &mut pattern_map, &mut reverse_map);

    // Step 2: Find digit 0 by testing middle segment candidates
    find_digit_zero(patterns, &mut pattern_map, &mut reverse_map);

    // Step 3: Decode remaining 6-segment digits (6, 9)
    decode_six_segment_digits(patterns, &mut pattern_map, &mut reverse_map);

    // Step 4: Decode 5-segment digits (2, 3, 5)
    decode_five_segment_digits(patterns, &mut pattern_map, &mut reverse_map);

    pattern_map
}

fn identify_unique_digits(
    patterns: &[String],
    pattern_map: &mut std::collections::HashMap<String, i32>,
    reverse_map: &mut std::collections::HashMap<i32, String>,
) {
    for pattern in patterns {
        let digit = match pattern.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        };

        if let Some(d) = digit {
            pattern_map.insert(pattern.clone(), d);
            reverse_map.insert(d, pattern.clone());
        }
    }
}

fn find_digit_zero(
    patterns: &[String],
    pattern_map: &mut std::collections::HashMap<String, i32>,
    reverse_map: &mut std::collections::HashMap<i32, String>,
) {
    let digit_4 = reverse_map.get(&4).cloned().unwrap();
    let digit_1 = reverse_map.get(&1).cloned().unwrap();
    let digit_8 = reverse_map.get(&8).cloned().unwrap();

    // Find middle segment candidates (segments in 4 but not in 1)
    let mid_candidates = diff(&digit_4, &digit_1);

    for c in mid_candidates.chars() {
        let maybe_zero = diff(&digit_8, &c.to_string());
        if patterns.contains(&maybe_zero) {
            pattern_map.insert(maybe_zero.clone(), 0);
            reverse_map.insert(0, maybe_zero);
            break;
        }
    }
}

fn decode_six_segment_digits(
    patterns: &[String],
    pattern_map: &mut std::collections::HashMap<String, i32>,
    reverse_map: &mut std::collections::HashMap<i32, String>,
) {
    let digit_1 = reverse_map.get(&1).cloned().unwrap();
    let digit_0 = reverse_map.get(&0).cloned().unwrap();

    let mut six_segment_patterns: Vec<String> = patterns
        .iter()
        .filter(|p| p.len() == 6 && **p != digit_0)
        .cloned()
        .collect();

    // Find 9: 6-segment pattern that fully contains 1
    if let Some(pos) = six_segment_patterns
        .iter()
        .position(|pattern| diff(pattern, &digit_1).len() == 4)
    {
        let digit_9 = six_segment_patterns.remove(pos);
        pattern_map.insert(digit_9.clone(), 9);
        reverse_map.insert(9, digit_9);
    }

    // Remaining 6-segment pattern is 6
    if let Some(digit_6) = six_segment_patterns.first() {
        pattern_map.insert(digit_6.clone(), 6);
        reverse_map.insert(6, digit_6.clone());
    }
}

fn decode_five_segment_digits(
    patterns: &[String],
    pattern_map: &mut std::collections::HashMap<String, i32>,
    reverse_map: &mut std::collections::HashMap<i32, String>,
) {
    let digit_1 = reverse_map.get(&1).cloned().unwrap();
    let digit_6 = reverse_map.get(&6).cloned().unwrap();

    let mut five_segment_patterns: Vec<String> =
        patterns.iter().filter(|p| p.len() == 5).cloned().collect();

    // Find 3: 5-segment pattern that fully contains 1
    if let Some(pos) = five_segment_patterns
        .iter()
        .position(|pattern| diff(pattern, &digit_1).len() == 3)
    {
        let digit_3 = five_segment_patterns.remove(pos);
        pattern_map.insert(digit_3.clone(), 3);
        reverse_map.insert(3, digit_3);
    }

    // Find 5: 5-segment pattern that is fully contained in 6
    if let Some(pos) = five_segment_patterns
        .iter()
        .position(|pattern| diff(&digit_6, pattern).len() == 1)
    {
        let digit_5 = five_segment_patterns.remove(pos);
        pattern_map.insert(digit_5.clone(), 5);
        reverse_map.insert(5, digit_5);
    }

    // Remaining 5-segment pattern is 2
    if let Some(digit_2) = five_segment_patterns.first() {
        pattern_map.insert(digit_2.clone(), 2);
        reverse_map.insert(2, digit_2.clone());
    }
}

fn solve_p1(input: &str) -> i32 {
    let entries = parse_input(input);
    let mut count = 0;
    for entry in entries {
        for digit in entry.output {
            let len = digit.len();
            // Digits 1, 7, 4, 8 have unique segment counts
            if len == 2 || len == 3 || len == 4 || len == 7 {
                count += 1;
            }
        }
    }
    count
}

fn solve_p2(input: &str) -> i32 {
    let entries = parse_input(input);
    let mut total = 0;
    for entry in entries {
        let pattern_map = decode(&entry.patterns);
        let mut value = 0;
        for digit in entry.output {
            let digit_value = match pattern_map.get(&digit) {
                Some(&decoded_digit) => decoded_digit,
                None => 0,
            };
            value = value * 10 + digit_value;
        }
        total += value;
    }
    total
}

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let input_path = std::path::Path::new(manifest_dir).join("input.txt");
    let input = std::fs::read_to_string(input_path).unwrap();

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
    fn test_decode() {
        let input = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab";
        let patterns: Vec<String> = input
            .split_whitespace()
            .map(|s| {
                let mut chars: Vec<char> = s.chars().collect();
                chars.sort();
                chars.into_iter().collect()
            })
            .collect();
        let pattern_map = decode(&patterns);

        // Test that the unique digit patterns are correctly identified
        assert_eq!(pattern_map.get(&"abcdefg".to_string()), Some(&8)); // 7 segments
        assert_eq!(pattern_map.get(&"abd".to_string()), Some(&7)); // 3 segments
        assert_eq!(pattern_map.get(&"abef".to_string()), Some(&4)); // 4 segments
        assert_eq!(pattern_map.get(&"ab".to_string()), Some(&1)); // 2 segments

        // Test that zero is identified (should be missing the middle segment)
        assert_eq!(pattern_map.get(&"abcdeg".to_string()), Some(&0));
        assert_eq!(pattern_map.get(&"abcdef".to_string()), Some(&9)); // 6 segments
        assert_eq!(pattern_map.get(&"bcdefg".to_string()), Some(&6)); // 6 segments
        assert_eq!(pattern_map.get(&"bcdef".to_string()), Some(&5)); // 5 segments
        assert_eq!(pattern_map.get(&"acdfg".to_string()), Some(&2)); // 5 segments
        assert_eq!(pattern_map.get(&"abcdf".to_string()), Some(&3)); // 5 segments
    }

    #[test]
    fn test_solve_with_test_input() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let test_input_path = std::path::Path::new(manifest_dir).join("test_input.txt");
        let input = std::fs::read_to_string(test_input_path).unwrap();

        let answer = solve_p1(&input);
        assert_eq!(answer, 26);

        let answer = solve_p2(&input);
        assert_eq!(answer, 61229);
    }
}
