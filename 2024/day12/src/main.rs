use std::{collections::HashSet, hash::Hash};

use pathfinding::matrix::Matrix;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Region {
    name: char,
    area: usize,
    perimiter: usize,
    corners: usize,
}

impl Region {
    fn new(name: char) -> Self {
        Self {
            name,
            area: 0,
            perimiter: 0,
            corners: 0,
        }
    }

    fn price(&self) -> usize {
        self.area * self.perimiter
    }

    fn discount_price(&self) -> usize {
        self.corners * self.area
    }
}

fn parse_input(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|line| line.chars().collect::<Vec<_>>())).unwrap()
}

fn process_region(
    grid: &Matrix<char>,
    start: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) -> Region {
    let region_name = grid[start];
    let mut region = Region::new(region_name);
    let mut stack = vec![start];
    while let Some(pos) = stack.pop() {
        if visited.contains(&pos) {
            continue;
        }

        let mut nlen = 0;
        grid.neighbours(pos, false).for_each(|neighbour| {
            nlen += 1;
            if grid[neighbour] == region_name {
                stack.push(neighbour);
            } else {
                region.perimiter += 1;
            }
        });

        // Positions at the edge of the grid have less than 4 neighbours and the grid edges still
        // need to be counted as part of the perimiter.
        if nlen < 4 {
            region.perimiter += 4 - nlen;
        }

        region.corners += count_outside_corners(&grid, pos);
        region.corners += count_inside_corners(&grid, pos);
        region.area += 1;
        visited.insert(pos);
    }
    region
}

fn count_outside_corners(grid: &Matrix<char>, pos: (usize, usize)) -> usize {
    let (row, col) = (pos.0 as isize, pos.1 as isize);
    let region_name = grid[pos];
    let mut corners = 0;

    // Check top left corner
    let neighbours = [(row - 1, col), (row, col - 1)];
    if is_corner(grid, region_name, &neighbours) {
        corners += 1;
    }

    // Check top right corner
    let neighbours = [(row - 1, col), (row, col + 1)];
    if is_corner(grid, region_name, &neighbours) {
        corners += 1;
    }

    // Check bottom left corner
    let neighbours = [(row + 1, col), (row, col - 1)];
    if is_corner(grid, region_name, &neighbours) {
        corners += 1;
    }

    // Check bottom right corner
    let neighbours = [(row + 1, col), (row, col + 1)];
    if is_corner(grid, region_name, &neighbours) {
        corners += 1;
    }
    corners
}

fn is_corner(grid: &Matrix<char>, value: char, neighbours: &[(isize, isize)]) -> bool {
    neighbours.iter().all(|(row, col)| {
        if *row < 0 || *row as usize > grid.rows || *col < 0 || *col as usize > grid.columns {
            return true;
        }
        grid.get((*row as usize, *col as usize)) != Some(&value)
    })
}

fn count_inside_corners(grid: &Matrix<char>, pos: (usize, usize)) -> usize {
    let (row, col) = pos;
    let region_name = grid[pos];
    let mut corners = 0;

    let nrows = grid.rows;
    let ncols = grid.columns;

    // Check top left corner
    if row > 0
        && col > 0
        && grid[(row - 1, col)] == region_name
        && grid[(row, col - 1)] == region_name
        && grid[(row - 1, col - 1)] != region_name
    {
        corners += 1;
    }

    // Check top right corner
    if row > 0
        && col < ncols - 1
        && grid[(row - 1, col)] == region_name
        && grid[(row, col + 1)] == region_name
        && grid[(row - 1, col + 1)] != region_name
    {
        corners += 1;
    }

    // Check bottom left corner
    if row < nrows - 1
        && col > 0
        && grid[(row + 1, col)] == region_name
        && grid[(row, col - 1)] == region_name
        && grid[(row + 1, col - 1)] != region_name
    {
        corners += 1;
    }

    // Check bottom right corner
    if row < nrows - 1
        && col < ncols - 1
        && grid[(row + 1, col)] == region_name
        && grid[(row, col + 1)] == region_name
        && grid[(row + 1, col + 1)] != region_name
    {
        corners += 1;
    }
    corners
}

fn solve_p1(grid: &Matrix<char>) -> usize {
    let mut regions: Vec<Region> = Vec::new();
    let mut visited = HashSet::new();
    grid.items().for_each(|(pos, _)| {
        if !visited.contains(&pos) {
            regions.push(process_region(&grid, pos, &mut visited));
        }
    });
    regions.iter().map(|r| r.price()).sum()
}

fn solve_p2(grid: &Matrix<char>) -> usize {
    let mut regions: Vec<Region> = Vec::new();
    let mut visited = HashSet::new();
    grid.items().for_each(|(pos, _)| {
        if !visited.contains(&pos) {
            regions.push(process_region(&grid, pos, &mut visited));
        }
    });
    regions.iter().map(|r| r.discount_price()).sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let grid = parse_input(input);

    let start = std::time::Instant::now();
    let answer = solve_p1(&grid);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:.1?}");

    let start = std::time::Instant::now();
    let answer = solve_p2(&grid);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, elapsed: {elapsed:.1?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = "AAAA
BBCD
BBCC
EEEC";

    const TEST_INPUT_2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const TEST_INPUT_3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const TEST_INPUT_4: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    const TEST_INPUT_5: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn test_solve_with_test_input() {
        let grid1 = parse_input(TEST_INPUT_1);
        let grid2 = parse_input(TEST_INPUT_2);
        let grid3 = parse_input(TEST_INPUT_3);
        let grid4 = parse_input(TEST_INPUT_4);
        let grid5 = parse_input(TEST_INPUT_5);

        let answer = solve_p1(&grid1);
        assert_eq!(answer, 140);
        let answer = solve_p1(&grid2);
        assert_eq!(answer, 772);
        let answer = solve_p1(&grid3);
        assert_eq!(answer, 1930);
        let answer = solve_p2(&grid1);
        assert_eq!(answer, 80);
        let answer = solve_p2(&grid2);
        assert_eq!(answer, 436);
        let answer = solve_p2(&grid4);
        assert_eq!(answer, 236);
        let answer = solve_p2(&grid5);
        assert_eq!(answer, 368);
        let answer = solve_p2(&grid3);
        assert_eq!(answer, 1206);
    }

    #[test]
    fn test_is_corner() {
        let grid = parse_input(TEST_INPUT_1);
        let neighbours = [(-1, 0), (0, -1)];
        assert_eq!(is_corner(&grid, 'A', &neighbours), true);
        let neighbours = [(-1, 0), (0, 0)];
        assert_eq!(is_corner(&grid, 'A', &neighbours), false);
        let neighbours = [(-1, 3), (0, 4)];
        assert_eq!(is_corner(&grid, 'A', &neighbours), true);

        let neighbours = [(0, 0), (1, -1)];
        assert_eq!(is_corner(&grid, 'B', &neighbours), true);
        let neighbours = [(0, 1), (1, 2)];
        assert_eq!(is_corner(&grid, 'B', &neighbours), true);
        let neighbours = [(3, 0), (2, -1)];
        assert_eq!(is_corner(&grid, 'B', &neighbours), true);
        let neighbours = [(3, 1), (2, 2)];
        assert_eq!(is_corner(&grid, 'B', &neighbours), true);
        let neighbours = [(1, 1), (2, 2)];
        assert_eq!(is_corner(&grid, 'B', &neighbours), false);
    }

    #[test]
    fn test_count_corners() {
        let grid = parse_input(TEST_INPUT_1);
        assert_eq!(count_outside_corners(&grid, (0, 0)), 2);
        assert_eq!(count_outside_corners(&grid, (0, 1)), 0);
        assert_eq!(count_outside_corners(&grid, (0, 3)), 2);
        assert_eq!(count_outside_corners(&grid, (1, 0)), 1);
        assert_eq!(count_outside_corners(&grid, (2, 0)), 1);
        assert_eq!(count_outside_corners(&grid, (1, 1)), 1);
        assert_eq!(count_outside_corners(&grid, (2, 1)), 1);

        assert_eq!(count_outside_corners(&grid, (1, 2)), 2);
        assert_eq!(count_outside_corners(&grid, (2, 2)), 1);
        assert_eq!(count_outside_corners(&grid, (2, 3)), 1);
        assert_eq!(count_outside_corners(&grid, (3, 3)), 2);
    }
}
