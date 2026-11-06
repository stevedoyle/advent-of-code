use std::collections::HashSet;

use aoc2020::*;

const DAY: u8 = 8;

fn parse_input(input: &str) -> Vec<(&str, i32)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let command = parts.next().unwrap();
            let value = parts.next().unwrap().parse::<i32>().unwrap();
            (command, value)
        })
        .collect()
}

fn solve_p1(input: &str) -> usize {
    let program = parse_input(input);
    let mut accumulator = 0;
    let mut pc: usize = 0;
    let mut executed = HashSet::new();
    while !executed.contains(&pc) {
        executed.insert(pc);
        let (command, value) = &program[pc];
        match *command {
            "acc" => {
                accumulator += *value;
                pc += 1;
            }
            "jmp" => {
                pc = (pc as i32 + *value) as usize;
            }
            "nop" => {
                pc += 1;
            }
            _ => panic!("Unknown command: {}", command),
        }
    }
    accumulator as usize
}

fn solve_p2(input: &str) -> usize {
    let program = parse_input(input);
    for i in 0..program.len() {
        let mut accumulator = 0;
        let mut pc: usize = 0;
        let mut executed = HashSet::new();
        while !executed.contains(&pc) {
            if pc == program.len() {
                return accumulator as usize;
            }
            executed.insert(pc);
            let (command, value) = &program[pc];
            match *command {
                "acc" => {
                    accumulator += *value;
                    pc += 1;
                }
                "jmp" => {
                    if i == pc {
                        pc += 1;
                    } else {
                        pc = (pc as i32 + *value) as usize;
                    }
                }
                "nop" => {
                    if i == pc {
                        pc = (pc as i32 + *value) as usize;
                    } else {
                        pc += 1;
                    }
                }
                _ => panic!("Unknown command: {}", command),
            }
        }
    }
    0
}

fn main() {
    run(DAY, solve_p1, solve_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve_p1(&read_test_input(DAY, 1)), 5);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_p2(&read_test_input(DAY, 1)), 8);
    }
}
