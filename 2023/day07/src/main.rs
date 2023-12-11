use std::{error::Error, fs, str::FromStr};

#[derive(Debug, PartialEq)]
pub struct ParseError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub enum HandKind {
    #[default]
    HighCard,
    OnePair,
    TwoPair,
    ThreeOAK,
    FullHouse,
    FourOAK,
    FiveOAK,
}

pub mod part1 {
    use std::{collections::HashMap, str::FromStr};

    use regex::Regex;

    use crate::HandKind;
    use crate::ParseError;

    #[derive(Debug, Default)]
    pub struct Game(Vec<Deal>);

    impl FromStr for Game {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let game = Game(
                s.lines()
                    .map(|line| Deal::from_str(line).unwrap())
                    .collect(),
            );
            Ok(game)
        }
    }

    impl std::ops::Deref for Game {
        type Target = Vec<Deal>;
        fn deref(&self) -> &Vec<Deal> {
            &self.0
        }
    }

    impl std::ops::DerefMut for Game {
        fn deref_mut(&mut self) -> &mut Vec<Deal> {
            &mut self.0
        }
    }

    impl Game {
        pub fn score(&mut self) {
            self.sort();
            self.iter_mut()
                .enumerate()
                .for_each(|(idx, deal)| deal.rank = (idx + 1) as u32);
        }

        pub fn winnings(&self) -> u32 {
            self.iter().map(|d| d.bid * d.rank).sum()
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Deal {
        pub hand: Hand,
        pub bid: u32,
        pub rank: u32,
    }

    impl FromStr for Deal {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let re = Regex::new(r"([\dAKQJT]+)\s+(\d+)").map_err(|_| ParseError)?;
            let captures = re.captures(s).ok_or(ParseError)?;
            let hand = Hand::from_str(captures.get(1).unwrap().as_str())?;
            let bid = captures.get(2).unwrap().as_str().parse().unwrap();
            let deal = Deal { hand, bid, rank: 1 };
            Ok(deal)
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Hand {
        pub kind: HandKind,
        pub cards: Vec<Card>,
    }

    impl FromStr for Hand {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let cards = s.chars().map(|c| Card::from_char(c).unwrap()).collect();
            let mut hand = Hand {
                cards,
                kind: Default::default(),
            };
            hand.classify();
            Ok(hand)
        }
    }

    impl Hand {
        pub fn classify(&mut self) {
            let counts: HashMap<Card, u8> = self.cards.iter().fold(HashMap::new(), |mut map, c| {
                *map.entry(c.clone()).or_insert(0) += 1;
                map
            });

            // Check for a full house (3 OAK + 2 OAK)
            if counts.values().all(|&v| v == 2 || v == 3) {
                self.kind = HandKind::FullHouse;
                return;
            }
            // Check for * of a kind
            match counts.values().max() {
                Some(5) => {
                    self.kind = HandKind::FiveOAK;
                    return;
                }
                Some(4) => {
                    self.kind = HandKind::FourOAK;
                    return;
                }
                Some(3) => {
                    self.kind = HandKind::ThreeOAK;
                    return;
                }
                _ => (),
            }
            // Check for Pair(s)
            match counts.values().filter(|&v| *v == 2).count() {
                2 => {
                    self.kind = HandKind::TwoPair;
                    return;
                }
                1 => {
                    self.kind = HandKind::OnePair;
                    return;
                }
                _ => (),
            }
            // We are left with a High Card
            self.kind = HandKind::HighCard;
        }
    }

    // impl PartialOrd for Hand {
    //     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    //         match self.kind.partial_cmp(&other.kind) {
    //             Some(core::cmp::Ordering::Equal) => {}
    //             ord => return ord,
    //         }
    //         self.cards.partial_cmp(&other.cards)
    //     }
    // }

    // impl PartialEq for Hand {
    //     fn eq(&self, other: &Self) -> bool {
    //         self.kind == other.kind && self.cards == other.cards
    //     }
    // }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
    pub enum Card {
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
        Ace,
    }

    impl Card {
        fn from_char(s: char) -> Result<Self, ParseError> {
            match s {
                '2' => Ok(Card::Two),
                '3' => Ok(Card::Three),
                '4' => Ok(Card::Four),
                '5' => Ok(Card::Five),
                '6' => Ok(Card::Six),
                '7' => Ok(Card::Seven),
                '8' => Ok(Card::Eight),
                '9' => Ok(Card::Nine),
                'T' => Ok(Card::Ten),
                'J' => Ok(Card::Jack),
                'Q' => Ok(Card::Queen),
                'K' => Ok(Card::King),
                'A' => Ok(Card::Ace),
                _ => Err(ParseError),
            }
        }
    }
}

pub mod part2 {
    use std::{collections::HashMap, str::FromStr};

    use regex::Regex;

    use crate::HandKind;
    use crate::ParseError;

    #[derive(Debug, Default)]
    pub struct Game(Vec<Deal>);

    impl FromStr for Game {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let game = Game(
                s.lines()
                    .map(|line| Deal::from_str(line).unwrap())
                    .collect(),
            );
            Ok(game)
        }
    }

    impl std::ops::Deref for Game {
        type Target = Vec<Deal>;
        fn deref(&self) -> &Vec<Deal> {
            &self.0
        }
    }

    impl std::ops::DerefMut for Game {
        fn deref_mut(&mut self) -> &mut Vec<Deal> {
            &mut self.0
        }
    }

    impl Game {
        pub fn score(&mut self) {
            self.sort();
            self.iter_mut()
                .enumerate()
                .for_each(|(idx, deal)| deal.rank = (idx + 1) as u32);
        }

        pub fn winnings(&self) -> u32 {
            self.iter().map(|d| d.bid * d.rank).sum()
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Deal {
        pub hand: Hand,
        pub bid: u32,
        pub rank: u32,
    }

    impl FromStr for Deal {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let re = Regex::new(r"([\dAKQJT]+)\s+(\d+)").map_err(|_| ParseError)?;
            let captures = re.captures(s).ok_or(ParseError)?;
            let hand = Hand::from_str(captures.get(1).unwrap().as_str())?;
            let bid = captures.get(2).unwrap().as_str().parse().unwrap();
            let deal = Deal { hand, bid, rank: 1 };
            Ok(deal)
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Hand {
        pub kind: HandKind,
        pub cards: Vec<Card>,
    }

    impl FromStr for Hand {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let cards = s.chars().map(|c| Card::from_char(c).unwrap()).collect();
            let mut hand = Hand {
                cards,
                kind: Default::default(),
            };
            hand.classify();
            Ok(hand)
        }
    }

    impl Hand {
        pub fn classify(&mut self) {
            let (jokers, without_jokers): (Vec<Card>, Vec<Card>) =
                self.cards.iter().partition(|&c| *c == Card::Joker);

            let jokers_count = jokers.len();
            let kind = self.classify_without_jokers(&without_jokers);

            // | Kind |   0  |   1  |   2  |   3  |   4  |
            // | ---- | ---- | ---- | ---- | ---- | ---- |
            // | 5OAK | 5OAK | -    | -    | -    | -    |
            // | 4OAK | 4OAK | 5OAK | -    | -    | -    |
            // | FH   | FH   | -    | -    | -    | -    |
            // | 3OAK | 3OAK | 4OAK | 5OAK | -    | -    |
            // | 2P   | 2P   | FH   | -    | -    | -    |
            // | 1P   | 1P   | 3OAK | 4OAK | 5OAK | -    |
            // | HC   | HC   | 1P   | 3OAK | 4OAK | 5OAK |
            self.kind = match jokers_count {
                0 => kind,
                1 => match kind {
                    HandKind::FourOAK => HandKind::FiveOAK,
                    HandKind::ThreeOAK => HandKind::FourOAK,
                    HandKind::TwoPair => HandKind::FullHouse,
                    HandKind::OnePair => HandKind::ThreeOAK,
                    HandKind::HighCard => HandKind::OnePair,
                    _ => Default::default(),
                },
                2 => match kind {
                    HandKind::ThreeOAK => HandKind::FiveOAK,
                    HandKind::OnePair => HandKind::FourOAK,
                    HandKind::HighCard => HandKind::ThreeOAK,
                    _ => Default::default(),
                },
                3 => match kind {
                    HandKind::OnePair => HandKind::FiveOAK,
                    HandKind::HighCard => HandKind::FourOAK,
                    _ => Default::default(),
                },
                _ => HandKind::FiveOAK,
            }
        }

        fn classify_without_jokers(&mut self, cards: &[Card]) -> HandKind {
            let counts: HashMap<Card, u8> = cards.iter().fold(HashMap::new(), |mut map, c| {
                *map.entry(c.clone()).or_insert(0) += 1;
                map
            });

            // Check for a full house (3 OAK + 2 OAK)
            if counts.values().all(|&v| v == 2 || v == 3) {
                return HandKind::FullHouse;
            }
            // Check for * of a kind
            match counts.values().max() {
                Some(5) => {
                    return HandKind::FiveOAK;
                }
                Some(4) => {
                    return HandKind::FourOAK;
                }
                Some(3) => {
                    return HandKind::ThreeOAK;
                }
                _ => (),
            }
            // Check for Pair(s)
            match counts.values().filter(|&v| *v == 2).count() {
                2 => {
                    return HandKind::TwoPair;
                }
                1 => {
                    return HandKind::OnePair;
                }
                _ => (),
            }
            // We are left with a High Card
            HandKind::HighCard
        }
    }

    // impl PartialOrd for Hand {
    //     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    //         match self.kind.partial_cmp(&other.kind) {
    //             Some(core::cmp::Ordering::Equal) => {}
    //             ord => return ord,
    //         }
    //         self.cards.partial_cmp(&other.cards)
    //     }
    // }

    // impl PartialEq for Hand {
    //     fn eq(&self, other: &Self) -> bool {
    //         self.kind == other.kind && self.cards == other.cards
    //     }
    // }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
    pub enum Card {
        Joker,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Queen,
        King,
        Ace,
    }

    impl Card {
        fn from_char(s: char) -> Result<Self, ParseError> {
            match s {
                'J' => Ok(Card::Joker),
                '2' => Ok(Card::Two),
                '3' => Ok(Card::Three),
                '4' => Ok(Card::Four),
                '5' => Ok(Card::Five),
                '6' => Ok(Card::Six),
                '7' => Ok(Card::Seven),
                '8' => Ok(Card::Eight),
                '9' => Ok(Card::Nine),
                'T' => Ok(Card::Ten),
                'Q' => Ok(Card::Queen),
                'K' => Ok(Card::King),
                'A' => Ok(Card::Ace),
                _ => Err(ParseError),
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let mut game = part1::Game::from_str(&input).unwrap();
    game.score();
    let winnings = game.winnings();
    println!("Part 1: Winnings = {winnings}");

    let mut game = part2::Game::from_str(&input).unwrap();
    game.score();
    let winnings = game.winnings();
    println!("Part 2: Winnings = {winnings}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483",
        )
    }

    #[test]
    fn test_parse_input() {
        let input = get_input();
        let game = part1::Game::from_str(&input).unwrap();
        assert_eq!(game.len(), 5);
        assert_eq!(game[0].bid, 765);
        assert_eq!(game.last().unwrap().bid, 483);
    }

    #[test]
    fn test_hand_classify() {
        assert_eq!(
            part1::Hand::from_str("32T3K").unwrap().kind,
            HandKind::OnePair
        );
        assert_eq!(
            part1::Hand::from_str("T55J5").unwrap().kind,
            HandKind::ThreeOAK
        );
        assert_eq!(
            part1::Hand::from_str("KK677").unwrap().kind,
            HandKind::TwoPair
        );
        assert_eq!(
            part1::Hand::from_str("KTJJT").unwrap().kind,
            HandKind::TwoPair
        );
        assert_eq!(
            part1::Hand::from_str("QQQJA").unwrap().kind,
            HandKind::ThreeOAK
        );
    }

    #[test]
    fn test_hand_compare() -> Result<(), ParseError> {
        assert!(part1::Hand::from_str("77777")? > part1::Hand::from_str("77778")?);
        assert!(part1::Hand::from_str("77888")? > part1::Hand::from_str("77788")?);
        assert!(part1::Hand::from_str("33332")? > part1::Hand::from_str("2AAAA")?);
        Ok(())
    }

    #[test]
    fn test_deal_scoring() -> Result<(), ParseError> {
        let mut game = part1::Game::from_str(&get_input()).unwrap();
        game.score();
        assert_eq!(game[0].rank, 1);
        assert_eq!(game[0].hand, part1::Hand::from_str("32T3K")?);
        assert_eq!(game[1].rank, 2);
        assert_eq!(game[1].hand, part1::Hand::from_str("KTJJT")?);
        assert_eq!(game[2].rank, 3);
        assert_eq!(game[2].hand, part1::Hand::from_str("KK677")?);
        assert_eq!(game[3].rank, 4);
        assert_eq!(game[3].hand, part1::Hand::from_str("T55J5")?);
        assert_eq!(game[4].rank, 5);
        assert_eq!(game[4].hand, part1::Hand::from_str("QQQJA")?);

        Ok(())
    }

    #[test]
    fn test_sample_p1() -> Result<(), ParseError> {
        let mut game = part1::Game::from_str(&get_input()).unwrap();
        game.score();
        let winnings = game.winnings();
        assert_eq!(winnings, 6440);

        Ok(())
    }

    #[test]
    fn test_hand_compare_with_jokers() -> Result<(), ParseError> {
        assert!(part2::Hand::from_str("77777")? > part2::Hand::from_str("7777J")?);
        assert!(part2::Hand::from_str("7778J")? > part2::Hand::from_str("7788J")?);
        assert!(part2::Hand::from_str("33333")? > part2::Hand::from_str("JAAAA")?);
        Ok(())
    }

    #[test]
    fn test_deal_scoring_p2() -> Result<(), ParseError> {
        let mut game = part2::Game::from_str(&get_input()).unwrap();
        game.score();
        assert_eq!(game[0].rank, 1);
        assert_eq!(game[0].hand, part2::Hand::from_str("32T3K")?);
        assert_eq!(game[0].hand.kind, HandKind::OnePair);

        assert_eq!(game[1].rank, 2);
        assert_eq!(game[1].hand, part2::Hand::from_str("KK677")?);
        assert_eq!(game[1].hand.kind, HandKind::TwoPair);

        assert_eq!(game[2].rank, 3);
        assert_eq!(game[2].hand, part2::Hand::from_str("T55J5")?);
        assert_eq!(game[2].hand.kind, HandKind::FourOAK);

        assert_eq!(game[3].rank, 4);
        assert_eq!(game[3].hand, part2::Hand::from_str("QQQJA")?);
        assert_eq!(game[3].hand.kind, HandKind::FourOAK);

        assert_eq!(game[4].rank, 5);
        assert_eq!(game[4].hand, part2::Hand::from_str("KTJJT")?);
        assert_eq!(game[4].hand.kind, HandKind::FourOAK);

        Ok(())
    }

    #[test]
    fn test_sample_p2() -> Result<(), ParseError> {
        let mut game = part2::Game::from_str(&get_input()).unwrap();
        game.score();
        let winnings = game.winnings();
        assert_eq!(winnings, 5905);

        Ok(())
    }
}
