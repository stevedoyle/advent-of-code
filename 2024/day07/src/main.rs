use std::str::FromStr;

#[derive(Debug)]
struct Equation {
    target: usize,
    operands: Vec<usize>,
}

impl FromStr for Equation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (target, rest) = s.split_once(": ").unwrap();
        let target = target.parse().unwrap();

        let operands = rest
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        Ok(Self { target, operands })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

fn parse_input(input: &str) -> Vec<Equation> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn generate_combinations(n: usize, ops: &[Operator]) -> Vec<Vec<Operator>> {
    fn backtrack(
        n: usize,
        ops: &[Operator],
        current: &mut Vec<Operator>,
        results: &mut Vec<Vec<Operator>>,
    ) {
        if current.len() == n {
            results.push(current.clone());
            return;
        }

        for &op in ops {
            current.push(op);
            backtrack(n, ops, current, results);
            current.pop();
        }
    }

    let mut results = Vec::new();
    backtrack(n, ops, &mut Vec::new(), &mut results);
    results
}

fn concatenate(a: usize, b: usize) -> usize {
    let mut multiplier = 1;

    while multiplier < b {
        multiplier *= 10;
    }

    a * multiplier + b
}

fn is_solvable(eq: &Equation, ops: &[Operator]) -> bool {
    let num_operators = eq.operands.len() - 1;
    if num_operators == 0 {
        return eq.operands[0] == eq.target;
    }
    let operator_combinations = generate_combinations(num_operators, ops);
    for combo in operator_combinations {
        let mut result = eq.operands[0];
        for (i, &op) in combo.iter().enumerate() {
            match op {
                Operator::Add => result += eq.operands[i + 1],
                Operator::Multiply => result *= eq.operands[i + 1],
                Operator::Concatenate => result = concatenate(result, eq.operands[i + 1]),
            }
        }
        if result == eq.target {
            return true;
        }
    }
    false
}

fn solve(equations: &[Equation], ops: &[Operator]) -> usize {
    equations
        .iter()
        .filter(|eq| is_solvable(eq, ops))
        .map(|eq| eq.target)
        .sum()
}

fn solve_p1(equations: &[Equation]) -> usize {
    let operators = [Operator::Add, Operator::Multiply];
    solve(equations, &operators)
}

fn solve_p2(equations: &[Equation]) -> usize {
    let operators = [Operator::Add, Operator::Multiply, Operator::Concatenate];
    solve(equations, &operators)
}

fn main() {
    let input = include_str!("../input.txt");
    let equations = parse_input(input);

    let start = std::time::Instant::now();
    let answer = solve_p1(&equations);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:.1?}");

    let start = std::time::Instant::now();
    let answer = solve_p2(&equations);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, elapsed: {elapsed:.1?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_with_test_input() {
        let input = include_str!("../test.txt");
        let equations = parse_input(input);

        let answer = solve_p1(&equations);
        assert_eq!(answer, 3749);
        let answer = solve_p2(&equations);
        assert_eq!(answer, 11387);
    }

    #[test]
    fn test_concatenate() {
        assert_eq!(concatenate(1, 2), 12);
        assert_eq!(concatenate(12, 3), 123);
        assert_eq!(concatenate(123, 4), 1234);
        assert_eq!(concatenate(12, 34), 1234);
    }
}
