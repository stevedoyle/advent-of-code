use std::{error::Error, fs, str::FromStr};

#[derive(Debug, Default)]
struct Game {
    id: u32,
    sets: Vec<CubeMix>,
    min_blue: u32,
    min_green: u32,
    min_red: u32,
    power: u32,
}

impl Game {
    fn new(id: u32) -> Self {
        Game {
            id,
            sets: Vec::with_capacity(10),
            min_blue: 0,
            min_green: 0,
            min_red: 0,
            power: 0,
        }
    }

    fn update_profile(&mut self) {
        self.min_blue = self.sets.iter().map(|x| x.blue).max().unwrap_or_default();
        self.min_green = self.sets.iter().map(|x| x.green).max().unwrap_or_default();
        self.min_red = self.sets.iter().map(|x| x.red).max().unwrap_or_default();
        self.power = self.min_blue * self.min_green * self.min_red;
    }
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = Game::new(0);

        let mut parts = s.trim().split(':');
        if let Some(head) = parts.next() {
            game.id = parse_game_id(head)?;
        }
        if let Some(game_data) = parts.next() {
            game.sets = parse_game_data(game_data)?;
        }
        game.update_profile();
        Ok(game)
    }
}

fn parse_game_id(s: &str) -> Result<u32, ParseGameError> {
    match s.split_once(' ') {
        Some((_, id)) => id.parse::<u32>().map_err(|_| ParseGameError),
        _ => Err(ParseGameError),
    }
}

fn parse_game_data(data: &str) -> Result<Vec<CubeMix>, ParseGameError> {
    let mix = data.split(';').map(parse_grab).collect();
    Ok(mix)
}

fn parse_grab(cube_str: &str) -> CubeMix {
    let mut cubemix = CubeMix::default();
    for cube in cube_str.split(',') {
        if let Some((count, color)) = cube.trim().split_once(' ') {
            let val: u32 = count.parse().unwrap_or_default();
            match color {
                "blue" => cubemix.blue = val,
                "green" => cubemix.green = val,
                "red" => cubemix.red = val,
                _ => (),
            }
        }
    }
    cubemix
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

#[derive(Debug, Default, PartialEq)]
struct CubeMix {
    blue: u32,
    green: u32,
    red: u32,
}

fn parse_games(data: &str) -> Vec<Game> {
    data.lines()
        .map(|line| Game::from_str(line).unwrap_or_default())
        .collect()
}

fn filter_games<'a>(games: &'a [Game], target: &CubeMix) -> Vec<&'a Game> {
    let filtered: Vec<&Game> = games
        .iter()
        .filter(|&g| {
            g.min_red <= target.red && g.min_blue <= target.blue && g.min_green <= target.green
        })
        .collect();
    filtered
}

fn id_sum(games: &[&Game]) -> u32 {
    games.iter().map(|&g| g.id).sum()
}

fn power_sum(games: &[Game]) -> u32 {
    games.iter().map(|g| g.power).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let target = CubeMix {
        blue: 14,
        green: 13,
        red: 12,
    };

    let text = fs::read_to_string("input.txt")?;
    let games = parse_games(&text);
    let filtered = filter_games(&games, &target);
    let sum1 = id_sum(&filtered);
    let sum2 = power_sum(&games);

    println!("Part 1: {sum1}");
    println!("Part 2: {sum2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_parse() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game::from_str(line).unwrap();
        assert_eq!(game.id, 1);
        assert_eq!(game.sets.len(), 3);
        assert_eq!(
            game.sets[0],
            CubeMix {
                blue: 3,
                green: 0,
                red: 4,
            }
        );
        assert_eq!(
            game.sets[1],
            CubeMix {
                blue: 6,
                green: 2,
                red: 1,
            }
        );
        assert_eq!(
            game.sets[2],
            CubeMix {
                blue: 0,
                green: 2,
                red: 0,
            }
        );
        assert_eq!(game.min_blue, 6);
        assert_eq!(game.min_green, 2);
        assert_eq!(game.min_red, 4);
        assert_eq!(game.power, 48);
    }

    #[test]
    fn test_id_sum() {
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";
        let target = CubeMix {
            blue: 14,
            green: 13,
            red: 8,
        };
        let games = parse_games(data);
        let filtered = filter_games(&games, &target);
        assert_eq!(id_sum(&filtered), 8);
        assert_eq!(power_sum(&games), 2286);
    }
}
