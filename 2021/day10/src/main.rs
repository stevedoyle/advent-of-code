fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

fn syntax_error_score(line: &str) -> usize {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return 3;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return 57;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return 1197;
                }
            }
            '>' => {
                if stack.pop() != Some('<') {
                    return 25137;
                }
            }
            _ => unreachable!(),
        }
    }
    0
}
fn solve_p1(input: &str) -> usize {
    let nav = parse_input(input);
    nav.iter().map(|line| syntax_error_score(line)).sum()
}

fn solve_p2(input: &str) -> usize {
    let nav = parse_input(input);
    let mut scores = Vec::new();
    let valid_lines = nav.iter().filter(|line| syntax_error_score(line) == 0);
    for line in valid_lines {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => {
                    stack.pop();
                }
                ']' => {
                    stack.pop();
                }
                '}' => {
                    stack.pop();
                }
                '>' => {
                    stack.pop();
                }
                _ => {}
            }
        }
        let mut score: i64 = 0;
        while let Some(c) = stack.pop() {
            score *= 5;
            score += match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0,
            }
        }
        scores.push(score);
    }
    scores.sort();
    scores[scores.len() / 2] as usize
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
    fn test_solve_with_test_input() {
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let answer = solve_p1(&input);
        assert_eq!(answer, 26397);
        let answer = solve_p2(&input);
        assert_eq!(answer, 288957);
    }
}
