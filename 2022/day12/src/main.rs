use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use pathfinding::prelude::Matrix;

fn parse_grid_input(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|line| line.chars().collect::<Vec<_>>())).unwrap()
}

fn solve_p1(input: &str) -> i32 {
    let mut grid = parse_grid_input(input);

    let start = grid.values().find_position(|&&ch| ch == 'S').unwrap();
    let start = (start.0 as usize / grid.columns, start.0 % grid.columns);
    let target = grid.values().find_position(|&&ch| ch == 'E').unwrap();
    let target = (target.0 as usize / grid.columns, target.0 % grid.columns);

    grid[start] = 'a';
    grid[target] = 'z';

    shortest_path(&grid, start, target).unwrap()
}

fn solve_p2(input: &str) -> i32 {
    let mut grid = parse_grid_input(input);

    let start = grid.values().find_position(|&&ch| ch == 'S').unwrap();
    let start = (start.0 as usize / grid.columns, start.0 % grid.columns);
    let target = grid.values().find_position(|&&ch| ch == 'E').unwrap();
    let target = (target.0 as usize / grid.columns, target.0 % grid.columns);

    grid[start] = 'a';
    grid[target] = 'z';

    grid.values()
        .enumerate()
        .filter_map(|(pos, &ch)| match ch {
            'a' => shortest_path(&grid, (pos / grid.columns, pos % grid.columns), target),
            _ => None,
        })
        .min()
        .unwrap()
}

fn shortest_path(grid: &Matrix<char>, start: (usize, usize), end: (usize, usize)) -> Option<i32> {
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

fn neighbors(grid: &Matrix<char>, row: usize, col: usize) -> Vec<(usize, usize)> {
    let max_allowed = grid[(row, col)] as i32 + 1;
    grid.neighbours((row, col), false)
        .filter(|(r, c)| grid[(*r, *c)] as i32 <= max_allowed)
        .collect()
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
