use pathfinding::matrix::Matrix;
use pathfinding::prelude::astar;

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once(",").unwrap();
            (lhs.parse().unwrap(), rhs.parse().unwrap())
        })
        .collect()
}

fn successors(grid: &Matrix<char>, pos: (usize, usize)) -> Vec<((usize, usize), usize)> {
    grid.neighbours(pos, false)
        .filter(|&p| grid[p] == '.')
        .map(|p| (p, 1))
        .collect()
}

fn distance(pos: &(usize, usize), goal: &(usize, usize)) -> usize {
    pos.0.abs_diff(goal.0) + pos.1.abs_diff(goal.1)
}

fn solve_p1(input: &str, bound: usize, block_count: usize) -> usize {
    let data = parse_input(input);
    let mut matrix: Matrix<char> = Matrix::new(bound + 1, bound + 1, '.');
    for (x, y) in &data[..block_count] {
        matrix[(*y, *x)] = '#';
    }
    let pos = (0, 0);
    let goal = (bound, bound);
    let path = astar(
        &pos,
        |p| successors(&matrix, *p),
        |p| distance(p, &goal),
        |p| *p == goal,
    )
    .unwrap();
    path.1
}

fn solve_p2(input: &str, bound: usize, start_check: usize) -> (usize, usize) {
    let data = parse_input(input);
    let mut matrix: Matrix<char> = Matrix::new(bound + 1, bound + 1, '.');
    for (x, y) in &data[..start_check] {
        matrix[(*y, *x)] = '#';
    }

    for (x, y) in &data[start_check..] {
        matrix[(*y, *x)] = '#';
        let pos = (0, 0);
        let goal = (bound, bound);
        let path = astar(
            &pos,
            |p| successors(&matrix, *p),
            |p| distance(p, &goal),
            |p| *p == goal,
        );
        if path.is_none() {
            return (*x, *y);
        }
    }
    (0, 0)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let start = std::time::Instant::now();
    let answer = solve_p1(&input, 70, 1024);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:.1?}");

    let start = std::time::Instant::now();
    let answer = solve_p2(&input, 70, 1024);
    let elapsed = start.elapsed();
    println!("Part 2: {answer:?}, elapsed: {elapsed:.1?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_with_test_input() {
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let answer = solve_p1(&input, 6, 12);
        assert_eq!(answer, 22);
        let answer = solve_p2(&input, 6, 12);
        assert_eq!(answer, (6, 1));
    }
}
