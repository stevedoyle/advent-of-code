use aoc2015::*;
use serde_json::Value;

const DAY: u8 = 12;

/// Join all lines into one string, trimming whitespace around each line.
///
/// Whitespace between JSON tokens is insignificant, and a newline can't occur
/// inside a JSON string or number, so dropping the line breaks is lossless.
fn parse_input(input: &str) -> String {
    input.lines().map(str::trim).collect()
}

fn solve_p1(input: &str) -> isize {
    let data = parse_input(input);
    nums::<isize>(&data).iter().sum()
}

/// Sum every number in the document, skipping any object that has a property
/// whose *value* is the string `"red"` (the whole object, including everything
/// nested inside it). Arrays are never skipped, and neither are objects whose
/// *key* is `"red"`.
fn sum_ignoring_red(value: &Value) -> isize {
    match value {
        Value::Number(n) => n.as_i64().expect("integer") as isize,
        Value::Array(items) => items.iter().map(sum_ignoring_red).sum(),
        Value::Object(props) => {
            if props.values().any(|v| v.as_str() == Some("red")) {
                0
            } else {
                props.values().map(sum_ignoring_red).sum()
            }
        }
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
    }
}

fn solve_p2(input: &str) -> isize {
    let json: Value = serde_json::from_str(&parse_input(input)).expect("valid JSON");
    sum_ignoring_red(&json)
}

fn main() {
    run(DAY, solve_p1, solve_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_joins_lines() {
        assert_eq!(parse_input("[1,2,3]"), "[1,2,3]");
        assert_eq!(parse_input("[1,\n  2,\r\n  3]\n"), "[1,2,3]");
        assert_eq!(parse_input(""), "");
    }

    #[test]
    fn part1() {
        assert_eq!(solve_p1("[1,2,3]"), 6);
        assert_eq!(solve_p1("{\"a\":2,\"b\":4}"), 6);
        assert_eq!(solve_p1("[[[3]]]"), 3);
        assert_eq!(solve_p1("{\"a\":{\"b\":4},\"c\":-1}"), 3);
        assert_eq!(solve_p1("{\"a\":[-1,1]}"), 0);
        assert_eq!(solve_p1("[-1, {\"a\":1}]"), 0);
        assert_eq!(solve_p1("[]"), 0);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_p2("[1,2,3]"), 6);
        assert_eq!(solve_p2("[1,{\"c\":\"red\",\"b\":2},3]"), 4);
        assert_eq!(solve_p2("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"), 0);
        assert_eq!(solve_p2("[1,\"red\",5]"), 6);
    }

    #[test]
    fn part2_edge_cases() {
        // A key named "red" is fine; only a property *value* of "red" counts.
        assert_eq!(solve_p2("{\"red\":1,\"a\":2}"), 3);
        // The skipped object takes everything nested inside it with it.
        assert_eq!(solve_p2("[1,{\"a\":{\"b\":10},\"c\":\"red\"},2]"), 3);
        // A red object nested in a non-red object only removes itself.
        assert_eq!(solve_p2("{\"a\":1,\"b\":{\"c\":\"red\",\"d\":100}}"), 1);
        // Arrays are never skipped, even if they contain "red".
        assert_eq!(solve_p2("[\"red\",{\"a\":4}]"), 4);
        // Multi-line input is joined before parsing.
        assert_eq!(solve_p2("[1,\n{\"c\":\"red\",\n\"b\":2},\n3]"), 4);
    }
}
