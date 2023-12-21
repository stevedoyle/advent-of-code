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

fn get_neighbours_inf(grid: &Grid<char>, pos: (isize, isize)) -> Vec<(isize, isize)> {
    let mut neigh = vec![];
    let (row, col) = pos;
    neigh.push((row - 1, col));
    neigh.push((row + 1, col));
    neigh.push((row, col - 1));
    neigh.push((row, col + 1));

    neigh
        .iter()
        .filter(|(row, col)| {
            let r = row.rem_euclid(grid.rows() as isize);
            let c = col.rem_euclid(grid.cols() as isize);
            let value = grid.get(r, c).unwrap();
            *value == '.' || *value == 'S'
        })
        .cloned()
        .collect()
}

fn process(grid: &Grid<char>, start: (usize, usize), steps: usize) -> usize {
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

    destinations.len()
}

fn process_inf(grid: &Grid<char>, start: (usize, usize), steps: usize) -> usize {
    let mut destinations: HashSet<(isize, isize)> = HashSet::new();
    destinations.insert((start.0 as isize, start.1 as isize));

    for _ in 0..steps {
        let candidates: Vec<(isize, isize)> = destinations
            .iter()
            .map(|pos| get_neighbours_inf(grid, *pos))
            .flatten()
            .collect();
        destinations = HashSet::from_iter(candidates.iter().cloned());
    }

    destinations.len()
}

fn process_p2(grid: &Grid<char>, start: (usize, usize), steps: usize) -> usize {
    // grid for part 2 is 131 characters wide.
    // We want to walk up to 26501365 squares, which is to say:
    // 65 squares plus 202300 times the length of the grid
    // 65 = number of steps to reach the edge from the starting point
    // This will only solve for the case where
    // steps mod squaresize == square size/2 -1
    let max_grid_traversals = steps / grid.rows();
    let mut progression = Vec::new();
    let grid_size = grid.rows();
    let distance_in_final_grid = steps % grid_size;

    // number of input squares it's possible to visit increases with geometric progession.
    // size of area compared with original map covers increases with step distance as:
    // 1, 9, 25, 49
    // (quadratic relationship) It follows that there will be a similar relactionship
    // between number of squares it's possible to visit and area covered. Hence:
    // quadratic progression, which we can get the first three values of by using
    // part 1 code:
    for i in 0..=2 {
        dbg!(distance_in_final_grid + i * grid_size);
        progression.push(process_inf(
            grid,
            start,
            distance_in_final_grid + i * grid_size,
        ));
    }
    dbg!(&progression);
    dbg!(max_grid_traversals);
    dbg!(steps);
    dbg!(grid_size);
    dbg!(distance_in_final_grid);
    // We know the relationship is quadratic, and the first three values in the sequence
    // Hence we can calculate the rest.
    progression[0]
        + progression[1] * max_grid_traversals
        + (max_grid_traversals * (max_grid_traversals - 1) / 2)
            * ((progression[2] - progression[1]) - (progression[1] - progression[0]))
}

fn solve_p1(input: &str, steps: usize) -> usize {
    let grid = parse_input(input);
    let start = get_start(&grid);
    process(&grid, start, steps)
}

fn solve_palt(input: &str, steps: usize) -> usize {
    let grid = parse_input(input);
    let start = get_start(&grid);
    process_inf(&grid, start, steps)
}

fn solve_p2(input: &str, steps: usize) -> usize {
    let grid = parse_input(input);
    let start = get_start(&grid);
    process_p2(&grid, start, steps)
}

fn main() {
    let input = include_str!("../input.txt");
    let answer = solve_p1(input, 64);
    println!("Part 1: {answer}");
    // let answer = solve_p2(input, 26501365);
    let answer = solve_palt(input, 26501365);
    println!("Part 2: {answer}");
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
        assert_eq!(positions, 16);
    }

    #[test]
    fn test_solve_with_test_input() {
        assert_eq!(solve_p1(INPUT, 6), 16);
        assert_eq!(solve_palt(INPUT, 6), 16);
        assert_eq!(solve_palt(INPUT, 10), 50);
        assert_eq!(solve_palt(INPUT, 50), 1594);
        assert_eq!(solve_palt(INPUT, 100), 6536);
        assert_eq!(solve_palt(INPUT, 500), 167004);
        assert_eq!(solve_palt(INPUT, 1000), 668697);
        // assert_eq!(solve_palt(INPUT, 5000), 16733044);
    }

    #[test]
    fn test_solve() {
        let input = include_str!("../input.txt");
        assert_eq!(solve_p1(input, 64), 3699);
        // assert_eq!(solve_p2(INPUT, 26501365), ??);
    }
}
