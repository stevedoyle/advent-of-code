use aoc2015::*;
use std::collections::HashSet;

const DAY: u8 = 11;

fn parse_input(input: &str) -> &str {
    input.trim()
}

/// A password is valid if it:
/// 1. has an increasing straight of at least three letters (`abc`, `bcd`, ... `xyz`; no wrapping),
/// 2. does not contain `i`, `o` or `l`, and
/// 3. has at least two different, non-overlapping pairs of letters (`aa`, `bb`, ...).
fn is_valid_password(pw: &str) -> bool {
    let b = pw.as_bytes();

    let has_straight = b
        .windows(3)
        .any(|w| w[1] == w[0].wrapping_add(1) && w[2] == w[1].wrapping_add(1));

    let has_confusing = b.iter().any(|c| matches!(c, b'i' | b'o' | b'l'));

    // Pairs of different letters can never overlap, so it is enough to count
    // how many distinct letters appear doubled. `aaaa` is one pair twice, not two pairs.
    let pair_letters: HashSet<u8> = b
        .windows(2)
        .filter(|w| w[0] == w[1])
        .map(|w| w[0])
        .collect();

    has_straight && !has_confusing && pair_letters.len() >= 2
}

/// Add one to the password, treating it as a base-26 number (`xx` -> `xy`, `xz` -> `ya`).
/// An all-`z` password wraps around to all `a`.
fn increment(pw: &mut [u8]) {
    for c in pw.iter_mut().rev() {
        if *c == b'z' {
            *c = b'a';
        } else {
            *c += 1;
            return;
        }
    }
}

/// If the password contains `i`, `o` or `l`, jump to the first password after
/// all those that share the same prefix up to that letter, e.g. `ghijklmn` -> `ghjaaaaa`.
/// Every password skipped over contains the bad letter, so none can be valid.
fn skip_confusing(pw: &mut [u8]) {
    if let Some(pos) = pw.iter().position(|c| matches!(c, b'i' | b'o' | b'l')) {
        pw[pos] += 1; // i -> j, o -> p, l -> m
        pw[pos + 1..].fill(b'a');
    }
}

/// The next valid password strictly after `pw`.
fn next_password(pw: &str) -> String {
    let mut candidate = pw.as_bytes().to_vec();
    loop {
        increment(&mut candidate);
        skip_confusing(&mut candidate);
        let s = std::str::from_utf8(&candidate).expect("passwords are ASCII");
        if is_valid_password(s) {
            return s.to_string();
        }
    }
}

fn solve_p1(input: &str) -> String {
    let seed = parse_input(input);
    next_password(seed)
}

/// The password expires again, so find the one after part 1's answer.
fn solve_p2(input: &str) -> String {
    let seed = parse_input(input);
    next_password(&next_password(seed))
}

fn main() {
    run(DAY, solve_p1, solve_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_passwords() {
        assert!(is_valid_password("abcdffaa"));
        assert!(is_valid_password("ghjaabcc"));
    }

    #[test]
    fn invalid_passwords() {
        // Has a straight (hij, ijk, ...) but contains i and l.
        assert!(!is_valid_password("hijklmmn"));
        // Has two pairs but no straight.
        assert!(!is_valid_password("abbceffg"));
        // Has a straight but only one pair.
        assert!(!is_valid_password("abbcegjk"));
    }

    #[test]
    fn password_rule_edge_cases() {
        // `aaaa` is the same pair twice, not two different pairs.
        assert!(!is_valid_password("abcaaaaz"));
        // The straight may not wrap around the alphabet.
        assert!(!is_valid_password("yzaaabbq"));
        // Each of i, o, l is rejected even when the other rules pass.
        assert!(!is_valid_password("abcdffoo"));
        assert!(!is_valid_password("abcdffil"));
    }

    fn inc(s: &str) -> String {
        let mut b = s.as_bytes().to_vec();
        increment(&mut b);
        String::from_utf8(b).unwrap()
    }

    fn skip(s: &str) -> String {
        let mut b = s.as_bytes().to_vec();
        skip_confusing(&mut b);
        String::from_utf8(b).unwrap()
    }

    #[test]
    fn increment_carries() {
        assert_eq!(inc("xx"), "xy");
        assert_eq!(inc("xz"), "ya");
        assert_eq!(inc("azz"), "baa");
        assert_eq!(inc("zz"), "aa");
    }

    #[test]
    fn skip_confusing_jumps_past_bad_letters() {
        assert_eq!(skip("ghijklmn"), "ghjaaaaa");
        assert_eq!(skip("abcdefgo"), "abcdefgp");
        assert_eq!(skip("lbcdefgh"), "maaaaaaa");
        assert_eq!(skip("abcdefgh"), "abcdefgh");
    }

    #[test]
    fn part1() {
        assert_eq!(solve_p1("abcdefgh"), "abcdffaa");
        assert_eq!(solve_p1("ghijklmn"), "ghjaabcc");
    }

    #[test]
    fn part2() {
        // abcdefgh -> abcdffaa -> next valid password after that
        assert_eq!(solve_p2("abcdefgh"), "abcdffbb");
    }
}
