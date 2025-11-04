use pathfinding::{matrix::Matrix, prelude::dijkstra};

fn parse_input(input: &str) -> Matrix<usize> {
    Matrix::from_rows(input.lines().map(|line| {
        line.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>()
    }))
    .unwrap()
}

fn successors(pos: (usize, usize), grid: &Matrix<usize>) -> Vec<((usize, usize), usize)> {
    grid.neighbours(pos, false).map(|p| (p, grid[p])).collect()
}

fn solve_p1(input: &str) -> i32 {
    let grid = parse_input(input);
    let start = (0, 0);
    let end = (grid.rows - 1, grid.columns - 1);
    let (_path, cost) = dijkstra(&start, |p| successors(*p, &grid), |p| *p == end).unwrap();
    cost as i32
}

fn expand_grid(grid: &Matrix<usize>) -> Matrix<usize> {
    let rows = grid.rows;
    let cols = grid.columns;
    let mut expanded = vec![vec![0; cols * 5]; rows * 5];

    for tile_row in 0..5 {
        for tile_col in 0..5 {
            for row in 0..rows {
                for col in 0..cols {
                    let original_value = grid[(row, col)];
                    let increment = tile_row + tile_col;
                    let new_value = ((original_value - 1 + increment) % 9) + 1;
                    expanded[tile_row * rows + row][tile_col * cols + col] = new_value;
                }
            }
        }
    }

    Matrix::from_rows(expanded).unwrap()
}

fn solve_p2(input: &str) -> i32 {
    let grid = parse_input(input);
    let grid = expand_grid(&grid);
    let start = (0, 0);
    let end = (grid.rows - 1, grid.columns - 1);
    let (_path, cost) = dijkstra(&start, |p| successors(*p, &grid), |p| *p == end).unwrap();
    cost as i32
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
        assert_eq!(answer, 40);
        let answer = solve_p2(&input);
        assert_eq!(answer, 315);
    }
}
