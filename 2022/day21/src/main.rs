use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    fn get_value(&self, v1: i64, v2: i64) -> i64 {
        use Operator::*;
        match self {
            Add => v1 + v2,
            Sub => v1 - v2,
            Mul => v1 * v2,
            Div => v1 / v2,
        }
    }

    fn get_opposite(&self) -> Operator {
        use Operator::*;
        match self {
            Add => Sub,
            Sub => Add,
            Mul => Div,
            Div => Mul,
        }
    }
}

enum Monkey {
    Const(i64),
    Expression(usize, usize, Operator),
}

impl Monkey {
    fn get_value(&self, monkeys: &[Monkey]) -> i64 {
        match self {
            Monkey::Const(c) => *c,
            Monkey::Expression(i1, i2, op) => op.get_value(
                monkeys[*i1].get_value(monkeys),
                monkeys[*i2].get_value(monkeys),
            ),
        }
    }

    fn get_expected1(&self, expected_result: i64, monkeys: &[Monkey]) -> i64 {
        match self {
            Monkey::Const(_) => panic!(),
            Monkey::Expression(_, i2, op) => op
                .get_opposite()
                .get_value(expected_result, monkeys[*i2].get_value(monkeys)),
        }
    }

    fn get_expected2(&self, expected_result: i64, monkeys: &[Monkey]) -> i64 {
        match self {
            Monkey::Const(_) => panic!(),
            Monkey::Expression(i1, _, op) => {
                let v1 = monkeys[*i1].get_value(monkeys);
                match op {
                    Operator::Add | Operator::Mul => {
                        op.get_opposite().get_value(expected_result, v1)
                    }
                    Operator::Sub | Operator::Div => op.get_value(v1, expected_result),
                }
            }
        }
    }
}

pub fn solve_p1(input: &str) -> i64 {
    let (monkeys, root_index, _) = parse_input(input);
    monkeys[root_index].get_value(&monkeys)
}

pub fn solve_p2(input: &str) -> i64 {
    let (monkeys, root_index, humn_index) = parse_input(input);
    let mut q: VecDeque<(usize, i64)> = VecDeque::new(); //(index, expected value)
    if let Monkey::Expression(i1, i2, _) = monkeys[root_index] {
        q.push_back((i2, monkeys[i1].get_value(&monkeys)));
        q.push_back((i1, monkeys[i2].get_value(&monkeys)));
    }

    while let Some((i, expected)) = q.pop_front() {
        if i == humn_index {
            return expected;
        }

        if let Monkey::Expression(i1, i2, _) = monkeys[i] {
            q.push_back((i1, monkeys[i].get_expected1(expected, &monkeys)));
            q.push_back((i2, monkeys[i].get_expected2(expected, &monkeys)));
        }
    }

    -1
}

fn parse_input(input: &str) -> (Vec<Monkey>, usize, usize) {
    let id_to_index: HashMap<&str, usize> = HashMap::from_iter(
        input
            .lines()
            .enumerate()
            .map(|(i, line)| (line.split_once(':').map(|(id, _)| id).unwrap(), i)),
    );

    (
        Vec::from_iter(input.lines().map(|line| {
            let (_, definition) = line.split_once(": ").unwrap();
            match definition.parse::<i64>() {
                Ok(c) => Monkey::Const(c),
                Err(_) => {
                    let mut parts = definition.split(' ');
                    let i1 = id_to_index.get(parts.next().unwrap()).unwrap();
                    let operator = match parts.next().unwrap() {
                        "+" => Operator::Add,
                        "-" => Operator::Sub,
                        "*" => Operator::Mul,
                        "/" => Operator::Div,
                        _ => panic!("bad input"),
                    };
                    let i2 = id_to_index.get(parts.next().unwrap()).unwrap();
                    Monkey::Expression(*i1, *i2, operator)
                }
            }
        })),
        *id_to_index.get("root").unwrap(),
        *id_to_index.get("humn").unwrap(),
    )
}

fn main() {
    let input = include_str!("../input.txt");

    let start = std::time::Instant::now();
    let answer = solve_p1(input);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:.1?}");

    let start = std::time::Instant::now();
    let answer = solve_p2(input);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, elapsed: {elapsed:.1?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 152);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 301);
    }
}
