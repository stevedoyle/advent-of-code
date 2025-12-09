use std::fs;

/// Read input file for a given day
pub fn read_input(day: u8) -> String {
    let path = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read input file: {}", path))
}

/// Read test input file for a given day
pub fn read_test_input(day: u8) -> String {
    let path = format!("inputs/day{:02}_test.txt", day);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read test input file: {}", path))
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

/// Point in 2D space
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    /// Create a new point
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// Parse a point from a string like "x,y" or "x y"
    pub fn parse(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = if s.contains(',') {
            s.split(',').collect()
        } else {
            s.split_whitespace().collect()
        };

        if parts.len() != 2 {
            return Err(format!("Expected 2 coordinates, found {}", parts.len()));
        }

        let x = parts[0]
            .trim()
            .parse()
            .map_err(|_| format!("Invalid x coordinate: {}", parts[0]))?;
        let y = parts[1]
            .trim()
            .parse()
            .map_err(|_| format!("Invalid y coordinate: {}", parts[1]))?;

        Ok(Self::new(x, y))
    }

    /// Manhattan distance from origin
    pub fn manhattan_distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }

    /// Manhattan distance to another point
    pub fn manhattan_distance_to(&self, other: &Point) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Add two points
    pub fn add(&self, other: &Point) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    /// Subtract two points
    pub fn sub(&self, other: &Point) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }

    /// Scale a point by a scalar
    pub fn scale(&self, scalar: isize) -> Self {
        Self::new(self.x * scalar, self.y * scalar)
    }

    /// Get 4-directional neighbors (up, down, left, right)
    pub fn neighbors4(&self) -> [Point; 4] {
        [
            Point::new(self.x, self.y - 1), // up
            Point::new(self.x, self.y + 1), // down
            Point::new(self.x - 1, self.y), // left
            Point::new(self.x + 1, self.y), // right
        ]
    }

    /// Get 8-directional neighbors (including diagonals)
    pub fn neighbors8(&self) -> [Point; 8] {
        [
            Point::new(self.x - 1, self.y - 1), // up-left
            Point::new(self.x, self.y - 1),     // up
            Point::new(self.x + 1, self.y - 1), // up-right
            Point::new(self.x - 1, self.y),     // left
            Point::new(self.x + 1, self.y),     // right
            Point::new(self.x - 1, self.y + 1), // down-left
            Point::new(self.x, self.y + 1),     // down
            Point::new(self.x + 1, self.y + 1), // down-right
        ]
    }
}

impl std::str::FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

/// Parse points from input lines
pub fn parse_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter_map(|line| Point::parse(line).ok())
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
            (row - 1, col), // up
            (row + 1, col), // down
            (row, col - 1), // left
            (row, col + 1), // right
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

    /// Run Dijkstra's algorithm on a grid with custom cost function
    /// Returns a map of positions to their minimum cost from the start
    pub fn dijkstra<T, F>(
        grid: &[Vec<T>],
        start: (usize, usize),
        cost_fn: F,
    ) -> std::collections::HashMap<(usize, usize), usize>
    where
        F: Fn(&T, &T) -> Option<usize>,
    {
        use std::cmp::Reverse;
        use std::collections::{BinaryHeap, HashMap};

        let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
        let mut heap = BinaryHeap::new();

        dist.insert(start, 0);
        heap.push((Reverse(0), start));

        while let Some((Reverse(cost), pos)) = heap.pop() {
            let (row, col) = pos;

            if dist.get(&pos).is_some_and(|&d| cost > d) {
                continue;
            }

            for (next_row, next_col) in valid_neighbors4(grid, row as isize, col as isize) {
                let current_cell = &grid[row][col];
                let next_cell = &grid[next_row][next_col];

                if let Some(edge_cost) = cost_fn(current_cell, next_cell) {
                    let next_cost = cost + edge_cost;
                    let next_pos = (next_row, next_col);

                    if next_cost < *dist.get(&next_pos).unwrap_or(&usize::MAX) {
                        dist.insert(next_pos, next_cost);
                        heap.push((Reverse(next_cost), next_pos));
                    }
                }
            }
        }

        dist
    }

    /// Run Dijkstra's algorithm and find shortest path to target
    /// Returns (cost, path) if a path exists
    pub fn dijkstra_path<T, F>(
        grid: &[Vec<T>],
        start: (usize, usize),
        target: (usize, usize),
        cost_fn: F,
    ) -> Option<(usize, Vec<(usize, usize)>)>
    where
        F: Fn(&T, &T) -> Option<usize>,
    {
        use std::cmp::Reverse;
        use std::collections::{BinaryHeap, HashMap};

        let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
        let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut heap = BinaryHeap::new();

        dist.insert(start, 0);
        heap.push((Reverse(0), start));

        while let Some((Reverse(cost), pos)) = heap.pop() {
            if pos == target {
                // Reconstruct path
                let mut path = vec![target];
                let mut current = target;
                while current != start {
                    current = *prev.get(&current)?;
                    path.push(current);
                }
                path.reverse();
                return Some((cost, path));
            }

            if dist.get(&pos).is_some_and(|&d| cost > d) {
                continue;
            }

            let (row, col) = pos;

            for (next_row, next_col) in valid_neighbors4(grid, row as isize, col as isize) {
                let current_cell = &grid[row][col];
                let next_cell = &grid[next_row][next_col];

                if let Some(edge_cost) = cost_fn(current_cell, next_cell) {
                    let next_cost = cost + edge_cost;
                    let next_pos = (next_row, next_col);

                    if next_cost < *dist.get(&next_pos).unwrap_or(&usize::MAX) {
                        dist.insert(next_pos, next_cost);
                        prev.insert(next_pos, pos);
                        heap.push((Reverse(next_cost), next_pos));
                    }
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
    fn test_point_new() {
        let p = Point::new(3, 4);
        assert_eq!(p.x, 3);
        assert_eq!(p.y, 4);
    }

    #[test]
    fn test_point_parse() {
        assert_eq!(Point::parse("3,4").unwrap(), Point::new(3, 4));
        assert_eq!(Point::parse("3 4").unwrap(), Point::new(3, 4));
        assert_eq!(Point::parse("-5,10").unwrap(), Point::new(-5, 10));
        assert!(Point::parse("invalid").is_err());
        assert!(Point::parse("1,2,3").is_err());
    }

    #[test]
    fn test_point_manhattan_distance() {
        let p1 = Point::new(3, 4);
        assert_eq!(p1.manhattan_distance(), 7);

        let p2 = Point::new(-3, -4);
        assert_eq!(p2.manhattan_distance(), 7);

        let p3 = Point::new(0, 0);
        let p4 = Point::new(3, 4);
        assert_eq!(p3.manhattan_distance_to(&p4), 7);
    }

    #[test]
    fn test_point_operations() {
        let p1 = Point::new(3, 4);
        let p2 = Point::new(1, 2);

        assert_eq!(p1.add(&p2), Point::new(4, 6));
        assert_eq!(p1.sub(&p2), Point::new(2, 2));
        assert_eq!(p1.scale(2), Point::new(6, 8));
    }

    #[test]
    fn test_point_neighbors() {
        let p = Point::new(5, 5);
        let n4 = p.neighbors4();
        assert_eq!(n4[0], Point::new(5, 4)); // up
        assert_eq!(n4[1], Point::new(5, 6)); // down
        assert_eq!(n4[2], Point::new(4, 5)); // left
        assert_eq!(n4[3], Point::new(6, 5)); // right

        let n8 = p.neighbors8();
        assert_eq!(n8.len(), 8);
        assert!(n8.contains(&Point::new(4, 4))); // up-left diagonal
        assert!(n8.contains(&Point::new(6, 6))); // down-right diagonal
    }

    #[test]
    fn test_parse_points() {
        let input = "1,2\n3,4\n5 6";
        let points = parse_points(input);
        assert_eq!(
            points,
            vec![Point::new(1, 2), Point::new(3, 4), Point::new(5, 6)]
        );
    }

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

    #[test]
    fn test_dijkstra() {
        // Simple 3x3 grid with uniform cost of 1
        let grid = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];

        let start = (0, 0);
        let distances = grid::dijkstra(&grid, start, |_, _| Some(1));

        // Check distances from top-left corner
        assert_eq!(distances.get(&(0, 0)), Some(&0));
        assert_eq!(distances.get(&(0, 1)), Some(&1));
        assert_eq!(distances.get(&(1, 0)), Some(&1));
        assert_eq!(distances.get(&(2, 2)), Some(&4));
    }

    #[test]
    fn test_dijkstra_with_walls() {
        // Grid where 0 is passable (cost 1) and 9 is a wall
        // Path from (0,0) to (2,2): (0,0) -> (0,1) -> (1,1) -> (2,1) -> (2,2)
        let grid = vec![vec![0, 0, 9], vec![9, 0, 9], vec![0, 0, 0]];

        let start = (0, 0);
        let distances = grid::dijkstra(&grid, start, |_, next| {
            if *next == 9 {
                None // Wall - no path
            } else {
                Some(1) // Passable - cost 1
            }
        });

        assert_eq!(distances.get(&(0, 0)), Some(&0));
        assert_eq!(distances.get(&(0, 1)), Some(&1));
        assert_eq!(distances.get(&(2, 2)), Some(&4)); // Correct distance is 4
        assert_eq!(distances.get(&(0, 2)), None); // Wall is unreachable
    }

    #[test]
    fn test_dijkstra_path() {
        let grid = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];

        let start = (0, 0);
        let target = (2, 2);
        let result = grid::dijkstra_path(&grid, start, target, |_, _| Some(1));

        assert!(result.is_some());
        let (cost, path) = result.unwrap();
        assert_eq!(cost, 4);
        assert_eq!(path.len(), 5);
        assert_eq!(path[0], start);
        assert_eq!(path[path.len() - 1], target);
    }

    #[test]
    fn test_dijkstra_path_no_path() {
        // Grid completely blocked
        let grid = vec![vec![0, 9, 0], vec![9, 9, 9], vec![0, 9, 0]];

        let start = (0, 0);
        let target = (2, 2);
        let result = grid::dijkstra_path(&grid, start, target, |_, next| {
            if *next == 9 {
                None
            } else {
                Some(1)
            }
        });

        assert!(result.is_none());
    }
}
