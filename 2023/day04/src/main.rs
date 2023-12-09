use regex::Regex;
use std::{collections::HashSet, error::Error, fs, str::FromStr};

#[derive(Debug)]
struct Card {
    winners: HashSet<u8>,
    mine: HashSet<u8>,
    count: u32,
}

impl Card {
    fn winnings(&self) -> u32 {
        let count = self.matched_numbers_count();
        match count {
            0 => 0,
            _ => 2_u32.pow(count - 1),
        }
    }

    fn matched_numbers_count(&self) -> u32 {
        self.winners.intersection(&self.mine).count() as u32
    }
}

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Card\s+(\d+):\s+((?:\d+\s+)+)\|\s+((?:\d+\s*)+)").unwrap();
        if let Some(captures) = re.captures(s) {
            let card = Card {
                count: 1,
                winners: captures
                    .get(2)
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect(),
                mine: captures
                    .get(3)
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect(),
            };
            return Ok(card);
        }
        Err(ParseCardError)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCardError;

fn calculate_winnings(cards: &[Card]) -> u32 {
    cards.iter().map(|card| card.winnings()).sum()
}

fn get_cards(text: &str) -> Vec<Card> {
    text.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| Card::from_str(line).unwrap())
        .collect()
}

fn process_scratchcards(cards: &mut Vec<Card>) {
    let mut counts = vec![1u32; cards.len()];

    for (idx, card) in cards.iter_mut().enumerate() {
        let count = card.matched_numbers_count();
        for x in 0..count {
            counts[idx + 1 + x as usize] += counts[idx];
        }
        card.count = counts[idx];
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let mut cards = get_cards(&input);
    let winnings = calculate_winnings(&cards);
    println!("Part 1 Winnings: {winnings}");
    process_scratchcards(&mut cards);
    let count: u32 = cards.iter().map(|c| c.count).sum();
    println!("Part 2 Total Card Count: {count}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_from_str() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::from_str(input).unwrap();
        assert_eq!(card.winners, HashSet::from([41, 48, 83, 86, 17]));
        assert_eq!(card.mine, HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]));
        assert_eq!(card.winnings(), 8);
    }

    #[test]
    fn test_scratchcard_winnings() {
        let input = "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let cards = get_cards(&input);
        let winnings = calculate_winnings(&cards);
        assert_eq!(winnings, 13);
    }

    #[test]
    fn test_scratchcard_count() {
        let input = "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let mut cards = get_cards(&input);
        process_scratchcards(&mut cards);
        let count: u32 = cards.iter().map(|c| c.count).sum();
        assert_eq!(count, 30);
    }
}
