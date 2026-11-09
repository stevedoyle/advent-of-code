use aoc2015::*;
use std::collections::HashMap;

const DAY: u8 = 7;

/// A gate input: either a literal signal or the name of another wire.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Operand {
    Value(u16),
    Wire(String),
}

impl Operand {
    fn parse(s: &str) -> Self {
        match s.parse() {
            Ok(n) => Operand::Value(n),
            Err(_) => Operand::Wire(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    /// `123 -> x` or `lx -> a`
    Set(Operand),
    /// `NOT x -> h`
    Not(Operand),
    /// `x AND y -> d`, `1 AND x -> y`
    And(Operand, Operand),
    /// `x OR y -> e`
    Or(Operand, Operand),
    /// `x LSHIFT 2 -> f`
    LShift(Operand, u16),
    /// `y RSHIFT 2 -> g`
    RShift(Operand, u16),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    output: String,
    operation: Operation,
}

impl Instruction {
    fn parse(line: &str) -> Self {
        let (expr, output) = line
            .split_once(" -> ")
            .unwrap_or_else(|| panic!("missing '->' in: {line}"));
        let tokens: Vec<&str> = expr.split_whitespace().collect();
        let shift = |n: &str| -> u16 {
            n.parse()
                .unwrap_or_else(|_| panic!("invalid shift amount in: {line}"))
        };
        let operation = match tokens.as_slice() {
            [a] => Operation::Set(Operand::parse(a)),
            ["NOT", a] => Operation::Not(Operand::parse(a)),
            [a, "AND", b] => Operation::And(Operand::parse(a), Operand::parse(b)),
            [a, "OR", b] => Operation::Or(Operand::parse(a), Operand::parse(b)),
            [a, "LSHIFT", n] => Operation::LShift(Operand::parse(a), shift(n)),
            [a, "RSHIFT", n] => Operation::RShift(Operand::parse(a), shift(n)),
            _ => panic!("invalid instruction: {line}"),
        };
        Instruction {
            output: output.to_string(),
            operation,
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::parse).collect()
}

/// Maps each wire name to the operation that drives it.
type Circuit<'a> = HashMap<&'a str, &'a Operation>;

fn eval_operand(op: &Operand, circuit: &Circuit, cache: &mut HashMap<String, u16>) -> u16 {
    match op {
        Operand::Value(n) => *n,
        Operand::Wire(name) => resolve(name, circuit, cache),
    }
}

/// Signal on `wire`, resolving its inputs first. Results are memoized in `cache`.
///
/// Instructions can appear in any order in the input, so a wire is evaluated
/// on demand instead of in file order.
fn resolve(wire: &str, circuit: &Circuit, cache: &mut HashMap<String, u16>) -> u16 {
    if let Some(&v) = cache.get(wire) {
        return v;
    }
    let operation = circuit
        .get(wire)
        .unwrap_or_else(|| panic!("no instruction drives wire '{wire}'"));
    let value = match operation {
        Operation::Set(a) => eval_operand(a, circuit, cache),
        Operation::Not(a) => !eval_operand(a, circuit, cache),
        Operation::And(a, b) => eval_operand(a, circuit, cache) & eval_operand(b, circuit, cache),
        Operation::Or(a, b) => eval_operand(a, circuit, cache) | eval_operand(b, circuit, cache),
        Operation::LShift(a, n) => eval_operand(a, circuit, cache) << n,
        Operation::RShift(a, n) => eval_operand(a, circuit, cache) >> n,
    };
    cache.insert(wire.to_string(), value);
    value
}

fn build_circuit(instructions: &[Instruction]) -> Circuit<'_> {
    instructions
        .iter()
        .map(|i| (i.output.as_str(), &i.operation))
        .collect()
}

fn solve_p1(input: &str, wire: &str) -> usize {
    let instructions = parse_input(input);
    let circuit = build_circuit(&instructions);
    resolve(wire, &circuit, &mut HashMap::new()) as usize
}

/// Run the circuit once, feed `a` back into `b`, then run it again with fresh wires.
fn solve_p2(input: &str) -> usize {
    let instructions = parse_input(input);
    let mut circuit = build_circuit(&instructions);
    let a = resolve("a", &circuit, &mut HashMap::new());
    let override_b = Operation::Set(Operand::Value(a));
    circuit.insert("b", &override_b);
    resolve("a", &circuit, &mut HashMap::new()) as usize
}

fn main() {
    run(DAY, |input| solve_p1(input, "a"), solve_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ins(output: &str, operation: Operation) -> Instruction {
        Instruction {
            output: output.to_string(),
            operation,
        }
    }

    fn wire(name: &str) -> Operand {
        Operand::Wire(name.to_string())
    }

    #[test]
    fn parse_each_gate_type() {
        use Operand::Value;
        assert_eq!(
            Instruction::parse("123 -> x"),
            ins("x", Operation::Set(Value(123)))
        );
        assert_eq!(
            Instruction::parse("lx -> a"),
            ins("a", Operation::Set(wire("lx")))
        );
        assert_eq!(
            Instruction::parse("NOT x -> h"),
            ins("h", Operation::Not(wire("x")))
        );
        assert_eq!(
            Instruction::parse("x AND y -> d"),
            ins("d", Operation::And(wire("x"), wire("y")))
        );
        assert_eq!(
            Instruction::parse("1 AND x -> y"),
            ins("y", Operation::And(Value(1), wire("x")))
        );
        assert_eq!(
            Instruction::parse("x OR y -> e"),
            ins("e", Operation::Or(wire("x"), wire("y")))
        );
        assert_eq!(
            Instruction::parse("x LSHIFT 2 -> f"),
            ins("f", Operation::LShift(wire("x"), 2))
        );
        assert_eq!(
            Instruction::parse("y RSHIFT 2 -> g"),
            ins("g", Operation::RShift(wire("y"), 2))
        );
    }

    #[test]
    fn parse_test_input() {
        assert_eq!(parse_input(&read_test_input(DAY, 1)).len(), 8);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_p1(&read_test_input(DAY, 1), "d"), 72);
        assert_eq!(solve_p1(&read_test_input(DAY, 1), "e"), 507);
        assert_eq!(solve_p1(&read_test_input(DAY, 1), "f"), 492);
        assert_eq!(solve_p1(&read_test_input(DAY, 1), "g"), 114);
        assert_eq!(solve_p1(&read_test_input(DAY, 1), "h"), 65412);
        assert_eq!(solve_p1(&read_test_input(DAY, 1), "i"), 65079);
        assert_eq!(solve_p1(&read_test_input(DAY, 1), "x"), 123);
        assert_eq!(solve_p1(&read_test_input(DAY, 1), "y"), 456);
    }

    #[test]
    fn part2() {
        // First pass: a = 5 << 1 = 10. Then b is overridden to 10, so a = 10 << 1 = 20.
        assert_eq!(solve_p2("5 -> b\nb LSHIFT 1 -> a"), 20);
    }
}
