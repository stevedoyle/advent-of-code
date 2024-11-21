use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
struct Move {
    count: i32,
    from: i32,
    to: i32,
}

fn solve_p1(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);

    for m in moves {
        let from = m.from as usize - 1;
        let to = m.to as usize - 1;
        for _ in 0..m.count {
            let c = stacks[from].pop_back().unwrap();
            stacks[to].push_back(c);
        }
    }

    stacks.iter().map(|s| s.back().unwrap()).collect::<String>()
}

fn solve_p2(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);

    for m in moves {
        let from = m.from as usize - 1;
        let to = m.to as usize - 1;
        let mut temp = Vec::new();
        for _ in 0..m.count {
            let c = stacks[from].pop_back().unwrap();
            temp.push(c);
        }
        stacks[to].extend(temp.into_iter().rev());
    }

    stacks.iter().map(|s| s.back().unwrap()).collect::<String>()
}

fn parse_input(input: &str) -> (Vec<VecDeque<char>>, Vec<Move>) {
    let mut lines = input.lines();
    let mut stacks = vec![];
    let mut moves = vec![];

    for line in lines.by_ref() {
        if !line.contains("[") {
            break;
        }
        for (i, c) in line.chars().enumerate() {
            if c.is_alphabetic() {
                let stack_num = i / 4;
                while stacks.len() <= stack_num {
                    stacks.push(VecDeque::new());
                }
                let stack = stacks.get_mut(stack_num).unwrap();
                stack.push_front(c);
            }
        }
    }

    for line in lines {
        if line.starts_with("move") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let count = parts[1].parse().unwrap();
            let from = parts[3].parse().unwrap();
            let to = parts[5].parse().unwrap();
            moves.push(Move { count, from, to });
        }
    }
    (stacks, moves)
}

fn main() {
    let input = include_str!("../input.txt");
    let answer = solve_p1(input);
    println!("Part 1: {answer}");
    let answer = solve_p2(input);
    println!("Part 2: {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, "CMZ");
        let answer = solve_p2(INPUT);
        assert_eq!(answer, "MCD");
    }
}
