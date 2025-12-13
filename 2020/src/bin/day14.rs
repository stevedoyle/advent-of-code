use std::collections::HashMap;

use aoc2020::*;

struct Instruction {
    mask: String,
    address: usize,
    value: usize,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let mut current_mask = String::new();
    let mut instructions: Vec<Instruction> = Vec::new();
    input.lines().for_each(|line| {
        if line.starts_with("mask") {
            let val = line.split(" = ").nth(1).unwrap().to_string();
            current_mask = val.clone();
        } else {
            let parts: Vec<&str> = line.split(" = ").collect();
            let address = parts[0][4..parts[0].len() - 1].parse().unwrap();
            let value = parts[1].parse().unwrap();
            instructions.push(Instruction {
                mask: current_mask.clone(),
                address,
                value,
            });
        }
    });
    instructions
}

fn solve_p1(input: &str) -> usize {
    let instructions = parse_input(input);
    let mut memory = HashMap::<usize, usize>::new();

    for instr in instructions {
        let mut value = instr.value;
        for (i, ch) in instr.mask.chars().rev().enumerate() {
            match ch {
                '0' => value &= !(1 << i),
                '1' => value |= 1 << i,
                'X' => {}
                _ => panic!("Unexpected character in mask"),
            }
        }
        memory.insert(instr.address, value);
    }

    memory.iter().map(|(_, &v)| v).sum()
}

fn solve_p2(input: &str) -> usize {
    let instructions = parse_input(input);
    let mut memory = HashMap::<usize, usize>::new();

    for instr in instructions {
        let mut base_address = instr.address;
        let mut floating_bits = Vec::new();

        for (i, ch) in instr.mask.chars().rev().enumerate() {
            match ch {
                '0' => {}
                '1' => base_address |= 1 << i,
                'X' => floating_bits.push(i),
                _ => panic!("Unexpected character in mask"),
            }
        }

        let combinations: usize = 1 << floating_bits.len();
        for combo in 0..combinations {
            let mut address = base_address;
            for (j, bit_pos) in floating_bits.iter().enumerate() {
                if (combo & (1 << j)) != 0 {
                    address |= 1 << bit_pos;
                } else {
                    address &= !(1 << bit_pos);
                }
            }
            memory.insert(address, instr.value);
        }
    }
    memory.iter().map(|(_, &v)| v).sum()
}

fn main() {
    let input = read_input(14);

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
    fn test_p1() {
        let input = read_test_input(14);
        let answer = solve_p1(&input);
        assert_eq!(answer, 165);
    }

    #[test]
    fn test_p2() {
        let input = std::fs::read_to_string("inputs/day14_p2_test.txt").unwrap();
        let answer = solve_p2(&input);
        assert_eq!(answer, 208);
    }
}
