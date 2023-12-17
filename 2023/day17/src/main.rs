use anyhow::Result;
use grid::Grid;
use pathfinding::prelude::dijkstra;
use std::{cmp::min, fs};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn parse_input(input: &str) -> Grid<usize> {
    let mut grid = Grid::new(0, 0);

    input
        .lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            grid.push_row(
                line.trim()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect(),
            )
        });

    grid
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize, i64, Direction);

impl Pos {
    fn successors(&self, grid: &Grid<usize>, min: i64, max: i64) -> Vec<(Pos, usize)> {
        let &Pos(x, y, count, dir) = self;
        let directions = [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ];
        let mut results = vec![];
        for d in directions.iter() {
            if *d == dir && count == max {
                continue;
            }
            if *d != dir && count < min {
                continue;
            }
            match (*d, dir) {
                (Direction::North, Direction::South) => {
                    continue;
                }
                (Direction::South, Direction::North) => {
                    continue;
                }
                (Direction::East, Direction::West) => {
                    continue;
                }
                (Direction::West, Direction::East) => {
                    continue;
                }
                _ => {}
            };
            if *d == Direction::North && y == 0 {
                continue;
            }
            if *d == Direction::West && x == 0 {
                continue;
            }
            if *d == Direction::South && y == grid.rows() - 1 {
                continue;
            }
            if *d == Direction::East && x == grid.cols() - 1 {
                continue;
            }
            let next_count = if *d == dir { count + 1 } else { 1 };
            let (nx, ny) = match d {
                Direction::North => (x, y - 1),
                Direction::South => (x, y + 1),
                Direction::West => (x - 1, y),
                Direction::East => (x + 1, y),
            };
            let cost = *grid.get(ny, nx).unwrap();
            results.push((Pos(nx, ny, next_count, *d), cost));
        }
        results
    }

    fn is_goal(&self, grid: &Grid<usize>) -> bool {
        let &Pos(x, y, _, _) = self;
        x == grid.cols() - 1 && y == grid.rows() - 1
    }
}

// fn process(grid: &Grid<u32>) -> usize {
//     let mut unvisited: BTreeSet<(usize, usize)> = BTreeSet::new();
//     let mut distances: HashMap<(usize, usize), i32> =
//         HashMap::with_capacity(grid.cols() * grid.rows());
//     for ((row, col), _) in grid.indexed_iter() {
//         unvisited.insert((row, col));
//         distances.insert((row, col), -1);
//     }
//     *distances.get_mut(&(0, 0)).unwrap() = 0;

//     0
// }

fn solve_p1(input: &str) -> usize {
    let grid = parse_input(input);
    dijkstra(
        &Pos(0, 0, 0, Direction::East),
        |p| p.successors(&grid, 0, 3),
        |p| p.is_goal(&grid),
    )
    .unwrap()
    .1
}

fn solve_p2(input: &str) -> usize {
    let grid = parse_input(input);
    let min_dist_e = dijkstra(
        &Pos(0, 0, 0, Direction::East),
        |p| p.successors(&grid, 4, 10),
        |p| p.is_goal(&grid),
    )
    .unwrap()
    .1;

    let min_dist_s = dijkstra(
        &Pos(0, 0, 0, Direction::South),
        |p| p.successors(&grid, 4, 10),
        |p| p.is_goal(&grid),
    )
    .unwrap()
    .1;

    min(min_dist_e, min_dist_s)
}

fn main() -> Result<()> {
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

    const INPUT: &str = "
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533";

    #[test]
    fn test_parse_input() {
        let grid = parse_input(INPUT);
        assert_eq!(grid.get(0, 0), Some(&2));
        assert_eq!(grid.get(grid.rows() - 1, grid.cols() - 1), Some(&3));
    }

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 102);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 94);
    }

    #[test]
    fn test_solve() {
        let input = include_str!("../input.txt");
        let answer = solve_p1(input);
        assert_eq!(answer, 1008);
        let answer = solve_p2(input);
        assert_eq!(answer, 1210);
    }
}
