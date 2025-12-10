use std::str::FromStr;

use aoc2025::*;
use good_lp::*;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Button {
    wires: Vec<u16>,
    value: u16,
}

impl FromStr for Button {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let wires: Vec<u16> = s
            .trim_matches(|c| c == '(' || c == ')')
            .split(',')
            .map(|num_str| num_str.parse::<u16>().unwrap())
            .collect();
        let mut value = 0;
        for wire in &wires {
            value |= 1 << wire;
        }
        Ok(Button { wires, value })
    }
}

#[derive(Debug, Clone)]
struct Machine {
    indicator: u16,
    buttons: Vec<Button>,
    joltage: Vec<u16>,
}
impl std::str::FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<&str>>();

        let indicator = parse_indicator(parts[0]);
        let buttons = parse_buttons(&parts[1..parts.len() - 1]);
        let joltage = parse_joltage(parts.last().unwrap());

        Ok(Machine {
            indicator,
            buttons,
            joltage,
        })
    }
}

fn parse_indicator(input: &str) -> u16 {
    // [..#..##..#.]
    let mut indicator = 0;
    for c in input.chars().rev() {
        indicator = match c {
            '#' => indicator << 1 | 1,
            '.' => indicator << 1,
            '[' | ']' => indicator,
            _ => panic!("Invalid character in indicator: {}", c),
        };
    }
    indicator
}

fn parse_buttons(input: &[&str]) -> Vec<Button> {
    input.iter().map(|s| s.parse().unwrap()).collect()
}

fn parse_joltage(input: &str) -> Vec<u16> {
    // {5,10,15}
    input
        .trim_matches(|c| c == '{' || c == '}')
        .split(',')
        .map(|s| s.parse::<u16>().unwrap())
        .collect()
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| line.parse::<Machine>().unwrap())
        .collect()
}

fn min_presses_for_indicator(machine: &Machine) -> usize {
    for i in 0..=machine.buttons.len() {
        for combo in machine.buttons.iter().combinations(i) {
            let combined = combo.iter().fold(0u16, |acc, &b| acc ^ b.value);
            if combined == machine.indicator {
                return i;
            }
        }
    }
    0
}

fn min_presses_for_joltage(machine: &Machine) -> usize {
    let mut vars = ProblemVariables::new();

    // Press Counts represents the number of times each button is pressed
    // Without specifying a floor, the minimum number of presses is
    // negative infinity.
    let mut button_presses = Vec::new();
    for _ in 0..machine.buttons.len() {
        // register each button's press count as a variable with the solver
        let variable = vars.add(variable().min(0).integer());
        // keep track of the variables so we can state the objective in terms
        // of them below
        button_presses.push(variable);
    }

    // We state the problem: smallest sum of all button presses
    let mut problem = good_lp::highs(vars.minimise(button_presses.iter().sum::<Expression>()));

    // Available solvers listed at https://docs.rs/crate/good_lp/latest
    // Use of the `highs`` solver as it:
    // - Requires no extra libraries (ruling out coin_cbc)
    // - Supports integers (clarabel doesn't support these, despite the
    //   docstring, highs does)
    // - Gives the right answer (sorry microlp)
    // - Is "fast"
    // - Doesn't require any additional mucking about to get it working
    //   (most of the others)

    // The value of each joltage counter is derived from the buttons pressed
    // we have one expression per counter
    let mut expressions =
        vec![Expression::with_capacity(machine.buttons.len()); machine.joltage.len()];

    for (i, button) in machine.buttons.iter().enumerate() {
        for x in button.wires.iter() {
            // for each button pressed, add the number of times it is pressed
            // to the total for the joltage counters it increments
            expressions[*x as usize] += button_presses[i];
        }
    }
    for (e, j) in expressions.into_iter().zip(machine.joltage.clone()) {
        // for each of the expressions for a given joltage counter's value,
        // add the constraint that the result of the expression must be the desired
        // joltage
        problem.add_constraint(e.eq(j as f64));
    }
    let solution = problem.solve().unwrap();
    button_presses
        .iter()
        .map(|&v| solution.value(v))
        .sum::<f64>() as usize
}

fn solve_p1(input: &str) -> usize {
    let machines = parse_input(input);
    machines.iter().map(min_presses_for_indicator).sum()
}

fn solve_p2(input: &str) -> usize {
    let machines = parse_input(input);
    machines.iter().map(min_presses_for_joltage).sum()
}

fn main() {
    let input = read_input(10);

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
        let input = read_test_input(10);
        let answer = solve_p1(&input);
        assert_eq!(answer, 7);
        let answer = solve_p2(&input);
        assert_eq!(answer, 33);
    }

    #[test]
    fn test_parse_machine() {
        let input = "[..#.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine: Machine = input.parse().unwrap();
        assert_eq!(machine.indicator, 0b0100 as u16); // Binary for ..#.
                                                      // Buttons: (3)=0b1000, (1,3)=0b1010, (2)=0b0100, (2,3)=0b1100, (0,2)=0b0101, (0,1)=0b0011
        assert_eq!(
            machine.buttons.iter().map(|b| b.value).collect_vec(),
            vec![0b1000, 0b1010, 0b0100, 0b1100, 0b0101, 0b0011]
        );
        assert_eq!(machine.joltage, vec![3, 5, 4, 7]);
    }

    #[test]
    fn test_fewest_indicator_button_presses() {
        let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let machine: Machine = input.parse().unwrap();
        let presses = min_presses_for_indicator(&machine);
        assert_eq!(presses, 2);
    }

    #[test]
    fn test_fewest_joltage_button_presses() {
        let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let machine: Machine = input.parse().unwrap();
        let presses = min_presses_for_joltage(&machine);
        assert_eq!(presses, 11);
    }
}
