use std::fmt;

#[derive(Debug, Default, Clone)]
struct Computer {
    instructions: Vec<Instruction>,
    pc: usize,
    a: usize,
    b: usize,
    c: usize,
}

impl Computer {
    fn new() -> Self {
        Self::default()
    }

    fn run(&mut self) -> String {
        let mut output = String::new();
        while self.pc < self.instructions.len() {
            let instr = &self.instructions[self.pc];
            // print!(
            //     "[a={} b={} c={}] {}: {};",
            //     self.a, self.b, self.c, self.pc, instr
            // );
            match instr.opcode {
                Opcode::Adv => {
                    let operand = self.combo(instr.operand);
                    self.a /= usize::pow(2, operand as u32);
                    self.pc += 1;
                }
                Opcode::Bxl => {
                    self.b ^= instr.operand as usize;
                    self.pc += 1;
                }
                Opcode::Bst => {
                    let operand = self.combo(instr.operand);
                    self.b = operand as usize % 8;
                    self.pc += 1;
                }
                Opcode::Jnz => {
                    if self.a != 0 {
                        self.pc = instr.operand as usize;
                    } else {
                        self.pc += 1;
                    }
                }
                Opcode::Bxc => {
                    self.b ^= self.c;
                    self.pc += 1;
                }
                Opcode::Out => {
                    let operand = self.combo(instr.operand);
                    if output.len() > 0 {
                        output.push(',');
                    }
                    output.push_str((operand % 8).to_string().as_str());
                    self.pc += 1;
                }
                Opcode::Bdv => {
                    let operand = self.combo(instr.operand);
                    self.b = self.a / usize::pow(2, operand as u32);
                    self.pc += 1;
                }
                Opcode::Cdv => {
                    let operand = self.combo(instr.operand);
                    self.c = self.a / usize::pow(2, operand as u32);
                    self.pc += 1;
                }
            }
            // println!(" [a={} b={} c={}]", self.a, self.b, self.c);
        }
        output
    }

    fn combo(&self, operand: u32) -> usize {
        match operand {
            0..=3 => operand as usize,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid operand"),
        }
    }
}

impl fmt::Display for Computer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Computer {{ a: {}, b: {}, c: {}, pc: {} }}\n",
            self.a, self.b, self.c, self.pc
        )?;
        for instr in &self.instructions {
            write!(f, "{}\n", instr)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    opcode: Opcode,
    operand: u32,
}

impl Instruction {
    fn new(opcode: usize, operand: u32) -> Self {
        let opcode = Opcode::from(opcode);
        Self { opcode, operand }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {}", self.opcode, self.operand)
    }
}

#[derive(Debug, Clone)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Opcode {
    fn from(opcode: usize) -> Self {
        match opcode {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("Invalid opcode"),
        }
    }
}

fn parse_input(input: &str) -> Computer {
    let mut computer = Computer::new();
    let mut line_iter = input.lines();
    computer.a = line_iter
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    computer.b = line_iter
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    computer.c = line_iter
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    line_iter.next(); // Blank line separating the registers and instructions

    let program_data = line_iter.next().unwrap().split_once(" ").unwrap().1;
    program_data
        .split(',')
        .collect::<Vec<_>>()
        .chunks(2)
        .for_each(|chunk| {
            let opcode = chunk[0].parse().unwrap();
            let operand = chunk[1].parse().unwrap();
            computer
                .instructions
                .push(Instruction::new(opcode, operand));
        });
    computer
}

fn parse_program(input: &str) -> String {
    input
        .lines()
        .skip(4)
        .next()
        .unwrap()
        .split_once(" ")
        .unwrap()
        .1
        .to_string()
}

fn solve_p1(input: &str) -> String {
    let mut computer = parse_input(input);
    computer.run()
}

fn solve_p2(input: &str) -> usize {
    let computer = parse_input(input);
    let program_str = parse_program(input);

    let program: Vec<usize> = program_str
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();

    let mut a = 0;
    for n in 1..=program.len() {
        let target = program[program.len() - n..].to_vec();
        let mut new_a = a << 3;
        loop {
            let mut comp = computer.clone();
            comp.a = new_a;
            let output = comp.run();
            let output: Vec<usize> = output.split(',').map(|x| x.parse().unwrap()).collect();
            if output == target {
                a = new_a;
                break;
            }
            new_a += 1;
        }
    }
    a
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
    fn test_solve_p1_with_test_input() {
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let answer = solve_p1(&input);
        assert_eq!(answer, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_solve_p2_with_test_input() {
        let input = std::fs::read_to_string("test_input_2.txt").unwrap();
        let answer = solve_p2(&input);
        assert_eq!(answer, 117440);
    }

    #[test]
    fn test_bst() {
        let mut computer = Computer::new();
        computer.c = 9;
        computer.instructions.push(Instruction::new(2, 6));
        computer.run();
        assert_eq!(computer.b, 1);
    }

    #[test]
    fn test_out() {
        let mut computer = Computer::new();
        computer.a = 10;
        computer.instructions.push(Instruction::new(5, 0));
        computer.instructions.push(Instruction::new(5, 1));
        computer.instructions.push(Instruction::new(5, 4));
        let output = computer.run();
        assert_eq!(output, "0,1,2");
    }

    #[test]
    fn test_computer_run1() {
        let mut computer = Computer::new();
        computer.a = 2024;
        computer.instructions.push(Instruction::new(0, 1));
        computer.instructions.push(Instruction::new(5, 4));
        computer.instructions.push(Instruction::new(3, 0));
        let output = computer.run();
        assert_eq!(output, "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(computer.a, 0);
    }

    #[test]
    fn test_computer_run2() {
        let mut computer = Computer::new();
        computer.b = 29;
        computer.instructions.push(Instruction::new(1, 7));
        computer.run();
        assert_eq!(computer.b, 26);
    }

    #[test]
    fn test_computer_run3() {
        let mut computer = Computer::new();
        computer.b = 2024;
        computer.c = 43690;
        computer.instructions.push(Instruction::new(4, 0));
        computer.run();
        assert_eq!(computer.b, 44354);
    }
}
