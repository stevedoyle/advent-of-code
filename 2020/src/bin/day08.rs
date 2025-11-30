use std::collections::HashSet;

use aoc2020::*;

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
    let input = read_input(8);

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
        let input = read_test_input(8);
        let answer = solve_p1(&input);
        assert_eq!(answer, 5);
        let answer = solve_p2(&input);
        assert_eq!(answer, 8);
    }
}
