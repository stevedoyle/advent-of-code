use std::collections::HashMap;

use aoc2020::*;

const DAY: u8 = 4;

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn parse_input(input: &str) -> Vec<HashMap<String, String>> {
    let passports: Vec<&str> = input.split("\n\n").collect();
    passports
        .into_iter()
        .map(|passport| {
            passport
                .split_whitespace()
                .filter_map(|field| {
                    let (key, value) = field.split_once(':')?;
                    Some((key.to_string(), value.to_string()))
                })
                .collect()
        })
        .collect()
}

fn solve_p1(input: &str) -> usize {
    let passports = parse_input(input);
    passports
        .into_iter()
        .filter(|passport| {
            REQUIRED_FIELDS
                .iter()
                .all(|&field| passport.contains_key(field))
        })
        .count()
}

fn validate_field(key: &str, value: &str) -> bool {
    match key {
        "byr" => {
            // four digits; at least 1920 and at most 2002
            value.len() == 4
                && value
                    .parse::<u32>()
                    .is_ok_and(|y| (1920..=2002).contains(&y))
        }
        "iyr" => {
            // four digits; at least 2010 and at most 2020
            value.len() == 4
                && value
                    .parse::<u32>()
                    .is_ok_and(|y| (2010..=2020).contains(&y))
        }
        "eyr" => {
            // four digits; at least 2020 and at most 2030
            value.len() == 4
                && value
                    .parse::<u32>()
                    .is_ok_and(|y| (2020..=2030).contains(&y))
        }
        "hgt" => {
            // a number followed by either cm or in
            if let Some(cm) = value.strip_suffix("cm") {
                cm.parse::<u32>().is_ok_and(|h| (150..=193).contains(&h))
            } else if let Some(inch) = value.strip_suffix("in") {
                inch.parse::<u32>().is_ok_and(|h| (59..=76).contains(&h))
            } else {
                false
            }
        }
        "hcl" => {
            // a # followed by exactly six characters 0-9 or a-f
            value.len() == 7
                && value.starts_with('#')
                && value[1..].chars().all(|c| c.is_ascii_hexdigit())
        }
        "ecl" => {
            // exactly one of: amb blu brn gry grn hzl oth
            matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
        }
        "pid" => {
            // a nine-digit number, including leading zeroes
            value.len() == 9 && value.chars().all(|c| c.is_ascii_digit())
        }
        "cid" => true, // ignored
        _ => false,
    }
}

fn solve_p2(input: &str) -> usize {
    let passports = parse_input(input);
    passports
        .into_iter()
        .filter(|passport| {
            REQUIRED_FIELDS
                .iter()
                .all(|&field| passport.contains_key(field))
                && passport
                    .iter()
                    .all(|(key, value)| validate_field(key, value))
        })
        .count()
}

fn main() {
    run(DAY, solve_p1, solve_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve_p1(&read_test_input(DAY, 1)), 2);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_p2(&read_test_input(DAY, 1)), 2);
    }
}
