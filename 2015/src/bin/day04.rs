use aoc2015::*;

const DAY: u8 = 4;

fn parse_input(input: &str) -> &str {
    input.trim()
}

fn find(key: &str, prefix: &str) -> usize {
    let mut i = 0;
    loop {
        let input = format!("{}{}", key, i);
        let digest = format!("{:x}", md5::compute(input));
        if digest.starts_with(prefix) {
            return i;
        }
        i += 1;
    }
}

fn solve_p1(input: &str) -> usize {
    let key = parse_input(input);
    find(key, "00000")
}

fn solve_p2(input: &str) -> usize {
    let key = parse_input(input);
    find(key, "000000")
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
