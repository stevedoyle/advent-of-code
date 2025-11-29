use aoc2020::*;

fn parse_input(input: &str) -> Grid<char> {
    parse_grid(input)
}

fn traverse_slope(slope: &Grid<char>, right: usize, down: usize) -> usize {
    let (rows, cols) = grid::dimensions(slope);
    let mut pos = (0, 0);
    let mut tree_count = 0;
    while pos.0 < rows {
        if slope[pos.0][pos.1 % cols] == '#' {
            tree_count += 1;
        }
        pos.0 += down;
        pos.1 += right;
    }
    tree_count
}

fn solve_p1(input: &str) -> usize {
    let slope = parse_input(input);
    traverse_slope(&slope, 3, 1)
}

fn solve_p2(input: &str) -> usize {
    let slope = parse_input(input);
    let mut product = 1;
    for (right, down) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let count = traverse_slope(&slope, *right, *down);
        product *= count;
    }
    product
}

fn main() {
    let input = read_input(3);

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
        let input = read_test_input(3);
        let answer = solve_p1(&input);
        assert_eq!(answer, 7);
        let answer = solve_p2(&input);
        assert_eq!(answer, 336);
    }
}
