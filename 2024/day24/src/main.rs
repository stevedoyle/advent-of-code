use std::{
    cmp::Reverse,
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use regex::Regex;

#[derive(Debug, Default, PartialEq)]
struct Gate {
    id: String,
    in_a: Option<String>,
    in_b: Option<String>,
    operation: Option<Operation>,
    out: Option<u8>,
}

impl Gate {
    fn new(
        id: String,
        in_a: Option<String>,
        operation: Option<Operation>,
        in_b: Option<String>,
        value: Option<u8>,
    ) -> Self {
        Self {
            id,
            in_a,
            in_b,
            operation,
            out: value,
        }
    }

    fn evaluate(&mut self, a: u8, b: u8) {
        if self.out.is_some() {
            return;
        }

        match self.operation {
            Some(Operation::And) => {
                self.out = Some(a & b);
            }
            Some(Operation::Or) => {
                self.out = Some(a | b);
            }
            Some(Operation::Xor) => {
                self.out = Some(a ^ b);
            }
            _ => panic!("Invalid operation"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Operation {
    And,
    Or,
    Xor,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Operation::And),
            "OR" => Ok(Operation::Or),
            "XOR" => Ok(Operation::Xor),
            _ => Err(()),
        }
    }
}

fn parse_input(input: &str) -> HashMap<String, Gate> {
    let mut gates = HashMap::new();
    let (wires, equations) = input.split_once("\n\n").unwrap();
    wires.lines().for_each(|line| {
        let (name, value) = line.split_once(": ").unwrap();
        gates.insert(
            name.to_string(),
            Gate::new(name.to_string(), None, None, None, value.parse().ok()),
        );
    });
    equations.lines().for_each(|line| {
        let (in_parts, out) = line.split_once(" -> ").unwrap();
        let mut parts = in_parts.split_whitespace();
        gates.insert(
            out.to_string(),
            Gate::new(
                out.to_string(),
                Some(parts.next().unwrap().to_string()),
                Some(parts.next().unwrap().parse().unwrap()),
                Some(parts.next().unwrap().to_string()),
                None,
            ),
        );
    });
    gates
}

fn solve_p1(input: &str) -> usize {
    let mut gates = parse_input(input);
    let mut queue = VecDeque::new();
    gates.iter().for_each(|(name, gate)| {
        if gate.out.is_none() {
            queue.push_back(name.clone());
        }
    });
    while let Some(name) = queue.pop_front() {
        let gate = gates.get(&name).unwrap();
        let a = gates.get(gate.in_a.as_ref().unwrap()).unwrap();
        let b = gates.get(gate.in_b.as_ref().unwrap()).unwrap();
        if a.out.is_some() && b.out.is_some() {
            let a = a.out.unwrap();
            let b = b.out.unwrap();
            let gate = gates.get_mut(&name).unwrap();
            gate.evaluate(a, b);
        } else {
            queue.push_back(name);
        }
    }
    let mut zgates: Vec<&Gate> = gates
        .iter()
        .filter(|(_, gate)| gate.id.starts_with("z"))
        .map(|(_, gate)| gate)
        .collect();
    zgates.sort_unstable_by_key(|gate| Reverse(&gate.id));
    zgates
        .iter()
        .fold(0, |acc, gate| acc << 1 | gate.out.unwrap() as usize)
}

fn parse_input_p2(input: &str) -> (&str, Vec<[&str; 4]>) {
    let (wire_values, gate_connections) = input.split_once("\n\n").unwrap();
    let gate_re = Regex::new(r"(.{3}) (AND|OR|XOR) (.{3}) -> (.{3})").unwrap();
    let gate_connections = gate_connections
        .lines()
        .map(|line| {
            let caps = gate_re.captures(line).unwrap();
            let (_, s) = caps.extract::<4>();
            s
        })
        .collect::<Vec<_>>();
    (wire_values, gate_connections)
}

fn solve_p2(input: &str) -> String {
    let (_, gate_connections) = parse_input_p2(input);
    let mut wire_map: HashMap<&str, Vec<(&str, &str)>> = HashMap::new();

    for &[a, op, b, ret] in gate_connections.iter() {
        wire_map.entry(a).or_default().push((op, ret));
        wire_map.entry(b).or_default().push((op, ret));
    }

    let mut wrong_wires = vec![];
    for &[lhs, op, rhs, ret] in gate_connections.iter() {
        // basically we ensure the adder looks like this:
        // https://en.wikipedia.org/wiki/Adder_(electronics)#/media/File:Fulladder.gif
        let chained_ops = wire_map.get(&ret);
        let chained_ops_contain =
            |op| chained_ops.is_some_and(|v| v.iter().any(|a| a.0 == op));

        let has_chained_xor = chained_ops_contain("XOR");
        let has_chained_and = chained_ops_contain("AND");
        let has_chained_or = chained_ops_contain("OR");
        let takes_first_input = lhs.ends_with("00") && rhs.ends_with("00");
        let takes_input_bit = (lhs.starts_with('x') && rhs.starts_with('y'))
            || (rhs.starts_with('x') && lhs.starts_with('y'));
        let outputs_bit = ret.starts_with('z');
        let outputs_last_bit = ret == "z45";

        let valid = match op {
            "XOR" => {
                // XOR only outputs a bit if it doesn't take an input bit
                if !takes_input_bit && outputs_bit {
                    true
                // XOR only takes an input bit if a XOR follows it
                } else if takes_input_bit && has_chained_xor {
                    true
                // unless the input bits are the first bits (no carryover bit exists)
                } else { takes_first_input && outputs_bit }
            }
            "OR" => {
                // OR either outputs into z45 or an AND and XOR (carryover bit)
                outputs_last_bit || (has_chained_and && has_chained_xor)
            }
            "AND" => {
                // ANDs only lead into ORs
                if has_chained_or {
                    true
                // unless the input bits are the first bits (no carryover bit exists)
                } else { takes_first_input }
            }
            _ => {
                unreachable!()
            }
        };
        if !valid {
            wrong_wires.push(ret);
        }
    }
    wrong_wires.sort_unstable();
    wrong_wires.join(",").to_string()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let start = std::time::Instant::now();
    let answer = solve_p1(&input);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:.1?}");

    let start = std::time::Instant::now();
    let answer = solve_p2(&input);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, elapsed: {elapsed:.1?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_with_test_input() {
        let input1 = std::fs::read_to_string("test_input1.txt").unwrap();
        let answer = solve_p1(&input1);
        assert_eq!(answer, 4);
        let input2 = std::fs::read_to_string("test_input2.txt").unwrap();
        let answer = solve_p1(&input2);
        assert_eq!(answer, 2024);
    }
}
