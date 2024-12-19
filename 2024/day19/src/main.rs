use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let (towel_input, design_input) = input.split_once("\n\n").unwrap();
    let towels = towel_input.split(", ").map(|s| s.to_string()).collect();
    let designs = design_input.lines().map(|s| s.to_string()).collect();
    (towels, designs)
}

fn is_valid_design(towels: &[String], design: &str) -> bool {
    let mut stack = Vec::new();
    stack.push(design);

    while let Some(current_design) = stack.pop() {
        for towel in towels {
            if current_design.starts_with(towel) {
                let remaining_design = &current_design[towel.len()..];
                if remaining_design.is_empty() {
                    return true;
                }
                stack.push(remaining_design);
            }
        }
    }

    false
}

fn count_ways(towels: &[String], design: &str, cache: &mut HashMap<String, usize>) -> usize {
    if cache.contains_key(design) {
        return cache[design];
    }

    if design.is_empty() {
        return 1;
    }

    let ways = towels
        .iter()
        .filter(|&t| design.starts_with(t))
        .map(|t| count_ways(towels, &design[t.len()..], cache))
        .sum();
    cache.insert(design.to_string(), ways);
    ways
}

fn solve_p1(towels: &[String], designs: &[String]) -> usize {
    designs
        .iter()
        .filter(|d| is_valid_design(towels, d))
        .count()
}

fn solve_p2(towels: &[String], designs: &[String]) -> usize {
    let mut cache: HashMap<String, usize> = HashMap::new();
    designs
        .iter()
        .map(|d| count_ways(towels, d, &mut cache))
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (towels, designs) = parse_input(&input);

    let start = std::time::Instant::now();
    let answer = solve_p1(&towels, &designs);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:.1?}");

    let start = std::time::Instant::now();
    let answer = solve_p2(&towels, &designs);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, elapsed: {elapsed:.1?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_with_test_input() {
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let (towels, designs) = parse_input(&input);
        let answer = solve_p1(&towels, &designs);
        assert_eq!(answer, 6);
        let answer = solve_p2(&towels, &designs);
        assert_eq!(answer, 16);
    }
}
