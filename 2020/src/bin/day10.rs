use aoc2020::*;

fn solve_p1(input: &str) -> usize {
    let mut adapters: Vec<usize> = parse_lines(input);
    adapters.push(0); // Add charging outlet
    adapters.push(adapters.iter().max().unwrap() + 3); // Add device's built-in adapter
    adapters.sort_unstable();
    let mut diff1 = 0;
    let mut diff3 = 0;
    for pair in adapters.windows(2) {
        let diff = pair[1] - pair[0];
        if diff == 1 {
            diff1 += 1;
        } else if diff == 3 {
            diff3 += 1;
        }
    }
    diff1 * diff3
}

fn solve_p2(input: &str) -> usize {
    let mut adapters: Vec<usize> = parse_lines(input);
    adapters.push(0); // Add charging outlet
    adapters.push(adapters.iter().max().unwrap() + 3); // Add device's built-in adapter
    adapters.sort_unstable();
    let mut ways: Vec<usize> = vec![0; adapters.len()];
    ways[0] = 1; // One way to reach the charging outlet
    for i in 1..adapters.len() {
        for j in (0..i).rev() {
            if adapters[i] - adapters[j] <= 3 {
                ways[i] += ways[j];
            } else {
                break;
            }
        }
    }
    *ways.last().unwrap_or(&0) as usize // Total ways to reach the device's built-in adapter
}

fn main() {
    let input = read_input(10);

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
        let input = read_test_input(10);
        let answer = solve_p1(&input);
        assert_eq!(answer, 220);
        let answer = solve_p2(&input);
        assert_eq!(answer, 19208);
    }
}
