use aoc2015::*;

const DAY: u8 = 1;

fn solve_p1(input: &str) -> i32 {
    let mut floor = 0;
    for c in input.chars() {
        if c == '(' {
            floor += 1;
        }
        if c == ')' {
            floor -= 1;
        }
    }
    floor
}

fn solve_p2(input: &str) -> i32 {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            floor += 1;
        }
        if c == ')' {
            floor -= 1;
        }
        if floor == -1 {
            return (i + 1) as i32;
        }
    }
    0
}

fn main() {
    run(DAY, solve_p1, solve_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve_p1("(())"), 0);
        assert_eq!(solve_p1("()()"), 0);
        assert_eq!(solve_p1("((("), 3);
        assert_eq!(solve_p1("(()(()("), 3);
        assert_eq!(solve_p1("))((((("), 3);
        assert_eq!(solve_p1("())"), -1);
        assert_eq!(solve_p1("))("), -1);
        assert_eq!(solve_p1(")())())"), -3);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_p2(")"), 1);
        assert_eq!(solve_p2("()())"), 5);
    }
}
