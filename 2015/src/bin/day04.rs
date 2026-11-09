use aoc2015::*;

const DAY: u8 = 4;

fn parse_input(input: &str) -> &str {
    input.trim()
}

/// True if the first `zeros` hex digits of the digest are all zero.
fn has_leading_zeros(digest: &[u8; 16], zeros: usize) -> bool {
    let full_bytes = zeros / 2;
    digest[..full_bytes].iter().all(|&b| b == 0)
        && (zeros % 2 == 0 || digest[full_bytes] >> 4 == 0)
}

fn find(key: &str, zeros: usize) -> usize {
    let mut i = 0;
    loop {
        let digest = md5::compute(format!("{}{}", key, i));
        if has_leading_zeros(&digest.0, zeros) {
            return i;
        }
        i += 1;
    }
}

fn solve_p1(input: &str) -> usize {
    let key = parse_input(input);
    find(key, 5)
}

fn solve_p2(input: &str) -> usize {
    let key = parse_input(input);
    find(key, 6)
}

fn main() {
    run(DAY, solve_p1, solve_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve_p1("abcdef"), 609043);
        assert_eq!(solve_p1("pqrstuv"), 1048970);
    }

    #[test]
    fn part2() {}
}
