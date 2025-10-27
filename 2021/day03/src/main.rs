fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '1').collect())
        .collect()
}

fn solve_p1(input: &str) -> usize {
    let data = parse_input(input);
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..data[0].len() {
        let mask = 1 << (data[0].len() - 1 - i);
        let mut count_ones = 0;
        for num in &data {
            if num[i] {
                count_ones += 1;
            }
        }
        if count_ones * 2 >= data.len() {
            gamma |= mask;
        } else {
            epsilon |= mask;
        }
    }
    gamma * epsilon
}

fn solve_p2(input: &str) -> i32 {
    let data = parse_input(input);
    let mut oxygen_candidates = data.clone();
    let mut co2_candidates = data.clone();
    let bit_length = data[0].len();
    for i in 0..bit_length {
        if oxygen_candidates.len() > 1 {
            let mut count_ones = 0;
            for num in &oxygen_candidates {
                if num[i] {
                    count_ones += 1;
                }
            }
            let keep_one = count_ones * 2 >= oxygen_candidates.len();
            oxygen_candidates.retain(|num| num[i] == keep_one);
        }
        if co2_candidates.len() > 1 {
            let mut count_ones = 0;
            for num in &co2_candidates {
                if num[i] {
                    count_ones += 1;
                }
            }
            let keep_one = count_ones * 2 < co2_candidates.len();
            co2_candidates.retain(|num| num[i] == keep_one);
        }
    }
    let oxygen = oxygen_candidates[0]
        .iter()
        .fold(0, |acc, &b| (acc << 1) | (b as i32));
    let co2 = co2_candidates[0]
        .iter()
        .fold(0, |acc, &b| (acc << 1) | (b as i32));
    oxygen * co2
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

    const TEST_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_solve_with_test_input() {
        let input = TEST_INPUT;
        let answer = solve_p1(&input);
        assert_eq!(answer, 198);
        let answer = solve_p2(&input);
        assert_eq!(answer, 230);
    }
}
