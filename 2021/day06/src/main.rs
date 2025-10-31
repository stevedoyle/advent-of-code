fn parse_input(input: &str) -> Vec<usize> {
    input.split(',').map(|num| num.parse().unwrap()).collect()
}

fn fish_count(days: usize, initial_fish: &[usize]) -> usize {
    let mut fish_counts = [0usize; 9];
    for &f in initial_fish {
        fish_counts[f] += 1;
    }
    for _ in 0..days {
        fish_counts.rotate_left(1);
        fish_counts[6] += fish_counts[8];
    }
    fish_counts.iter().sum()
}

fn solve_p1(input: &str) -> usize {
    let fish = parse_input(input);
    fish_count(80, &fish)
}
fn solve_p2(input: &str) -> usize {
    let fish = parse_input(input);
    fish_count(256, &fish)
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

    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_solve_with_test_input() {
        let input = TEST_INPUT;
        let answer = solve_p1(&input);
        assert_eq!(answer, 5934);
        let answer = solve_p2(&input);
        assert_eq!(answer, 26984457539);
    }
}
