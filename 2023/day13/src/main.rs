use grid::Grid;
use std::{cmp::min, error::Error, fs};

fn parse_input(input: &str) -> Vec<Grid<char>> {
    let mut grids: Vec<Grid<char>> = Vec::new();
    let mut grid = Grid::new(0, 0);

    for line in input.lines() {
        if line.trim().is_empty() {
            if grid.rows() > 0 {
                grids.push(grid);
                grid = Grid::new(0, 0);
            }
            continue;
        }
        grid.push_row(line.trim().chars().collect());
    }
    if grid.rows() > 0 {
        grids.push(grid);
    }
    grids
}

fn find_vertical_line(grid: &Grid<char>) -> Option<usize> {
    // Approach: Compare spans of columns at a time. Iterating over the possible span combinations.
    let ncols = grid.cols();
    for col_idx in 1..ncols {
        let span = min(col_idx, ncols - col_idx);
        let top: Vec<&char> = ((col_idx - span)..col_idx)
            .rev()
            .map(|i| grid.iter_col(i))
            .flat_map(|it| it.clone())
            .collect();
        let bottom: Vec<&char> = (col_idx..(col_idx + span))
            .map(|i| grid.iter_col(i))
            .flat_map(|it| it.clone())
            .collect();
        if top.iter().eq(bottom.iter()) {
            return Some(col_idx);
        }
    }
    None
}

fn find_horizontal_line(grid: &Grid<char>) -> Option<usize> {
    let nrows = grid.rows();
    for row_idx in 1..grid.rows() {
        let span = min(row_idx, nrows - row_idx);
        let top: Vec<&char> = ((row_idx - span)..row_idx)
            .rev()
            .map(|i| grid.iter_row(i))
            .flat_map(|it| it.clone())
            .collect();
        let bottom: Vec<&char> = (row_idx..(row_idx + span))
            .map(|i| grid.iter_row(i))
            .flat_map(|it| it.clone())
            .collect();
        if top.iter().eq(bottom.iter()) {
            return Some(row_idx);
        }
    }
    None
}

fn find_vertical_line_with_smudge(grid: &Grid<char>) -> Option<usize> {
    // A smudge is at the spot where there is only one difference in the reflection
    let ncols = grid.cols();
    for col_idx in 1..ncols {
        let span = min(col_idx, ncols - col_idx);
        let top: Vec<&char> = ((col_idx - span)..col_idx)
            .rev()
            .map(|i| grid.iter_col(i))
            .flat_map(|it| it.clone())
            .collect();
        let bottom: Vec<&char> = (col_idx..(col_idx + span))
            .map(|i| grid.iter_col(i))
            .flat_map(|it| it.clone())
            .collect();
        let diff_count = top
            .iter()
            .zip(bottom.iter())
            .filter(|(&a, &b)| a != b)
            .count();
        if diff_count == 1 {
            return Some(col_idx);
        }
    }
    None
}

fn find_horizontal_line_with_smudge(grid: &Grid<char>) -> Option<usize> {
    let nrows = grid.rows();
    for row_idx in 1..grid.rows() {
        let span = min(row_idx, nrows - row_idx);
        let top: Vec<&char> = ((row_idx - span)..row_idx)
            .rev()
            .map(|i| grid.iter_row(i))
            .flat_map(|it| it.clone())
            .collect();
        let bottom: Vec<&char> = (row_idx..(row_idx + span))
            .map(|i| grid.iter_row(i))
            .flat_map(|it| it.clone())
            .collect();
        let diff_count = top
            .iter()
            .zip(bottom.iter())
            .filter(|(&a, &b)| a != b)
            .count();
        if diff_count == 1 {
            return Some(row_idx);
        }
    }
    None
}

fn solve_p1(input: &str) -> usize {
    let grids = parse_input(input);
    let vsum: usize = grids.iter().filter_map(find_vertical_line).sum();
    let hsum: usize = grids.iter().filter_map(find_horizontal_line).sum();
    vsum + (100 * hsum)
}

fn solve_p2(input: &str) -> usize {
    let grids = parse_input(input);
    let vsum: usize = grids
        .iter()
        .filter_map(find_vertical_line_with_smudge)
        .sum();
    let hsum: usize = grids
        .iter()
        .filter_map(find_horizontal_line_with_smudge)
        .sum();
    vsum + (100 * hsum)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let answer = solve_p1(&input);
    println!("Part 1: {answer}");

    let answer = solve_p2(&input);
    println!("Part 2: {answer}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        String::from(
            "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#",
        )
    }

    #[test]
    fn test_parse() {
        let grids = parse_input(&get_test_input());
        assert_eq!(grids.len(), 2);
        assert_eq!(grids[0].rows(), 7);
        assert_eq!(grids[0].cols(), 9);
        assert_eq!(grids[1].rows(), 7);
        assert_eq!(grids[1].cols(), 9);
    }

    #[test]
    fn test_find_lines() {
        let grids = parse_input(&get_test_input());
        let vline = find_vertical_line(&grids[0]);
        assert_eq!(vline, Some(5));
        let vline = find_vertical_line(&grids[1]);
        assert_eq!(vline, None);

        let vline = find_horizontal_line(&grids[0]);
        assert_eq!(vline, None);
        let vline = find_horizontal_line(&grids[1]);
        assert_eq!(vline, Some(4));
    }

    #[test]
    fn test_find_lines_with_smudge() {
        let grids = parse_input(&get_test_input());
        let vline = find_vertical_line_with_smudge(&grids[0]);
        assert_eq!(vline, None);
        let vline = find_vertical_line_with_smudge(&grids[1]);
        assert_eq!(vline, None);

        let vline = find_horizontal_line_with_smudge(&grids[0]);
        assert_eq!(vline, Some(3));
        let vline = find_horizontal_line_with_smudge(&grids[1]);
        assert_eq!(vline, Some(1));
    }

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(&get_test_input());
        assert_eq!(answer, 405);
        let answer = solve_p2(&get_test_input());
        assert_eq!(answer, 400);
    }

    #[test]
    fn test_solve() -> Result<(), Box<dyn Error>> {
        let input = fs::read_to_string("input.txt")?;
        let answer = solve_p1(&input);
        assert_eq!(answer, 35360);
        let answer = solve_p2(&input);
        assert_eq!(answer, 36755);

        Ok(())
    }
}
