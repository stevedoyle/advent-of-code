use aoc_utils::parse_grid_input;
use grid::Grid;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;

fn solve_p1(input: &str) -> i32 {
    let mut grid = parse_grid_input(input);

    let start = grid.iter().find_position(|&&ch| ch == 'S').unwrap();
    let start = (start.0 as usize / grid.cols(), start.0 % grid.cols());
    let target = grid.iter().find_position(|&&ch| ch == 'E').unwrap();
    let target = (target.0 as usize / grid.cols(), target.0 % grid.cols());

    grid[start] = 'a';
    grid[target] = 'z';

    shortest_path(&grid, start, target).unwrap()
}

fn solve_p2(input: &str) -> i32 {
    let mut grid = parse_grid_input(input);

    let start = grid.iter().find_position(|&&ch| ch == 'S').unwrap();
    let start = (start.0 as usize / grid.cols(), start.0 % grid.cols());
    let target = grid.iter().find_position(|&&ch| ch == 'E').unwrap();
    let target = (target.0 as usize / grid.cols(), target.0 % grid.cols());

    grid[start] = 'a';
    grid[target] = 'z';

    grid.iter()
        .enumerate()
        .filter_map(|(pos, &ch)| match ch {
            'a' => shortest_path(&grid, (pos / grid.cols(), pos % grid.cols()), target),
            _ => None,
        })
        .min()
        .unwrap()
}

fn shortest_path(grid: &Grid<char>, start: (usize, usize), end: (usize, usize)) -> Option<i32> {
    let result = dijkstra(
        &start,
        |&(r, c)| neighbors(grid, r, c).into_iter().map(|p| (p, 1)),
        |&p| p == end,
    );
    if let Some((_, cost)) = result {
        return Some(cost);
    }
    None
}

fn neighbors(grid: &Grid<char>, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let nrows = grid.rows();
    let ncols = grid.cols();
    let max_allowed = grid[(row, col)] as i32 + 1;
    if row > 0 && grid[(row - 1, col)] as i32 <= max_allowed {
        result.push((row - 1, col));
    }
    if row < nrows - 1 && grid[(row + 1, col)] as i32 <= max_allowed {
        result.push((row + 1, col));
    }
    if col > 0 && grid[(row, col - 1)] as i32 <= max_allowed {
        result.push((row, col - 1));
    }
    if col < ncols - 1 && grid[(row, col + 1)] as i32 <= max_allowed {
        result.push((row, col + 1));
    }
    result
}

fn main() {
    let input = include_str!("../input.txt");
    let answer = solve_p1(input);
    println!("Part 1: {answer}");
    let answer = solve_p2(input);
    println!("Part 2: {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 31);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 29);
    }
}
