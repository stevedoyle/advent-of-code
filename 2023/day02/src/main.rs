use std::{collections::HashMap, str::FromStr};

struct Game {
    id: u32,
    sets: Vec<CubeMix>,
}

impl Game {
    fn new(id: u32) -> Self {
        Game {
            id,
            sets: Vec::with_capacity(10),
        }
    }
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(pat)
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

#[derive(Debug)]
struct CubeMix {
    cubes: HashMap<Color, u32>,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Color {
    Blue,
    Green,
    Red,
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_parse() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    }
}
