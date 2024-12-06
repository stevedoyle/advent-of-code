use std::collections::HashSet;

use pathfinding::matrix::Matrix;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|line| line.chars().collect::<Vec<_>>())).unwrap()
}

fn next_pos(curr_pos: (usize, usize), dir: Direction) -> (isize, isize) {
    let curr_pos: (isize, isize) = (curr_pos.0 as isize, curr_pos.1 as isize);
    match dir {
        Direction::Up => (curr_pos.0 - 1, curr_pos.1),
        Direction::Down => (curr_pos.0 + 1, curr_pos.1),
        Direction::Left => (curr_pos.0, curr_pos.1 - 1),
        Direction::Right => (curr_pos.0, curr_pos.1 + 1),
    }
}

fn turn_right(curr_dir: Direction) -> Direction {
    match curr_dir {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}

fn walk(grid: &Matrix<char>, start: (usize, usize), dir: Direction) -> Option<Vec<(usize, usize)>> {
    let mut path = Vec::new();
    let mut pos = start;
    let mut dir = dir;
    path.push(pos);
    let mut visited = HashSet::new();

    loop {
        let next = next_pos(pos, dir);
        if next.0 < 0
            || next.0 >= grid.rows as isize
            || next.1 < 0
            || next.1 >= grid.columns as isize
        {
            // We have moved off the grid so we are done.
            break;
        }
        let next = (next.0 as usize, next.1 as usize);
        if grid[next] == '#' {
            // Only checkinf for loops when we hit an obstacle.
            if visited.contains(&(pos, dir)) {
                // We have been here before so we are in a loop.
                return None;
            }
            visited.insert((pos, dir));

            // We have hit a wall so we turn right.
            dir = turn_right(dir);
            continue;
        }
        pos = next;
        path.push(pos);
    }
    Some(path)
}

fn solve_p1(input: &str) -> usize {
    let grid = parse_input(input);
    let start = grid.values().position(|&c| c == '^').unwrap();
    let start = (start / grid.columns, start % grid.columns);

    let path = walk(&grid, start, Direction::Up).unwrap_or_default();
    let mut unique_positions = HashSet::new();
    path.iter().for_each(|p| {
        unique_positions.insert(p);
    });
    unique_positions.len()
}

fn solve_p2(input: &str) -> usize {
    let mut grid = parse_input(input);
    let start = grid.values().position(|&c| c == '^').unwrap();
    let start = (start / grid.columns, start % grid.columns);

    let path = walk(&grid, start, Direction::Up).unwrap();

    let mut obstacles = HashSet::new();
    path.iter().skip(1).for_each(|p| {
        grid[*p] = '#';
        if walk(&grid, start, Direction::Up).is_none() {
            obstacles.insert(*p);
        }
        grid[*p] = '.';
    });
    obstacles.len()
}

fn main() {
    let input = include_str!("../input.txt");

    let start = std::time::Instant::now();
    let answer = solve_p1(input);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:.1?}");

    let start = std::time::Instant::now();
    let answer = solve_p2(input);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, elapsed: {elapsed:.1?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 41);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 6);
    }
}
