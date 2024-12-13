use fitting::{linalg, ndarray::array};
use regex::Regex;

#[derive(Debug, Default)]
struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

fn parse_input(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();
    let a_pattern = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let b_pattern = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_pattern = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let machine_text = input.split("\n\n");
    for m in machine_text {
        let mut machine = Machine::default();
        let a = a_pattern.captures(m).unwrap();
        let b = b_pattern.captures(m).unwrap();
        let prize = prize_pattern.captures(m).unwrap();
        machine.a = (a[1].parse().unwrap(), a[2].parse().unwrap());
        machine.b = (b[1].parse().unwrap(), b[2].parse().unwrap());
        machine.prize = (prize[1].parse().unwrap(), prize[2].parse().unwrap());
        machines.push(machine);
    }
    machines
}

fn solve(machines: &[Machine], limit: usize, correction: usize) -> usize {
    let mut tokens = 0;
    for m in machines.iter() {
        let coef = array![[m.a.0 as f64, m.b.0 as f64], [m.a.1 as f64, m.b.1 as f64]];
        let rhs = array![
            (m.prize.0 + correction) as f64,
            (m.prize.1 + correction) as f64
        ];
        let sol = linalg::solve(coef, rhs).unwrap();

        if sol[0] >= 0.0 && sol[1] >= 0.0 {
            let a_presses = sol[0].round() as usize;
            let b_presses = sol[1].round() as usize;

            // Check that the solution is correct in case of any float issues.
            if m.a.0 * a_presses + m.b.0 * b_presses != m.prize.0 + correction
                || m.a.1 * a_presses + m.b.1 * b_presses != m.prize.1 + correction
                || (limit > 0 && a_presses > limit)
                || (limit > 0 && b_presses > limit)
            {
                continue;
            }
            tokens += a_presses * 3 + b_presses;
        }
    }
    tokens
}

fn solve_p1(machines: &[Machine]) -> usize {
    solve(machines, 100, 0)
}

fn solve_p2(machines: &[Machine]) -> usize {
    solve(machines, 0, 10000000000000)
}

fn main() {
    let input = include_str!("../input.txt");
    let machines = parse_input(input);

    let start = std::time::Instant::now();
    let answer = solve_p1(&machines);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:.1?}");

    let start = std::time::Instant::now();
    let answer = solve_p2(&machines);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, elapsed: {elapsed:.1?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_with_test_input() {
        let input = std::fs::read_to_string("test.txt").unwrap();
        let machines = parse_input(&input);

        let answer = solve_p1(&machines);
        assert_eq!(answer, 480);
        let answer = solve_p2(&machines);
        assert_eq!(answer, 875318608908);
    }
}
