use log::debug;
use regex::Regex;
use std::error::Error;
use std::fs;

fn calibration_sum(text: &str) -> u32 {
    text.lines().map(extract_calib_value).sum()
}

fn calibration_sum_digits(text: &str) -> u32 {
    text.lines().map(extract_calib_value_digits).sum()
}

fn extract_calib_value_digits(line: &str) -> u32 {
    let mut first_digit = None;
    let mut last_digit = None;
    for ch in line.chars() {
        if !ch.is_ascii_digit() {
            continue;
        }

        let digit: u32 = ch.to_digit(10).expect("Not a valid digit!");

        if first_digit.is_none() {
            first_digit = Some(digit);
        }
        last_digit = Some(digit)
    }
    (first_digit.unwrap_or(0) * 10) + last_digit.unwrap_or(0)
}

fn extract_calib_value(line: &str) -> u32 {
    // The Regex crated doesn't support overlapping matches, so finding the last digit in a string
    // can be problematic. For example, a string that ends with 'oneight' will return 1 rather than
    // 8 when using the Regex crate.
    let pattern = r"one|two|three|four|five|six|seven|eight|nine|\d";
    let re = Regex::new(pattern).unwrap();
    let first = str_to_digit(re.find_iter(line).map(|m| m.as_str()).next());

    // One approach to compensate for this is to reverse the string and the pattern. Not very
    // efficient but it does the job. A future rev is needed to improve this.
    let rev_pattern = r"eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d";
    let re = Regex::new(rev_pattern).unwrap();
    let rev_line: String = line.chars().rev().collect();
    let last = str_to_digit(re.find_iter(&rev_line).map(|m| m.as_str()).next());

    debug!("String: {line}, first: {:?}, last: {:?}", first, last,);
    (first * 10) + last
}

fn str_to_digit(digit: Option<&str>) -> u32 {
    match digit {
        Some("one") | Some("eno") | Some("1") => 1,
        Some("two") | Some("owt") | Some("2") => 2,
        Some("three") | Some("eerht") | Some("3") => 3,
        Some("four") | Some("ruof") | Some("4") => 4,
        Some("five") | Some("evif") | Some("5") => 5,
        Some("six") | Some("xis") | Some("6") => 6,
        Some("seven") | Some("neves") | Some("7") => 7,
        Some("eight") | Some("thgie") | Some("8") => 8,
        Some("nine") | Some("enin") | Some("9") => 9,
        _ => 0,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("input.txt")?;
    let sum = calibration_sum_digits(&text);
    println!("Part 1: Calibration sum using only digits: {sum}");
    let sum = calibration_sum(&text);
    println!("Part 2: Calibration sum: {sum}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_parse_digits_only() {
        assert_eq!(extract_calib_value_digits("1abc2"), 12);
        assert_eq!(extract_calib_value_digits("pqr3stu8vwx"), 38);
        assert_eq!(extract_calib_value_digits("a1b2c3d4e5f"), 15);
        assert_eq!(extract_calib_value_digits("treb7uchet"), 77);
    }

    #[test]
    fn test_line_parse() {
        assert_eq!(extract_calib_value("two1nine"), 29);
        assert_eq!(extract_calib_value("eightwothree"), 83);
        assert_eq!(extract_calib_value("abcone2threexyz"), 13);
        assert_eq!(extract_calib_value("xtwone3four"), 24);
        assert_eq!(extract_calib_value("4nineeightseven2"), 42);
        assert_eq!(extract_calib_value("zoneight234"), 14);
        assert_eq!(extract_calib_value("7pqrstsixteen"), 76);
        assert_eq!(
            extract_calib_value("ggdone3nbmsthreefourninefiveoneightpr"),
            18
        );
    }

    #[test]
    fn test_sample_digits_only() {
        let sample_text = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        assert_eq!(calibration_sum_digits(sample_text), 142);
    }

    #[test]
    fn test_sample() {
        let sample_text = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        assert_eq!(calibration_sum(sample_text), 281);
    }
}
