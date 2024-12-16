use pathfinding::{
    matrix::Matrix,
    prelude::{astar, astar_bag},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Reindeer {
    pos: (usize, usize),
    dir: Direction,
}

impl Reindeer {
    fn new(pos: (usize, usize), dir: Direction) -> Self {
        Self { pos, dir }
    }
}

fn parse_input(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|line| line.chars().collect::<Vec<_>>())).unwrap()
}

fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn successors(grid: &Matrix<char>, r: &Reindeer) -> Vec<(Reindeer, usize)> {
    let mut potential_positions = Vec::new();

    match r.dir {
        Direction::Up => {
            potential_positions.push((Reindeer::new((r.pos.0 - 1, r.pos.1), Direction::Up), 1));
            potential_positions
                .push((Reindeer::new((r.pos.0, r.pos.1 - 1), Direction::Left), 1001));
            potential_positions.push((
                Reindeer::new((r.pos.0, r.pos.1 + 1), Direction::Right),
                1001,
            ));
        }
        Direction::Down => {
            potential_positions.push((Reindeer::new((r.pos.0 + 1, r.pos.1), Direction::Down), 1));
            potential_positions
                .push((Reindeer::new((r.pos.0, r.pos.1 - 1), Direction::Left), 1001));
            potential_positions.push((
                Reindeer::new((r.pos.0, r.pos.1 + 1), Direction::Right),
                1001,
            ));
        }
        Direction::Left => {
            potential_positions.push((Reindeer::new((r.pos.0, r.pos.1 - 1), Direction::Left), 1));
            potential_positions.push((Reindeer::new((r.pos.0 - 1, r.pos.1), Direction::Up), 1001));
            potential_positions
                .push((Reindeer::new((r.pos.0 + 1, r.pos.1), Direction::Down), 1001));
        }
        Direction::Right => {
            potential_positions.push((Reindeer::new((r.pos.0, r.pos.1 + 1), Direction::Right), 1));
            potential_positions.push((Reindeer::new((r.pos.0 - 1, r.pos.1), Direction::Up), 1001));
            potential_positions
                .push((Reindeer::new((r.pos.0 + 1, r.pos.1), Direction::Down), 1001));
        }
    }
    potential_positions.retain(|(n, _)| grid[n.pos] != '#');
    potential_positions
}

fn solve_p1(input: &str) -> usize {
    let grid = parse_input(input);
    let start = grid.values().position(|&x| x == 'S').unwrap();
    let start = (start / grid.columns, start % grid.columns);
    let goal = grid.values().position(|&x| x == 'E').unwrap();
    let goal = (goal / grid.columns, goal % grid.columns);

    let reidneer_start = Reindeer::new(start, Direction::Right);

    astar(
        &reidneer_start,
        |r| successors(&grid, r),
        |r| distance(r.pos, goal),
        |r| r.pos == goal,
    )
    .unwrap()
    .1
}

fn solve_p2(input: &str) -> usize {
    let grid = parse_input(input);
    let start = grid.values().position(|&x| x == 'S').unwrap();
    let start = (start / grid.columns, start % grid.columns);
    let goal = grid.values().position(|&x| x == 'E').unwrap();
    let goal = (goal / grid.columns, goal % grid.columns);

    let reidneer_start = Reindeer::new(start, Direction::Right);

    let paths = astar_bag(
        &reidneer_start,
        |r| successors(&grid, r),
        |r| distance(r.pos, goal),
        |r| r.pos == goal,
    )
    .unwrap();
    let mut unique_positions = std::collections::HashSet::new();
    for path in paths.0 {
        for r in path {
            unique_positions.insert(r.pos);
        }
    }
    unique_positions.len()
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

    #[test]
    fn test_solve_with_test_input() {
        let test_input_1 = std::fs::read_to_string("test_input_1.txt").unwrap();
        let test_input_2 = std::fs::read_to_string("test_input_2.txt").unwrap();
        let answer = solve_p1(&test_input_1);
        assert_eq!(answer, 7036);
        let answer = solve_p1(&test_input_2);
        assert_eq!(answer, 11048);
        let answer = solve_p2(&test_input_1);
        assert_eq!(answer, 45);
        let answer = solve_p2(&test_input_2);
        assert_eq!(answer, 64);
    }
}
