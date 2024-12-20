use std::collections::HashMap;

use pathfinding::{matrix::Matrix, prelude::dijkstra};

fn parse_input(input: &str) -> Matrix<char> {
    Matrix::from_rows(
        input
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>(),
    )
    .unwrap()
}

fn successors(pos: (usize, usize), grid: &Matrix<char>) -> Vec<((usize, usize), i32)> {
    grid.neighbours(pos, false)
        .filter(|p| grid[*p] != '#')
        .map(|p| (p, 1))
        .collect()
}

fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + (a.1.abs_diff(b.1))
}

fn get_cheats(grid: &Matrix<char>, path: &[(usize, usize)], pos: (usize, usize)) -> Vec<usize> {
    let mut cheats = Vec::new();
    if pos.0 > 1 && grid[(pos.0 - 1, pos.1)] == '#' {
        if let Some(idx) = path.iter().position(|&p| p == (pos.0 - 2, pos.1)) {
            cheats.push(idx - 1);
        }
    }
    if pos.0 < grid.rows - 2 && grid[(pos.0 + 1, pos.1)] == '#' {
        if let Some(idx) = path.iter().position(|&p| p == (pos.0 + 2, pos.1)) {
            cheats.push(idx - 1);
        }
    }
    if pos.1 > 1 && grid[(pos.0, pos.1 - 1)] == '#' {
        if let Some(idx) = path.iter().position(|&p| p == (pos.0, pos.1 - 2)) {
            cheats.push(idx - 1);
        }
    }
    if pos.1 < grid.columns - 2 && grid[(pos.0, pos.1 + 1)] == '#' {
        if let Some(idx) = path.iter().position(|&p| p == (pos.0, pos.1 + 2)) {
            cheats.push(idx - 1);
        }
    }

    cheats
}

fn get_cheats_within_distance(
    path: &[(usize, usize)],
    pos: (usize, usize),
    max_distance: usize,
    threshold: usize,
) -> usize {
    let mut cheats = 0;
    for (idx, p) in path.iter().enumerate() {
        let manhattan = distance(pos, *p);
        if manhattan > 1 && manhattan <= max_distance && manhattan < idx + 1 {
            let saving = idx + 1 - manhattan;
            if saving >= threshold {
                cheats += 1;
            }
        }
    }
    cheats
}

fn solve_p1(grid: &Matrix<char>, threshold: usize) -> usize {
    let start = grid.values().position(|&c| c == 'S').unwrap();
    let start = (start / grid.columns, start % grid.columns);
    let end = grid.values().position(|&c| c == 'E').unwrap();
    let end = (end / grid.columns, end % grid.columns);

    // First get the 'race track' path
    let (path, _cost) = dijkstra(&start, |p| successors(*p, grid), |p| p == &end).unwrap();

    // Now, find the cheats
    let mut cheats: HashMap<usize, usize> = HashMap::new();
    for (i, step) in path.iter().enumerate() {
        let cheats_at_current_pos = get_cheats(grid, &path[i + 1..], *step);
        if !cheats_at_current_pos.is_empty() {
            cheats_at_current_pos.iter().for_each(|c| {
                *cheats.entry(*c).or_insert(0) += 1;
            });
        }
    }

    cheats
        .iter()
        .filter(|(&k, _)| k >= threshold)
        .map(|(_, v)| v)
        .sum()
}

fn solve_p2(grid: &Matrix<char>, threshold: usize) -> usize {
    let start = grid.values().position(|&c| c == 'S').unwrap();
    let start = (start / grid.columns, start % grid.columns);
    let end = grid.values().position(|&c| c == 'E').unwrap();
    let end = (end / grid.columns, end % grid.columns);

    // First get the 'race track' path
    let (path, _cost) = dijkstra(&start, |p| successors(*p, grid), |p| p == &end).unwrap();

    // Now, find the cheats above the threshold and within the max allowed cheat time
    path.iter()
        .enumerate()
        .map(|(i, step)| get_cheats_within_distance(&path[i + 1..], *step, 20, threshold))
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let grid = parse_input(&input);

    let start = std::time::Instant::now();
    let answer = solve_p1(&grid, 100);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:.1?}");

    let start = std::time::Instant::now();
    let answer = solve_p2(&grid, 100);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, elapsed: {elapsed:.1?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_with_test_input() {
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let grid = parse_input(&input);

        let answer = solve_p1(&grid, 0);
        assert_eq!(answer, 44);
        let answer = solve_p1(&grid, 40);
        assert_eq!(answer, 2);
        let answer = solve_p2(&grid, 50);
        assert_eq!(answer, 285);
    }
}
