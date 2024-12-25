fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let items = input.split("\n\n").collect::<Vec<_>>();
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for item in items {
        let first_line = item.lines().next().unwrap();
        if first_line.contains("#") {
            let mut lock = vec![0; first_line.len()];
            for line in item.lines().skip(1) {
                line.chars().enumerate().for_each(|(i, c)| {
                    if c == '#' {
                        lock[i] += 1
                    }
                });
            }
            locks.push(lock);
        } else {
            let mut key = vec![0; first_line.len()];
            for line in item.lines().take(6) {
                line.chars().enumerate().for_each(|(i, c)| {
                    if c == '#' {
                        key[i] += 1
                    }
                });
            }
            keys.push(key);
        }
    }
    (locks, keys)
}

fn solve_p1(input: &str) -> i32 {
    let (locks, keys) = parse_input(input);
    let mut fit_count = 0;
    for lock in locks {
        for key in &keys {
            let mut a_fit = true;
            for i in 0..lock.len() {
                if lock[i] + key[i] >= 6 {
                    a_fit = false;
                    break;
                }
            }
            if a_fit {
                fit_count += 1;
            }
        }
    }
    fit_count
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let start = std::time::Instant::now();
    let answer = solve_p1(&input);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:.1?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_with_test_input() {
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let answer = solve_p1(&input);
        assert_eq!(answer, 3);
    }
}
