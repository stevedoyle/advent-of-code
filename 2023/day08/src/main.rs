use std::{collections::HashMap, error::Error, fs, str::FromStr};

use regex::Regex;

struct Map {
    directions: String,
    network: Network,
}

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line_iter = s.lines();
        let directions = line_iter.next().unwrap().trim().to_owned();

        let nodes = line_iter
            .filter(|line| !line.is_empty())
            .map(|line| Node::from_str(line).unwrap())
            .map(|n| (n.id.clone(), n))
            .collect();
        let map = Map {
            directions,
            network: Network(nodes),
        };
        Ok(map)
    }
}

impl Map {
    fn walk(&self) -> u32 {
        let mut next = "AAA";
        let mut dir_iter = self.directions.chars().cycle();
        let mut steps = 0;
        loop {
            if next == "ZZZ" {
                break;
            }
            next = self
                .network
                .get(next)
                .unwrap()
                .walk(dir_iter.next().unwrap());
            steps += 1;
        }
        steps
    }

    fn parallel_walk(&self) -> u32 {
        let mut next: Vec<&str> = self
            .network
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|k| k.as_str())
            .collect();
        let mut dir_iter = self.directions.chars().cycle();
        let mut steps = 0;
        loop {
            if next.iter().all(|&n| n.ends_with('Z')) {
                break;
            }
            let direction = dir_iter.next().unwrap();
            let tmp = next
                .iter()
                .map(|&id| self.network.get(id).unwrap().walk(direction))
                .collect();
            next = tmp;
            steps += 1;
        }
        steps
    }
}

struct Network(HashMap<String, Node>);

impl std::ops::Deref for Network {
    type Target = HashMap<String, Node>;
    fn deref(&self) -> &HashMap<String, Node> {
        &self.0
    }
}

impl std::ops::DerefMut for Network {
    fn deref_mut(&mut self) -> &mut HashMap<String, Node> {
        &mut self.0
    }
}

fn parse_input(input: &str) -> Map {
    Map::from_str(input).unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    id: String,
    left: String,
    right: String,
}

impl FromStr for Node {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").map_err(|_| ParseError)?;
        let captures = re.captures(s).ok_or(ParseError)?;
        let node = Node {
            id: captures.get(1).unwrap().as_str().to_owned(),
            left: captures.get(2).unwrap().as_str().to_owned(),
            right: captures.get(3).unwrap().as_str().to_owned(),
        };
        Ok(node)
    }
}

impl Node {
    fn walk(&self, direction: char) -> &str {
        match direction {
            'L' => &self.left,
            'R' => &self.right,
            _ => panic!("Invalid direction {direction}"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct ParseError;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let map = parse_input(&input);
    let steps = map.walk();
    println!("Part 1: Steps = {steps}");

    // let steps = map.parallel_walk();
    // println!("Part 2: Steps = {steps}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_parse() {
        let node = Node::from_str("AAA = (BBB, CCC)").unwrap();
        assert_eq!(node.id, "AAA");
        assert_eq!(node.left, "BBB");
        assert_eq!(node.right, "CCC");
    }

    fn get_input1() -> String {
        String::from(
            "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)",
        )
    }

    fn get_input2() -> String {
        String::from(
            "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)",
        )
    }

    #[test]
    fn test_parse_input() {
        let map = parse_input(&get_input1());
        assert_eq!(map.directions, "LLR");
        assert_eq!(map.network.len(), 3);
        assert!(map.network.contains_key("AAA"));
    }

    #[test]
    fn test_sample_map_walk() {
        let map = parse_input(&get_input1());
        let steps = map.walk();
        assert_eq!(steps, 6);
    }

    #[test]
    fn test_sample_map_walk_p2() {
        let map = parse_input(&get_input2());
        let steps = map.parallel_walk();
        assert_eq!(steps, 6);
    }
}
