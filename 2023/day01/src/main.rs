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
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").unwrap();
    // let matches: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();
    let matches: Vec<_> = re.find_iter(line).collect();
    println!("{matches:?}");
    let first = matches
        .iter()
        .min_by(|x, y| x.start().cmp(&y.start()))
        .unwrap()
        .as_str();
    let last = matches
        .iter()
        .max_by(|x, y| x.end().cmp(&y.end()))
        .unwrap()
        .as_str();
    println!("String: {line}, first: {:?}, last: {:?}", first, last,);
    (str_to_digit(Some(first)) * 10) + str_to_digit(Some(last))
}

fn str_to_digit(digit: Option<&str>) -> u32 {
    match digit {
        Some("one") | Some("1") => 1,
        Some("two") | Some("2") => 2,
        Some("three") | Some("3") => 3,
        Some("four") | Some("4") => 4,
        Some("five") | Some("5") => 5,
        Some("six") | Some("6") => 6,
        Some("seven") | Some("7") => 7,
        Some("eight") | Some("8") => 8,
        Some("nine") | Some("9") => 9,
        _ => 0,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("input.txt")?;
    let sum = calibration_sum_digits(&text);
    println!("Calibration sum using only digits: {sum}");
    let sum = calibration_sum(&text);
    println!("Calibration sum: {sum}");
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
        // assert_eq!(extract_calib_value("two1nine"), 29);
        // assert_eq!(extract_calib_value("eightwothree"), 83);
        // assert_eq!(extract_calib_value("abcone2threexyz"), 13);
        // assert_eq!(extract_calib_value("xtwone3four"), 24);
        // assert_eq!(extract_calib_value("4nineeightseven2"), 42);
        // assert_eq!(extract_calib_value("zoneight234"), 14);
        // assert_eq!(extract_calib_value("7pqrstsixteen"), 76);
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
