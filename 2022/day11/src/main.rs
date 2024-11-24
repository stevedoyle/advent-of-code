use regex::Regex;
use std::{collections::VecDeque, fmt};

struct Monkey {
    id: i32,
    starting_items: VecDeque<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    divisible: usize,
    true_monkey: i32,
    false_monkey: i32,
    inspected_count: usize,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            id: 0,
            starting_items: VecDeque::new(),
            operation: Box::new(|x| x),
            divisible: 0,
            true_monkey: 0,
            false_monkey: 0,
            inspected_count: 0,
        }
    }
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Monkey {{ id: {}, starting_items: {:?}, test: {}, true_monkey: {}, false_monkey: {} , inspected_count: {} }}",
            self.id, self.starting_items, self.divisible, self.true_monkey, self.false_monkey, self.inspected_count
        )
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let monkey_texts: Vec<&str> = input.split("\n\n").collect();
    monkey_texts.iter().map(|x| parse_monkey(x)).collect()
}

fn parse_monkey(text: &str) -> Monkey {
    let mut monkey = Monkey::new();

    let mut line_iter = text.lines();
    let mut line = line_iter.next().unwrap();
    while line.is_empty() {
        line = line_iter.next().unwrap();
    }

    monkey.id = Regex::new(r"Monkey (\d+):")
        .unwrap()
        .captures(line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    line = line_iter.next().unwrap();
    monkey.starting_items = line
        .split(": ")
        .nth(1)
        .unwrap()
        .split(", ")
        .map(|x| x.parse().unwrap())
        .collect();

    line = line_iter.next().unwrap();
    monkey.operation = parse_operation(line.split(": ").nth(1).unwrap());

    line = line_iter.next().unwrap();
    monkey.divisible = line
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .nth(2)
        .unwrap()
        .parse()
        .unwrap();

    line = line_iter.next().unwrap();
    let mut split = line.split(": ");
    monkey.true_monkey = split
        .nth(1)
        .unwrap()
        .split(" ")
        .nth(3)
        .unwrap()
        .parse()
        .unwrap();

    line = line_iter.next().unwrap();
    let mut split = line.split(": ");
    monkey.false_monkey = split
        .nth(1)
        .unwrap()
        .split(" ")
        .nth(3)
        .unwrap()
        .parse()
        .unwrap();
    monkey
}

fn parse_operation(operation: &str) -> Box<dyn Fn(usize) -> usize> {
    let re1 = Regex::new(r"new = old (.) (\d+)").unwrap();
    let re2 = Regex::new(r"new = old (.) old").unwrap();

    if re1.is_match(operation) {
        let cap = re1.captures(operation).unwrap();
        let op = cap.get(1).unwrap().as_str();
        let x: usize = cap.get(2).unwrap().as_str().parse().unwrap();
        match op {
            "*" => return Box::new(move |y| y * x),
            "+" => return Box::new(move |y| y + x),
            _ => return Box::new(move |y| y),
        }
    } else if re2.is_match(operation) {
        let cap = re2.captures(operation).unwrap();
        let op = cap.get(1).unwrap().as_str();
        match op {
            "*" => return Box::new(move |y| y * y),
            "+" => return Box::new(move |y| y + y),
            _ => return Box::new(move |y| y),
        }
    }
    Box::new(move |y| y)
}

fn solve_p1(input: &str) -> usize {
    let mut monkeys = parse_input(input);
    (0..20).for_each(|_| {
        (0..monkeys.len()).for_each(|i| {
            while !monkeys[i].starting_items.is_empty() {
                let monkey = &mut monkeys[i];
                monkey.inspected_count += 1;
                let mut item = monkey.starting_items.pop_front().unwrap();
                item = (monkey.operation)(item);
                item /= 3;
                let next_monkey_id = match item % monkey.divisible {
                    0 => monkey.true_monkey,
                    _ => monkey.false_monkey,
                };
                monkeys[next_monkey_id as usize]
                    .starting_items
                    .push_back(item);
            }
        });
    });

    monkeys.sort_by(|a, b| a.inspected_count.cmp(&b.inspected_count));
    let len = monkeys.len();
    monkeys[len - 1].inspected_count * monkeys[len - 2].inspected_count
}

fn solve_p2(input: &str) -> usize {
    let mut monkeys = parse_input(input);
    let divisor_product = monkeys.iter().fold(1, |acc, x| acc * x.divisible);
    (0..10_000).for_each(|_| {
        (0..monkeys.len()).for_each(|i| {
            while !monkeys[i].starting_items.is_empty() {
                let monkey = &mut monkeys[i];
                monkey.inspected_count += 1;
                let mut item = monkey.starting_items.pop_front().unwrap();
                item = (monkey.operation)(item) % divisor_product;
                let next_monkey_id = match item % monkey.divisible {
                    0 => monkey.true_monkey,
                    _ => monkey.false_monkey,
                };
                monkeys[next_monkey_id as usize]
                    .starting_items
                    .push_back(item);
            }
        });
    });

    monkeys.sort_by(|a, b| a.inspected_count.cmp(&b.inspected_count));
    let len = monkeys.len();
    monkeys[len - 1].inspected_count * monkeys[len - 2].inspected_count
}

fn main() {
    let input = include_str!("../input.txt");
    let answer = solve_p1(input);
    println!("Part 1: {answer}");
    let answer = solve_p2(input);
    println!("Part 2: {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 10605);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 2713310158);
    }
}
