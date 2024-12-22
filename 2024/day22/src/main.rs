use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn generate_one_secret(initial_secret: usize) -> usize {
    const MODULUS: usize = 16777216;
    let mut secret = initial_secret;
    // Step 1
    secret = ((secret * 64) ^ secret) % MODULUS;
    // Step 2
    secret = ((secret / 32) ^ secret) % MODULUS;
    // Step 3
    secret = ((secret * 2048) ^ secret) % MODULUS;
    secret
}

fn generate_secret(initial_secret: usize, iterations: usize) -> usize {
    let mut secret = initial_secret;
    (0..iterations).for_each(|_| secret = generate_one_secret(secret));
    secret
}

fn ones_digit(value: usize) -> usize {
    value % 10
}

fn generate_sequences(
    banana_count: &mut HashMap<(isize, isize, isize, isize), usize>,
    initial_secret: usize,
    iterations: usize,
) -> &mut HashMap<(isize, isize, isize, isize), usize> {
    let mut secret = initial_secret;
    let mut ones_digits = Vec::with_capacity(iterations);

    (0..iterations).for_each(|_| {
        secret = generate_one_secret(secret);
        ones_digits.push(ones_digit(secret));
    });

    let mut local_cache = HashMap::new();

    for window in ones_digits.windows(5) {
        let key = (
            window[1] as isize - window[0] as isize,
            window[2] as isize - window[1] as isize,
            window[3] as isize - window[2] as isize,
            window[4] as isize - window[3] as isize,
        );
        let value = window[4];

        local_cache.entry(key).or_insert_with(|| {
            *banana_count.entry(key).or_insert(0) += value;
            value
        });
    }
    banana_count
}

fn solve_p1(input: &str) -> usize {
    let iterations = 2000;
    let secrets = parse_input(input);
    secrets
        .iter()
        .map(|s| generate_secret(*s, iterations))
        .sum()
}

fn solve_p2(input: &str) -> usize {
    let iterations = 2000;
    let secrets = parse_input(input);

    let mut banana_count = HashMap::<(isize, isize, isize, isize), usize>::new();

    *secrets
        .iter()
        .fold(&mut banana_count, |current_banana_count, &secret| {
            generate_sequences(current_banana_count, secret, iterations)
        })
        .iter()
        .max_by_key(|&(_, &count)| count)
        .unwrap()
        .1
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

    #[test]
    fn test_solve_with_test_input() {
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let answer = solve_p1(&input);
        assert_eq!(answer, 37327623);
        let input = std::fs::read_to_string("test_input2.txt").unwrap();
        let answer = solve_p2(&input);
        assert_eq!(answer, 23);
    }

    #[test]
    fn test_generate_secret() {
        assert_eq!(generate_secret(123, 1), 15887950);
        assert_eq!(generate_secret(123, 2), 16495136);
        assert_eq!(generate_secret(123, 3), 527345);
        assert_eq!(generate_secret(123, 10), 5908254);
    }

    #[test]
    fn test_evolution() {
        let mut banana_count = HashMap::<(isize, isize, isize, isize), usize>::new();
        generate_sequences(&mut banana_count, 123, 10);
        assert_eq!(banana_count.get(&(-1, -1, 0, 2)), Some(&6));
    }
}
