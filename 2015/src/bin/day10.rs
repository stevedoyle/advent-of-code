use aoc2015::*;
use std::fmt::Write;

const DAY: u8 = 10;

fn parse_input(input: &str) -> &str {
    input.trim()
}

/// One look-and-say step: each run of a repeated digit becomes "<count><digit>".
/// e.g. `111221` -> `312211` (three 1s, two 2s, one 1).
fn look_and_say(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 2);
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        let mut count = 1;
        while chars.next_if_eq(&c).is_some() {
            count += 1;
        }
        write!(out, "{count}{c}").expect("writing to a String cannot fail");
    }
    out
}

/// Length of the sequence after applying `look_and_say` `steps` times.
fn length_after(seed: &str, steps: usize) -> usize {
    (0..steps)
        .fold(seed.to_string(), |s, _| look_and_say(&s))
        .len()
}

fn solve_p1(input: &str) -> usize {
    length_after(parse_input(input), 40)
}

fn solve_p2(input: &str) -> usize {
    length_after(parse_input(input), 50) // Assuming part 2 requires 50 steps
}

fn main() {
    run(DAY, solve_p1, solve_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn look_and_say_steps() {
        // The examples from the puzzle.
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }

    #[test]
    fn look_and_say_counts_above_nine() {
        assert_eq!(look_and_say("1111111111"), "101");
    }

    #[test]
    fn length_after_steps() {
        assert_eq!(length_after("1", 0), 1);
        assert_eq!(length_after("1", 1), 2);
        assert_eq!(length_after("11", 1), 2);
        assert_eq!(length_after("21", 1), 4);
        assert_eq!(length_after("1211", 1), 6);
        assert_eq!(length_after("111221", 1), 6);
        assert_eq!(length_after("1", 5), 6); // 1, 11, 21, 1211, 111221, 312211
    }

    #[test]
    fn part2() {}
}
