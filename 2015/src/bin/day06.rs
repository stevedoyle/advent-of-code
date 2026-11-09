use aoc2015::*;

const DAY: u8 = 6;

enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Instruction {
    command: Command,
    start: (usize, usize),
    end: (usize, usize),
}

impl Instruction {
    fn parse(s: &str) -> Self {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let (command, start_idx) = match parts[0] {
            "turn" => match parts[1] {
                "on" => (Command::TurnOn, 2),
                "off" => (Command::TurnOff, 2),
                _ => panic!("Invalid command"),
            },
            "toggle" => (Command::Toggle, 1),
            _ => panic!("Invalid command"),
        };
        let start: Vec<usize> = parts[start_idx]
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        let end: Vec<usize> = parts[start_idx + 2]
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        Instruction {
            command,
            start: (start[0], start[1]),
            end: (end[0], end[1]),
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::parse).collect()
}

fn solve_p1(input: &str) -> usize {
    let instructions = parse_input(input);
    let mut grid = vec![vec![false; 1000]; 1000];
    for instr in instructions {
        for row in &mut grid[instr.start.0..=instr.end.0] {
            for cell in &mut row[instr.start.1..=instr.end.1] {
                match instr.command {
                    Command::TurnOn => *cell = true,
                    Command::TurnOff => *cell = false,
                    Command::Toggle => *cell = !*cell,
                }
            }
        }
    }
    grid.iter().flatten().filter(|&&x| x).count()
}

fn solve_p2(input: &str) -> usize {
    let instructions = parse_input(input);
    let mut grid: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    for instr in instructions {
        for row in &mut grid[instr.start.0..=instr.end.0] {
            for cell in &mut row[instr.start.1..=instr.end.1] {
                match instr.command {
                    Command::TurnOn => *cell += 1,
                    Command::TurnOff => *cell = cell.saturating_sub(1),
                    Command::Toggle => *cell += 2,
                }
            }
        }
    }
    grid.iter().flatten().sum()
}

fn main() {
    run(DAY, solve_p1, solve_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve_p1("turn on 0,0 through 999,999"), 1_000_000);
        assert_eq!(solve_p1("toggle 0,0 through 999,0"), 1000);
        assert_eq!(solve_p1("turn off 499,499 through 500,500"), 0);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_p2("turn on 0,0 through 0,0"), 1);
        assert_eq!(solve_p2("toggle 0,0 through 999,999"), 2_000_000);
    }
}
