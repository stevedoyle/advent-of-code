use std::time::Instant;

use regex::Regex;

fn solve_p1(input: &str) -> isize {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    pattern
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| (a.parse::<isize>().unwrap(), b.parse::<isize>().unwrap()))
        .fold(0, |acc, (a, b)| acc + (a * b))
}

fn solve_p2(input: &str) -> isize {
    // First extract the input portions where mul instructions are enabled.
    // - Start until the first "don't()"
    // - "do()" until the next "don't()" or until the end of line
    let mut mul_enabled_regions = Vec::new();
    let parts = input.split("don't()").collect::<Vec<_>>();
    mul_enabled_regions.push(parts[0]);
    for part in parts.iter().skip(1) {
        let end = part.find("do()").unwrap_or(part.len());
        mul_enabled_regions.push(&part[end..]);
    }
    let filtered_input = mul_enabled_regions.join("");

    solve_p1(&filtered_input)
}

fn main() {
    let input = include_str!("../input.txt");

    let start = Instant::now();
    let answer = solve_p1(input);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, {elapsed:.2?}");

    let start = Instant::now();
    let answer = solve_p2(input);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, {elapsed:.2?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_with_test_input() {
        let p1_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let answer = solve_p1(&p1_input);
        assert_eq!(answer, 161);
        let p2_input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let answer = solve_p2(&p2_input);
        assert_eq!(answer, 48);
    }
}
