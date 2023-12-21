use std::collections::HashSet;

use grid::Grid;

fn parse_input(input: &str) -> Grid<char> {
    let mut grid = Grid::new(0, 0);

    input
        .lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| grid.push_row(line.trim().chars().collect()));

    grid
}

fn get_start(grid: &Grid<char>) -> (usize, usize) {
    grid.indexed_iter().find(|((_, _), &x)| x == 'S').unwrap().0
}

fn get_neighbours(grid: &Grid<char>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neigh = vec![];
    let (row, col) = pos;
    if row > 0 {
        // North
        neigh.push((row - 1, col));
    }
    if row < grid.rows() - 1 {
        // South
        neigh.push((row + 1, col));
    }
    if col > 0 {
        // West
        neigh.push((row, col - 1));
    }
    if col < grid.cols() - 1 {
        // East
        neigh.push((row, col + 1));
    }
    neigh
        .iter()
        .filter(|(row, col)| {
            let c = grid.get(*row, *col).unwrap();
            *c == '.' || *c == 'S'
        })
        .cloned()
        .collect()
}

fn process(grid: &Grid<char>, start: (usize, usize), steps: usize) -> HashSet<(usize, usize)> {
    let mut destinations: HashSet<(usize, usize)> = HashSet::new();
    destinations.insert(start);

    for _ in 0..steps {
        let candidates: Vec<(usize, usize)> = destinations
            .iter()
            .map(|pos| get_neighbours(grid, *pos))
            .flatten()
            .collect();
        destinations = HashSet::from_iter(candidates.iter().cloned());
    }

    destinations
}

fn solve_p1(input: &str, steps: usize) -> usize {
    let grid = parse_input(input);
    let start = get_start(&grid);
    let positions = process(&grid, start, steps);
    positions.len()
}

fn main() {
    let input = include_str!("../input.txt");
    let answer = solve_p1(input, 64);
    println!("Part 1: {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........";

    #[test]
    fn test_parse_input() {
        let grid = parse_input(INPUT);
        assert_eq!(grid.rows(), 11);
        assert_eq!(grid.cols(), 11);
        assert_eq!(get_start(&grid), (5, 5));
    }

    #[test]
    fn test_process() {
        let grid = parse_input(INPUT);
        let start = get_start(&grid);
        let positions = process(&grid, start, 6);
        assert_eq!(positions.len(), 16);
    }

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT, 6);
        assert_eq!(answer, 16);
    }
}
