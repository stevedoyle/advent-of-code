use std::fs;

/// Read input file for a given day
pub fn read_input(day: u8) -> String {
    let path = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", path))
}

/// Read test input file for a given day
pub fn read_test_input(day: u8) -> String {
    let path = format!("inputs/day{:02}_test.txt", day);
    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read test input file: {}", path))
}

/// Parse lines into a vector of type T
pub fn parse_lines<T>(input: &str) -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(|line| line.parse().expect("Failed to parse line"))
        .collect()
}

/// Parse a grid of characters
pub fn parse_grid(input: &str) -> Grid<char> {
    input.lines().map(|line| line.chars().collect()).collect()
}

/// Parse a grid of digits
pub fn parse_digit_grid(input: &str) -> Grid<u32> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Invalid digit"))
                .collect()
        })
        .collect()
}

/// Grid type alias
pub type Grid<T> = Vec<Vec<T>>;

/// Grid helper functions
pub mod grid {
    /// Get grid dimensions (rows, cols)
    pub fn dimensions<T>(grid: &[Vec<T>]) -> (usize, usize) {
        let rows = grid.len();
        let cols = if rows > 0 { grid[0].len() } else { 0 };
        (rows, cols)
    }

    /// Check if a position is within grid bounds
    pub fn in_bounds<T>(grid: &[Vec<T>], row: isize, col: isize) -> bool {
        row >= 0 && col >= 0 && (row as usize) < grid.len() && (col as usize) < grid[0].len()
    }

    /// Get value at position if in bounds
    pub fn get<T>(grid: &[Vec<T>], row: isize, col: isize) -> Option<&T> {
        if in_bounds(grid, row, col) {
            Some(&grid[row as usize][col as usize])
        } else {
            None
        }
    }

    /// Get 4-directional neighbors (up, down, left, right)
    pub fn neighbors4(row: isize, col: isize) -> [(isize, isize); 4] {
        [
            (row - 1, col),     // up
            (row + 1, col),     // down
            (row, col - 1),     // left
            (row, col + 1),     // right
        ]
    }

    /// Get 8-directional neighbors (including diagonals)
    pub fn neighbors8(row: isize, col: isize) -> [(isize, isize); 8] {
        [
            (row - 1, col - 1), // up-left
            (row - 1, col),     // up
            (row - 1, col + 1), // up-right
            (row, col - 1),     // left
            (row, col + 1),     // right
            (row + 1, col - 1), // down-left
            (row + 1, col),     // down
            (row + 1, col + 1), // down-right
        ]
    }

    /// Get valid 4-directional neighbors within grid bounds
    pub fn valid_neighbors4<T>(grid: &[Vec<T>], row: isize, col: isize) -> Vec<(usize, usize)> {
        neighbors4(row, col)
            .iter()
            .filter(|(r, c)| in_bounds(grid, *r, *c))
            .map(|(r, c)| (*r as usize, *c as usize))
            .collect()
    }

    /// Get valid 8-directional neighbors within grid bounds
    pub fn valid_neighbors8<T>(grid: &[Vec<T>], row: isize, col: isize) -> Vec<(usize, usize)> {
        neighbors8(row, col)
            .iter()
            .filter(|(r, c)| in_bounds(grid, *r, *c))
            .map(|(r, c)| (*r as usize, *c as usize))
            .collect()
    }

    /// Iterate over all positions in the grid
    pub fn positions<T>(grid: &[Vec<T>]) -> impl Iterator<Item = (usize, usize)> {
        let (rows, cols) = dimensions(grid);
        (0..rows).flat_map(move |r| (0..cols).map(move |c| (r, c)))
    }

    /// Find all positions where predicate is true
    pub fn find_all<T, F>(grid: &[Vec<T>], predicate: F) -> Vec<(usize, usize)>
    where
        F: Fn(&T) -> bool,
    {
        let (rows, cols) = dimensions(grid);
        (0..rows)
            .flat_map(|r| (0..cols).map(move |c| (r, c)))
            .filter(|(r, c)| predicate(&grid[*r][*c]))
            .collect()
    }

    /// Find first position where predicate is true
    pub fn find<T, F>(grid: &[Vec<T>], predicate: F) -> Option<(usize, usize)>
    where
        F: Fn(&T) -> bool,
    {
        for (r, row) in grid.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if predicate(cell) {
                    return Some((r, c));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let input = "1\n2\n3";
        let result: Vec<i32> = parse_lines(input);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_parse_grid() {
        let input = "abc\ndef";
        let result = parse_grid(input);
        assert_eq!(result, vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]);
    }

    #[test]
    fn test_grid_dimensions() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(grid::dimensions(&grid), (2, 3));
    }

    #[test]
    fn test_grid_in_bounds() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert!(grid::in_bounds(&grid, 0, 0));
        assert!(grid::in_bounds(&grid, 1, 2));
        assert!(!grid::in_bounds(&grid, 2, 0));
        assert!(!grid::in_bounds(&grid, 0, 3));
        assert!(!grid::in_bounds(&grid, -1, 0));
    }

    #[test]
    fn test_grid_get() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(grid::get(&grid, 0, 0), Some(&1));
        assert_eq!(grid::get(&grid, 1, 2), Some(&6));
        assert_eq!(grid::get(&grid, 2, 0), None);
        assert_eq!(grid::get(&grid, -1, 0), None);
    }

    #[test]
    fn test_grid_neighbors4() {
        let neighbors = grid::neighbors4(1, 1);
        assert_eq!(neighbors, [(0, 1), (2, 1), (1, 0), (1, 2)]);
    }

    #[test]
    fn test_grid_valid_neighbors4() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let neighbors = grid::valid_neighbors4(&grid, 0, 0);
        assert_eq!(neighbors, vec![(1, 0), (0, 1)]);

        let neighbors = grid::valid_neighbors4(&grid, 1, 1);
        assert_eq!(neighbors, vec![(0, 1), (1, 0), (1, 2)]);
    }

    #[test]
    fn test_grid_find() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(grid::find(&grid, |&x| x == 5), Some((1, 1)));
        assert_eq!(grid::find(&grid, |&x| x == 7), None);
    }

    #[test]
    fn test_grid_find_all() {
        let grid = vec![vec![1, 2, 3], vec![2, 4, 2]];
        let positions = grid::find_all(&grid, |&x| x == 2);
        assert_eq!(positions, vec![(0, 1), (1, 0), (1, 2)]);
    }
}
