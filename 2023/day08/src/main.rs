use std::{error::Error, fs};

use fnv::FnvHashMap;
use num::Integer;
use regex::Regex;

struct Map<'a> {
    directions: Vec<char>,
    network: Network<'a>,
}

impl<'a> From<&'a str> for Map<'a> {
    fn from(s: &'a str) -> Self {
        let mut line_iter = s.lines();
        let directions = line_iter.next().unwrap().trim().chars().collect();

        let nodes = line_iter
            .filter(|line| !line.is_empty())
            .map(Node::from)
            .map(|node| (node.id.to_owned(), node))
            .collect();
        let map = Map {
            directions,
            network: Network(nodes),
        };
        map
    }
}

impl Map<'_> {
    fn walk(&self, start: &str, end: &str) -> usize {
        let mut next = start;
        let mut dir_iter = self.directions.iter().cycle();
        let mut steps = 0;
        loop {
            if next.ends_with(end) {
                break;
            }
            next = self
                .network
                .get(next)
                .unwrap()
                .walk(*dir_iter.next().unwrap());
            steps += 1;
        }
        steps
    }

    fn multi_walk(&self) -> usize {
        let start: Vec<_> = self.network.keys().filter(|k| k.ends_with('A')).collect();
        let distances: Vec<usize> = start.iter().map(|&s| self.walk(s, "Z")).collect();
        distances.into_iter().reduce(|acc, d| acc.lcm(&d)).unwrap()
    }
}

struct Network<'a>(FnvHashMap<String, Node<'a>>);

impl<'a> std::ops::Deref for Network<'a> {
    type Target = FnvHashMap<String, Node<'a>>;
    fn deref(&self) -> &FnvHashMap<String, Node<'a>> {
        &self.0
    }
}

impl<'a> std::ops::DerefMut for Network<'a> {
    fn deref_mut(&mut self) -> &mut FnvHashMap<String, Node<'a>> {
        &mut self.0
    }
}

fn parse_input(input: &str) -> Map {
    Map::from(input)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node<'a> {
    id: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> From<&'a str> for Node<'a> {
    fn from(s: &'a str) -> Self {
        let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
        let captures = re.captures(s).unwrap();
        Node {
            id: captures.get(1).unwrap().as_str(),
            left: captures.get(2).unwrap().as_str(),
            right: captures.get(3).unwrap().as_str(),
        }
    }
}

impl Node<'_> {
    fn walk(&self, direction: char) -> &str {
        match direction {
            'L' => self.left,
            'R' => self.right,
            _ => panic!("Invalid direction {direction}"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct ParseError;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let map = parse_input(&input);
    let steps = map.walk("AAA", "ZZZ");
    println!("Part 1: Steps = {steps}");

    let steps = map.multi_walk();
    println!("Part 2: Steps = {steps}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_parse() {
        let node = Node::from("AAA = (BBB, CCC)");
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
        let input = get_input1();
        let map = parse_input(&input);
        assert_eq!(map.directions, vec!['L', 'L', 'R']);
        assert_eq!(map.network.len(), 3);
        assert!(map.network.contains_key("AAA"));
    }

    #[test]
    fn test_sample_map_walk() {
        let input = get_input1();
        let map = parse_input(&input);
        let steps = map.walk("AAA", "ZZZ");
        assert_eq!(steps, 6);
    }

    #[test]
    fn test_sample_map_walk_p2() {
        let input = get_input2();
        let map = parse_input(&input);
        let steps = map.multi_walk();
        assert_eq!(steps, 6);
    }
}
