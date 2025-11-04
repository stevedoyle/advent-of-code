use std::collections::HashMap;

type RuleMap = HashMap<String, char>;
type PairCounts = HashMap<String, i64>;
type CharCounts = HashMap<char, i64>;

fn parse_input(input: &str) -> (String, RuleMap) {
    let mut lines = input.lines();
    let template = lines.next().expect("Missing template").to_string();
    lines.next(); // skip empty line

    let rules = lines
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(" -> ").collect();
            if parts.len() == 2 {
                Some((parts[0].to_string(), parts[1].chars().next()?))
            } else {
                None
            }
        })
        .collect();

    (template, rules)
}

fn count_chars(s: &str) -> CharCounts {
    let mut counts = HashMap::new();
    for ch in s.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }
    counts
}

fn count_pairs(s: &str) -> PairCounts {
    let mut counts = HashMap::new();
    let chars: Vec<char> = s.chars().collect();
    for window in chars.windows(2) {
        let pair = format!("{}{}", window[0], window[1]);
        *counts.entry(pair).or_insert(0) += 1;
    }
    counts
}

fn simulate_polymer_growth(template: &str, rules: &RuleMap, steps: usize) -> i64 {
    let mut pair_counts = count_pairs(template);
    let mut char_counts = count_chars(template);

    for _ in 0..steps {
        let mut new_pair_counts = HashMap::new();

        for (pair, &count) in &pair_counts {
            if let Some(&insert_char) = rules.get(pair) {
                let chars: Vec<char> = pair.chars().collect();
                let left_pair = format!("{}{}", chars[0], insert_char);
                let right_pair = format!("{}{}", insert_char, chars[1]);

                *new_pair_counts.entry(left_pair).or_insert(0) += count;
                *new_pair_counts.entry(right_pair).or_insert(0) += count;
                *char_counts.entry(insert_char).or_insert(0) += count;
            } else {
                *new_pair_counts.entry(pair.clone()).or_insert(0) += count;
            }
        }

        pair_counts = new_pair_counts;
    }

    let max_count = *char_counts.values().max().unwrap();
    let min_count = *char_counts.values().min().unwrap();
    max_count - min_count
}

fn solve_p1(input: &str) -> i64 {
    let (template, rules) = parse_input(input);
    simulate_polymer_growth(&template, &rules, 10)
}

fn solve_p2(input: &str) -> i64 {
    let (template, rules) = parse_input(input);
    simulate_polymer_growth(&template, &rules, 40)
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
    fn test_solve_with_test_input() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let test_input_path = std::path::Path::new(manifest_dir).join("test_input.txt");
        let input = std::fs::read_to_string(test_input_path).unwrap();
        let answer = solve_p1(&input);
        assert_eq!(answer, 1588);
        let answer = solve_p2(&input);
        assert_eq!(answer, 2188189693529);
    }
}
