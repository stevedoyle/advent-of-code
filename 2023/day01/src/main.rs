use std::error::Error;
use std::fs;

fn calibration_sum(text: &str) -> u32 {
    let values: Vec<u32> = text.lines().map(|line| extract_calib_value(line)).collect();
    values.iter().sum()
}

fn extract_calib_value(line: &str) -> u32 {
    let mut first_digit = None;
    let mut last_digit = None;
    for ch in line.chars() {
        if !ch.is_digit(10) {
            continue;
        }

        let digit: u32 = ch.to_digit(10).expect("Not a valid digit!");

        if first_digit.is_none() {
            first_digit = Some(digit);
        }
        last_digit = Some(digit)
    }
    (first_digit.unwrap() * 10) + last_digit.unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("input.txt")?;
    let sum = calibration_sum(&text);
    println!("Calibration sum: {sum}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_parse() {
        assert_eq!(extract_calib_value("1abc2"), 12);
        assert_eq!(extract_calib_value("pqr3stu8vwx"), 38);
        assert_eq!(extract_calib_value("a1b2c3d4e5f"), 15);
        assert_eq!(extract_calib_value("treb7uchet"), 77);
    }

    #[test]
    fn test_sample() {
        let sample_text = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        assert_eq!(calibration_sum(sample_text), 142);
    }
}
