use log::debug;
use regex::Regex;
use std::{collections::HashMap, error::Error, fs, str::FromStr};

#[derive(Debug, Default)]
struct Seeds(Vec<usize>);

impl std::ops::Deref for Seeds {
    type Target = Vec<usize>;
    fn deref(&self) -> &Vec<usize> {
        &self.0
    }
}

impl std::ops::DerefMut for Seeds {
    fn deref_mut(&mut self) -> &mut Vec<usize> {
        &mut self.0
    }
}

impl FromStr for Seeds {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"seeds: ((?:\d+\s*)+)").unwrap();
        if let Some(captures) = re.captures(s) {
            let seeds = Seeds {
                0: captures
                    .get(1)
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect(),
            };
            return Ok(seeds);
        }
        Err(ParseError)
    }
}

struct Map {
    from: String,
    to: String,
    entries: Vec<Mapping>,
}

impl Map {
    fn convert(&self, value: usize) -> usize {
        match self.entries.iter().find(|&x| x.in_range(value)) {
            Some(mapping) => mapping.convert(value).expect("Mapping Conversion Error"),
            _ => value,
        }
    }
}

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\s*(\w+)-to-(\w+) map:").unwrap();
        if let Some(captures) = re.captures(s) {
            let map = Map {
                from: captures.get(1).unwrap().as_str().to_owned(),
                to: captures.get(2).unwrap().as_str().to_owned(),
                entries: Vec::with_capacity(100),
            };
            return Ok(map);
        }
        Err(ParseError)
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
struct Mapping {
    source: usize,
    dest: usize,
    length: usize,
}

impl Mapping {
    fn new(source: usize, dest: usize, length: usize) -> Self {
        Mapping {
            source,
            dest,
            length,
        }
    }

    fn in_range(&self, value: usize) -> bool {
        self.source <= value && value <= self.source + self.length
    }

    fn convert(&self, value: usize) -> Result<usize, ConversionError> {
        match self.in_range(value) {
            true => Ok(value - self.source + self.dest),
            false => Err(ConversionError),
        }
    }
}

impl FromStr for Mapping {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<usize> = s
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let mapping = Mapping {
            source: values[1],
            dest: values[0],
            length: values[2],
        };
        return Ok(mapping);
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

#[derive(Debug, PartialEq, Eq)]
struct ConversionError;

fn parse_input(input: &str) -> (Seeds, Vec<Map>) {
    let mut maps: Vec<Map> = Vec::with_capacity(100);
    let mut line_iter = input.lines();

    let seeds = Seeds::from_str(line_iter.next().unwrap()).expect("Error parsing seeds!");

    let mut in_map = false;
    for line in line_iter {
        if line.is_empty() {
            in_map = false;
            continue;
        }

        if !in_map {
            let map = Map::from_str(line).expect("Error parsing map!");
            maps.push(map);
            in_map = true;
        } else {
            let mapping = Mapping::from_str(line).expect("Error parsing mapping entry!");
            maps.last_mut().unwrap().entries.push(mapping);
        }
    }
    (seeds, maps)
}

fn get_seed_locations(seeds: &Seeds, maps: &Vec<Map>) -> HashMap<usize, usize> {
    let mut seed_loc_map = HashMap::with_capacity(100);

    for seed in &seeds.0 {
        let mut map = maps.iter().find(|&map| map.from == "seed").unwrap();
        let mut value = map.convert(*seed);
        debug!("{} ({}) -> {} ({})", map.from, *seed, map.to, value);
        loop {
            if map.to == "location" {
                break;
            }
            map = maps.iter().find(|&m| m.from == map.to).unwrap();
            value = map.convert(value);
            debug!(" -> {} ({})", map.to, value);
        }
        seed_loc_map.insert(*seed, value);
    }

    seed_loc_map
}

fn get_lowest_location(map: &HashMap<usize, usize>) -> usize {
    *map.iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|(_k, v)| v)
        .unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let (seeds, maps) = parse_input(&input);
    let seed_to_loc = get_seed_locations(&seeds, &maps);
    let lowest = get_lowest_location(&seed_to_loc);
    println!("Part 1: {lowest}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4",
        )
    }

    #[test]
    fn test_seeds_parse() {
        let input = "seeds: 79 14 55 13";
        let seeds = Seeds::from_str(&input).unwrap();
        assert_eq!(seeds.len(), 4);
        assert_eq!(seeds.0, vec![79, 14, 55, 13]);
    }

    #[test]
    fn test_map_parse() {
        let input = "seed-to-soil map:
            50 98 2
            52 50 48";

        let map = Map::from_str(&input).unwrap();
        assert_eq!(map.from, "seed");
        assert_eq!(map.to, "soil");
    }

    #[test]
    fn test_parse_input() {
        let input = get_input();
        let (seeds, maps) = parse_input(&input);

        assert_eq!(seeds.len(), 4);
        assert_eq!(seeds.0, vec![79, 14, 55, 13]);

        assert_eq!(maps.len(), 7);
        let map = &maps[0];
        assert_eq!(map.from, "seed");
        assert_eq!(map.to, "soil");
        assert_eq!(map.entries.len(), 2);
        assert_eq!(map.entries[0], Mapping::new(98, 50, 2));
        assert_eq!(map.entries[1], Mapping::new(50, 52, 48));

        let map = maps.last().unwrap();
        assert_eq!(map.from, "humidity");
        assert_eq!(map.to, "location");
        assert_eq!(map.entries.len(), 2);
        assert_eq!(map.entries[0], Mapping::new(56, 60, 37));
        assert_eq!(map.entries[1], Mapping::new(93, 56, 4));
    }

    #[test]
    fn test_get_nearest_location() {
        let input = get_input();
        let (seeds, maps) = parse_input(&input);
        let seed_to_loc = get_seed_locations(&seeds, &maps);
        dbg!(&seed_to_loc);
        let nearest = get_lowest_location(&seed_to_loc);
        assert_eq!(nearest, 35);
    }
}
