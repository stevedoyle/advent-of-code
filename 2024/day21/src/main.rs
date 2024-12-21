use std::collections::HashMap;

// 7  8  9     0  1  2
// 4  5  6     3  4  5
// 1  2  3     6  7  8
//    0  A     x  10 11
const NUMERIC_KEYS: &str = "789456123_0A";

//   ^ A      x 1 2
// < v >      3 4 5
const DIRECTIONAL_KEYS: &str = "_^A<v>";

#[derive(Debug)]
struct Keypad {
    keys: String,
    hole: (usize, usize), // row, col
}

impl Keypad {
    fn new(keys: &str, hole: (usize, usize)) -> Self {
        Keypad {
            keys: keys.to_string(),
            hole,
        }
    }

    fn press(&self, current: char, target: char) -> Vec<char> {
        let current_pos = self.keys.find(current).unwrap();
        let (current_row, current_col) = (current_pos / 3, current_pos % 3);
        let target_pos = self.keys.find(target).unwrap();
        let (target_row, target_col) = (target_pos / 3, target_pos % 3);
        let row_diff = target_row as isize - current_row as isize;
        let col_diff = target_col as isize - current_col as isize;

        let col_move = if col_diff > 0 { '>' } else { '<' };
        let mut col_move: Vec<char> = std::iter::repeat(col_move)
            .take(col_diff.unsigned_abs())
            .collect();

        let row_move = if row_diff > 0 { 'v' } else { '^' };
        let mut row_move: Vec<char> = std::iter::repeat(row_move)
            .take(row_diff.unsigned_abs())
            .collect();

        if target_row == self.hole.0 && current_col == self.hole.1 {
            col_move.extend(row_move);
            col_move
        } else if current_row == self.hole.0 && target_col == self.hole.1 {
            row_move.extend(col_move);
            row_move
        } else if col_move.iter().any(|&c| c == '<') {
            col_move.extend(row_move);
            col_move
        } else {
            row_move.extend(col_move);
            row_move
        }
    }
}

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

fn numeric_part(code: &str) -> usize {
    code.chars()
        .filter(|&c| c != 'A')
        .collect::<String>()
        .parse()
        .unwrap()
}

fn num_presses(
    keypads: &[Keypad],
    levels: usize,
    current: char,
    target: char,
    level: usize,
    cache: &mut HashMap<(char, char, usize), usize>,
) -> usize {
    let mut sequence = keypads[level].press(current, target);
    sequence.push('A');

    if let Some(&length) = cache.get(&(current, target, level)) {
        return length;
    }

    if level == levels {
        cache.insert((current, target, level), sequence.len());
        sequence.len()
    } else {
        let mut length = 0;
        let mut c = 'A';
        for &t in sequence.iter() {
            length += num_presses(keypads, levels, c, t, level + 1, cache);
            c = t;
        }
        cache.insert((current, target, level), length);
        length
    }
}

fn press_keypads_recursive(code: &str, keypads: &[Keypad]) -> usize {
    let mut cache = HashMap::new();
    let levels = keypads.len() - 1;
    let mut length = 0;
    let mut current = 'A';
    for c in code.chars() {
        length += num_presses(keypads, levels, current, c, 0, &mut cache);
        current = c;
    }

    length
}

fn solve(codes: &[String], chain_len: usize) -> usize {
    let mut keypads = vec![Keypad::new(NUMERIC_KEYS, (3, 0))];
    let dirkps = (0..chain_len)
        .map(|_| Keypad::new(DIRECTIONAL_KEYS, (0, 0)))
        .collect::<Vec<_>>();
    keypads.extend(dirkps);
    let mut complexity = 0;
    for code in codes {
        let sequence_length = press_keypads_recursive(code, &keypads);
        complexity += sequence_length * numeric_part(code);
    }

    complexity
}

fn solve_p1(input: &str) -> usize {
    let codes = parse_input(input);
    solve(&codes, 2)
}

fn solve_p2(input: &str) -> usize {
    let codes = parse_input(input);
    solve(&codes, 25)
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
        assert_eq!(answer, 126384);
    }
}
