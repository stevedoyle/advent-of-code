fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn solve_p1(input: &str) -> i32 {
    let data = parse_input(input);
    let mut count = 0;
    for window in data.windows(2) {
        if window[1] > window[0] {
            count += 1;
        }
    }
    count
}

fn solve_p2(input: &str) -> i32 {
    let data = parse_input(input);
    let mut count = 0;
    for window in data.windows(4) {
        let sum1: i32 = window[0..3].iter().sum();
        let sum2: i32 = window[1..4].iter().sum();
        if sum2 > sum1 {
            count += 1;
        }
    }
    count
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
        assert_eq!(answer, 7);
        let answer = solve_p2(&input);
        assert_eq!(answer, 5);
    }
}
