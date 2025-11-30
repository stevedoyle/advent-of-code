use aoc2020::*;

fn parse_input(input: &str) -> Vec<usize> {
    parse_lines(input)
}

fn find_first_invalid_number(numbers: &[usize], window_size: usize) -> Option<usize> {
    for i in window_size..numbers.len() {
        let target = numbers[i];
        let mut found = false;
        'outer: for j in (i - window_size)..i {
            for k in (j + 1)..i {
                if numbers[j] + numbers[k] == target {
                    found = true;
                    break 'outer;
                }
            }
        }
        if !found {
            return Some(target);
        }
    }
    None
}

fn solve_p1(input: &str, window_size: usize) -> usize {
    let numbers = parse_input(input);
    find_first_invalid_number(&numbers, window_size).unwrap()
}

fn solve_p2(input: &str, window_size: usize) -> usize {
    let numbers = parse_input(input);
    find_first_invalid_number(&numbers, window_size)
        .and_then(|invalid_number| {
            for start in 0..numbers.len() {
                let mut sum = 0;
                for end in start..numbers.len() {
                    sum += numbers[end];
                    if sum == invalid_number {
                        let range = &numbers[start..=end];
                        let min = *range.iter().min().unwrap();
                        let max = *range.iter().max().unwrap();
                        return Some(min + max);
                    } else if sum > invalid_number {
                        break;
                    }
                }
            }
            None
        })
        .unwrap()
}

fn main() {
    let input = read_input(9);

    let start = std::time::Instant::now();
    let answer = solve_p1(&input, 25);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:.1?}");

    let start = std::time::Instant::now();
    let answer = solve_p2(&input, 25);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, elapsed: {elapsed:.1?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_with_test_input() {
        let input = read_test_input(9);
        let answer = solve_p1(&input, 5);
        assert_eq!(answer, 127);
        let answer = solve_p2(&input, 5);
        assert_eq!(answer, 62);
    }
}
