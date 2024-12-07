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

fn parse_input(input: &str) -> Vec<Equation> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn is_solvable(goal: usize, operands: &[usize], p2: bool) -> bool {
    if operands.len() == 1 {
        return operands[0] == goal;
    }

    let n = operands.len();
    let head = operands[n - 1];
    let tail = &operands[..n - 1];
    if goal > head {
        let subgoal1 = goal - head;
        if is_solvable(subgoal1, tail, p2) {
            return true;
        }
    }

    let subgoal2 = goal / head;
    if subgoal2 * head == goal && is_solvable(subgoal2, tail, p2) {
        return true;
    }
    if p2 && goal > head {
        let mut h = head;
        let mut mul = 1;
        while h > 0 {
            h /= 10;
            mul *= 10;
        }
        let subgoal3 = (goal - head) / mul;
        if subgoal3 * mul + head == goal && is_solvable(subgoal3, tail, p2) {
            return true;
        }
    }
    false
}

fn solve_p1(equations: &[Equation]) -> usize {
    equations
        .iter()
        .filter(|eq| {
            is_solvable(
                eq.target,
                &eq.operands,
                false,
            )
        })
        .map(|eq| eq.target)
        .sum()
}

fn solve_p2(equations: &[Equation]) -> usize {
    equations
        .iter()
        .filter(|eq| {
            is_solvable(
                eq.target,
                &eq.operands,
                true,
            )
        })
        .map(|eq| eq.target)
        .sum()
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
}
