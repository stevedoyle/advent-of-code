fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn find_median(crabs: &[i32]) -> i32 {
    let mut sorted = crabs.to_vec();
    sorted.sort_unstable();
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) / 2
    } else {
        sorted[mid]
    }
}

fn solve_p1(input: &str) -> i32 {
    let crabs = parse_input(input);
    let median = find_median(&crabs);
    crabs.iter().map(|&pos| (pos - median).abs()).sum()
}

fn solve_p2(input: &str) -> i32 {
    let crabs = parse_input(input);
    let mean = crabs.iter().copied().sum::<i32>() as f32 / crabs.len() as f32;
    println!("Mean position: {}", mean);
    let mean_floor = mean.floor() as i32;
    let mean_ceil = mean.ceil() as i32;
    let cost_floor: i32 = crabs
        .iter()
        .map(|&pos| {
            let n = (pos - mean_floor).abs();
            n * (n + 1) / 2
        })
        .sum();
    let cost_ceil = crabs
        .iter()
        .map(|&pos| {
            let n = (pos - mean_ceil).abs();
            n * (n + 1) / 2
        })
        .sum();
    cost_floor.min(cost_ceil)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

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

    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_solve_with_test_input() {
        let input = TEST_INPUT;
        let answer = solve_p1(&input);
        assert_eq!(answer, 37);
        let answer = solve_p2(&input);
        assert_eq!(answer, 168);
    }
}
