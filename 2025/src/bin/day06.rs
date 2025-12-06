use aoc2025::*;

fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<char>) {
    let mut operators = Vec::new();
    let mut numbers = Vec::new();

    input.lines().for_each(|line| {
        if ['+', '*'].contains(&line.trim().chars().next().unwrap()) {
            operators = line
                .split_whitespace()
                .map(|s| s.chars().next().unwrap())
                .collect();
        } else {
            numbers.push(
                line.split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect(),
            );
        }
    });
    (numbers, operators)
}

fn parse_input_as_grid(input: &str) -> Grid<char> {
    parse_grid(input)
}

fn extract_number_from_column(grid: &Grid<char>, col: usize) -> Option<usize> {
    let mut digit_chars = Vec::new();
    let (nrows, _) = grid::dimensions(grid);
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
    let number: usize = digit_str.parse().unwrap();
    Some(number)
}

fn process_group(numbers: &[usize], operator: &char) -> usize {
    match operator {
        '+' => numbers.iter().sum(),
        '*' => numbers.iter().product(),
        _ => panic!("Unknown operator"),
    }
}

fn solve_p1(input: &str) -> usize {
    let (numbers, operators) = parse_input(input);
    let mut results = Vec::new();
    for (col, operator) in operators.iter().enumerate() {
        match operator {
            '+' => results.push(numbers.iter().map(|row| row[col]).sum()),
            '*' => results.push(numbers.iter().map(|row| row[col]).product()),
            _ => panic!("Unknown operator"),
        }
    }
    results.iter().sum()
}

fn solve_p2(input: &str) -> usize {
    let grid = parse_input_as_grid(input);
    // Iterate over the grid columns starting at the rightmost column.
    // For each column, collect the digits into a number. The lower row numbers contain the most
    // significant digits.
    // The final row is the operator.
    // A blank row represents a space between numbers.
    let mut numbers: Vec<usize> = Vec::new();
    let mut results = 0;

    let (nrows, ncols) = grid::dimensions(&grid);

    let operators: Vec<char> = grid[nrows - 1]
        .iter()
        .cloned()
        .filter(|&c| c == '+' || c == '*')
        .collect();
    let mut operator_iter = operators.iter().rev();

    for col in (0..ncols).rev() {
        let current_number = extract_number_from_column(&grid, col);
        if let Some(num) = current_number {
            numbers.push(num);
            continue;
        }
        results += process_group(&numbers, operator_iter.next().unwrap());
        numbers.clear();
    }
    results += process_group(&numbers, operator_iter.next().unwrap());
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
