//! Shared helpers for Advent of Code 2026.
//!
//! Coordinate convention: everything uses [`Point`] `{ x, y }` with `x` = column,
//! `y` = row, and `y` increasing downwards (so `Point::UP` is `(0, -1)`).

use std::fmt::{Debug, Display};
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;

pub mod grid;
pub mod point;

pub use grid::Grid;
pub use point::{DIRS4, DIRS8, Point};

// ---------------------------------------------------------------------------
// Input
// ---------------------------------------------------------------------------

/// Read `inputs/dayNN.txt`.
pub fn read_input(day: u8) -> String {
    read_file(&format!("inputs/day{day:02}.txt"))
}

/// Read a test input: `n = 1` → `inputs/dayNN_test.txt`, `n >= 2` → `inputs/dayNN_testN.txt`.
pub fn read_test_input(day: u8, n: u8) -> String {
    if n <= 1 {
        read_file(&format!("inputs/day{day:02}_test.txt"))
    } else {
        read_file(&format!("inputs/day{day:02}_test{n}.txt"))
    }
}

/// Paths are resolved relative to the crate root, so binaries and tests work from any directory.
fn read_file(rel: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()))
}

// ---------------------------------------------------------------------------
// Running
// ---------------------------------------------------------------------------

/// Load the day's input and run both parts, printing each answer with its timing.
pub fn run<A: Display, B: Display>(day: u8, p1: impl Fn(&str) -> A, p2: impl Fn(&str) -> B) {
    let input = read_input(day);
    timed("Part 1", || p1(&input));
    timed("Part 2", || p2(&input));
}

fn timed<T: Display>(label: &str, f: impl FnOnce() -> T) {
    let start = Instant::now();
    let answer = f();
    println!("{label}: {answer} ({:.1?})", start.elapsed());
}

// ---------------------------------------------------------------------------
// Parsing
// ---------------------------------------------------------------------------

/// Parse each line into `T`.
pub fn parse_lines<T>(input: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    input
        .lines()
        .map(|l| l.parse().expect("failed to parse line"))
        .collect()
}

/// Extract every integer from a string, ignoring all other characters.
///
/// A `-` counts as a minus sign only when it is not preceded by a digit,
/// so ranges like `"3-7"` give `[3, 7]` while `"x=-4"` gives `[-4]`.
/// Panics if a negative number is parsed into an unsigned type.
pub fn nums<T>(s: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let b = s.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        let is_minus = b[i] == b'-'
            && i + 1 < b.len()
            && b[i + 1].is_ascii_digit()
            && (i == 0 || !b[i - 1].is_ascii_digit());
        if b[i].is_ascii_digit() || is_minus {
            let start = i;
            i += 1;
            while i < b.len() && b[i].is_ascii_digit() {
                i += 1;
            }
            out.push(s[start..i].parse().expect("failed to parse number"));
        } else {
            i += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nums_extracts_signed_integers() {
        assert_eq!(nums::<i64>("p=3,-4 v=-1,12"), vec![3, -4, -1, 12]);
        assert_eq!(nums::<i64>("3-7"), vec![3, 7]);
        assert_eq!(nums::<u32>("Game 12: 5 red"), vec![12, 5]);
        assert!(nums::<i32>("no numbers").is_empty());
    }

    #[test]
    fn parse_lines_parses_each_line() {
        assert_eq!(parse_lines::<i32>("1\n-2\n3"), vec![1, -2, 3]);
    }
}
