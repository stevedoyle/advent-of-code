use std::{fs, iter::zip};

use regex::Regex;

struct Race {
    time: usize,
    distance: usize,
}

fn ways_to_win(race: &Race) -> usize {
    (0..race.time)
        .filter(|x| (race.time - x) * x > race.distance)
        .count()
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = parse_times(lines.next().unwrap());
    let distances = parse_distances(lines.next().unwrap());
    zip(times, distances)
        .map(|(t, d)| Race {
            time: t,
            distance: d,
        })
        .collect()
}

fn parse_times(input: &str) -> Vec<usize> {
    let re_time = Regex::new(r"Time:\s+((?:\d+\s*)+)").unwrap();
    re_time
        .captures(input)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn parse_distances(input: &str) -> Vec<usize> {
    let re_distance = Regex::new(r"Distance:\s+((?:\d+\s*)+)").unwrap();
    re_distance
        .captures(input)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn parse_input2(input: &str) -> Race {
    let mut lines = input.lines();
    Race {
        time: parse_time(lines.next().unwrap()),
        distance: parse_distance(lines.next().unwrap()),
    }
}

fn parse_time(input: &str) -> usize {
    let re_time = Regex::new(r"Time:\s+((?:\d+\s*)+)").unwrap();
    re_time
        .captures(input)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .split_whitespace()
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn parse_distance(input: &str) -> usize {
    let re_distance = Regex::new(r"Distance:\s+((?:\d+\s*)+)").unwrap();
    re_distance
        .captures(input)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .split_whitespace()
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let races = parse_input(&input);
    let product: usize = races.iter().map(ways_to_win).product();
    println!("Part 1: {product}");

    let race = parse_input2(&input);
    let ways = ways_to_win(&race);
    println!("Part 2: {ways}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "Time:      7  15   30
        Distance:  9  40  200",
        )
    }

    #[test]
    fn test_parse() {
        let input = get_input();
        let races = parse_input(&input);
        assert_eq!(races.len(), 3);
        assert_eq!(races[0].time, 7);
        assert_eq!(races[0].distance, 9);
        assert_eq!(races[1].time, 15);
        assert_eq!(races[1].distance, 40);
        assert_eq!(races[2].time, 30);
        assert_eq!(races[2].distance, 200);
    }

    #[test]
    fn test_parse2() {
        let input = get_input();
        let race = parse_input2(&input);
        assert_eq!(race.time, 71530);
        assert_eq!(race.distance, 940200);
    }

    #[test]
    fn test_ways_to_win() {
        let race = Race {
            time: 7,
            distance: 9,
        };

        let count = ways_to_win(&race);
        assert_eq!(count, 4);
    }

    #[test]
    fn test_sample() {
        let input = get_input();
        let races = parse_input(&input);
        let product: usize = races.iter().map(|r| ways_to_win(r)).product();
        assert_eq!(product, 288);
    }

    #[test]
    fn test_sample_p2() {
        let input = get_input();
        let race = parse_input2(&input);
        let ways = ways_to_win(&race);
        assert_eq!(ways, 71503);
    }
}
