use aoc2025::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn from_char(c: char) -> Result<Self, char> {
        match c {
            '+' => Ok(Operator::Add),
            '*' => Ok(Operator::Multiply),
            _ => Err(c),
        }
    }

    fn apply(&self, numbers: &[usize]) -> usize {
        match self {
            Operator::Add => numbers.iter().sum(),
            Operator::Multiply => numbers.iter().product(),
        }
    }
}

fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<Operator>) {
    let lines: Vec<&str> = input.lines().collect();

    // Last line contains operators
    let operators = lines
        .last()
        .expect("Input should have at least one line")
        .split_whitespace()
        .map(|s| {
            let c = s.chars().next().expect("Operator should not be empty");
            Operator::from_char(c).expect("Invalid operator character")
        })
        .collect();

    // All lines except last contain numbers
    let numbers = lines[..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<usize>().expect("Failed to parse number"))
                .collect()
        })
        .collect();

    (numbers, operators)
}

fn extract_number_from_column(grid: &Grid<char>, col: usize) -> Option<usize> {
    let mut digit_chars = Vec::new();
    let (nrows, _) = grid::dimensions(grid);
    // Skip the last row which contains operators
    for row in grid.iter().take(nrows - 1) {
        let c = row[col];
        if c != ' ' {
            digit_chars.push(c);
        }
    }
    if digit_chars.is_empty() {
        return None;
    }
    let digit_str: String = digit_chars.into_iter().collect();
    let number: usize = digit_str.parse().expect("Failed to parse column number");
    Some(number)
}

fn solve_p1(input: &str) -> usize {
    let (numbers, operators) = parse_input(input);
    operators
        .iter()
        .enumerate()
        .map(|(col, operator)| {
            let column_values: Vec<usize> = numbers.iter().map(|row| row[col]).collect();
            operator.apply(&column_values)
        })
        .sum()
}

fn solve_p2(input: &str) -> usize {
    let grid = parse_grid(input);

    // Part 2 reads numbers VERTICALLY from right to left:
    // - Each column contains digits of a number (top to bottom = most to least significant)
    // - Empty columns separate different number groups
    // - Bottom row contains operators (one per group)
    // - Process: scan right-to-left, collect numbers in each group, apply operator
    //
    // Example:
    //   1 2 3     3 2 8     5 1     6 4
    //     4 5     6 4     3 8 7     2 3
    //       6     9 8     2 1 5     3 1 4
    //   *         +         *         +
    //
    // Groups (right-to-left):
    //   Group 1: 64, 23, 314 → sum
    //   Group 2: 51, 387, 215 → product
    //   Group 3: 328, 64, 98 → sum
    //   Group 4: 123, 45, 6 → product

    let mut numbers: Vec<usize> = Vec::new();
    let mut results = 0;

    let (nrows, ncols) = grid::dimensions(&grid);

    // Extract operators from bottom row (filtering out spaces)
    let operators: Vec<Operator> = grid[nrows - 1]
        .iter()
        .copied()
        .filter(|&c| c == '+' || c == '*')
        .map(|c| Operator::from_char(c).expect("Invalid operator in grid"))
        .collect();
    let mut operator_iter = operators.iter().rev();

    // Process columns from right to left
    for col in (0..ncols).rev() {
        let current_number = extract_number_from_column(&grid, col);
        if let Some(num) = current_number {
            numbers.push(num);
            continue;
        }
        // Empty column = end of group, apply operator
        results += operator_iter
            .next()
            .expect("Missing operator for group")
            .apply(&numbers);
        numbers.clear();
    }
    // Process final group
    results += operator_iter
        .next()
        .expect("Missing operator for final group")
        .apply(&numbers);
    results
}

fn main() {
    let input = read_input(6);

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
        let input = read_test_input(6);
        let answer = solve_p1(&input);
        assert_eq!(answer, 4277556);
        let answer = solve_p2(&input);
        assert_eq!(answer, 3263827);
    }
}
