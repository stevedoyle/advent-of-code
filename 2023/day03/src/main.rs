use regex::Regex;
use std::{error::Error, fs};

#[derive(Debug, Clone)]
struct PartNumber {
    value: u32,
    border: Vec<Coordinate>,
}

impl PartNumber {
    fn from_match(line_idx: i32, x_start: i32, x_end: i32, s: &str) -> Self {
        let mut pn = PartNumber {
            value: s.parse::<u32>().unwrap(),
            border: Vec::with_capacity(20),
        };
        for x in (x_start - 1)..(x_end + 1) {
            pn.border.push(Coordinate::new(x, line_idx - 1));
        }
        for x in (x_start - 1)..(x_end + 1) {
            pn.border.push(Coordinate::new(x, line_idx));
        }
        for x in (x_start - 1)..(x_end + 1) {
            pn.border.push(Coordinate::new(x, line_idx + 1));
        }
        pn
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Coordinate { x, y }
    }
}

fn get_parts(engine: &str) -> Vec<PartNumber> {
    let re_number = Regex::new(r"(\d+)").unwrap();
    let re_symbol = Regex::new(r"([^\.\d]+)").unwrap();

    let mut numbers: Vec<PartNumber> = Vec::with_capacity(1000);
    let mut symbols: Vec<Coordinate> = Vec::with_capacity(1000);

    for (idx, line) in engine.lines().enumerate() {
        let mut number: Vec<PartNumber> = re_number
            .find_iter(line)
            .map(|m| {
                PartNumber::from_match(idx as i32, m.start() as i32, m.end() as i32, m.as_str())
            })
            .collect();
        numbers.append(&mut number);

        let mut symbol: Vec<Coordinate> = re_symbol
            .find_iter(line)
            .map(|m| Coordinate::new(m.start() as i32, idx as i32))
            .collect();

        symbols.append(&mut symbol);
    }

    let mut parts: Vec<PartNumber> = Vec::with_capacity(1000);
    for s in symbols {
        let mut tmp: Vec<PartNumber> = numbers
            .clone()
            .into_iter()
            .filter(|n| n.border.contains(&s))
            .collect();
        parts.append(&mut tmp);
    }

    parts
}

fn get_part_numbers(parts: &[PartNumber]) -> Vec<u32> {
    parts.iter().map(|n| n.value).collect()
}

fn get_gears(engine: &str) -> Vec<u32> {
    let re_number = Regex::new(r"(\d+)").unwrap();
    let re_gear = Regex::new(r"(\*)").unwrap();

    let mut numbers: Vec<PartNumber> = Vec::with_capacity(1000);
    let mut gear_symbols: Vec<Coordinate> = Vec::with_capacity(1000);

    for (idx, line) in engine.lines().enumerate() {
        let mut number: Vec<PartNumber> = re_number
            .find_iter(line)
            .map(|m| {
                PartNumber::from_match(idx as i32, m.start() as i32, m.end() as i32, m.as_str())
            })
            .collect();
        numbers.append(&mut number);

        let mut symbol: Vec<Coordinate> = re_gear
            .find_iter(line)
            .map(|m| Coordinate::new(m.start() as i32, idx as i32))
            .collect();

        gear_symbols.append(&mut symbol);
    }

    let mut gears: Vec<u32> = Vec::with_capacity(1000);
    for s in gear_symbols {
        let parts: Vec<PartNumber> = numbers
            .clone()
            .into_iter()
            .filter(|n| n.border.contains(&s))
            .collect();
        if parts.len() == 2 {
            gears.push(parts.iter().map(|pn| pn.value).product())
        }
    }

    gears
}

fn main() -> Result<(), Box<dyn Error>> {
    let engine = fs::read_to_string("input.txt")?;
    let part_numbers = get_part_numbers(&get_parts(&engine));
    let sum: u32 = part_numbers.iter().sum();
    println!("Part 1 sum: {sum}");

    let gear_ratios = get_gears(&engine);
    let sum: u32 = gear_ratios.iter().sum();
    println!("Part 1 sum: {sum}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_part_numbers() {
        let engine = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

        let parts = get_parts(&engine);
        let part_numbers = get_part_numbers(&parts);
        assert_eq!(part_numbers.iter().sum::<u32>(), 4361);
    }

    #[test]
    fn test_get_gears() {
        let engine = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

        let gears = get_gears(&engine);
        assert_eq!(gears.iter().sum::<u32>(), 467835);
    }
}
