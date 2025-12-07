use std::collections::{HashMap, HashSet};

use aoc2025::*;

fn find_start(manifold: &[Vec<char>]) -> (usize, usize) {
    let start_col = manifold[0]
        .iter()
        .position(|&c| c == 'S')
        .expect("Start position not found");
    (0, start_col)
}

fn solve_p1(input: &str) -> usize {
    let manifold = parse_grid(input);
    let mut rays = HashSet::new();
    let mut splits = 0;
    let (_, start_col) = find_start(&manifold);
    rays.insert(start_col);

    for row in &manifold[1..] {
        let mut next_rays = HashSet::new();
        for &col in &rays {
            if row[col] == '^' {
                // Hit a splitter, branch left and right
                next_rays.insert(col - 1);
                next_rays.insert(col + 1);
                splits += 1;
            } else {
                // Continue straight
                next_rays.insert(col);
            }
        }
        rays = next_rays;
    }
    splits
}

fn count_paths(
    manifold: &[Vec<char>],
    pos: (usize, usize),
    target_row: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    // Check memo first
    if let Some(&cached) = memo.get(&pos) {
        return cached;
    }

    let (r, c) = pos;

    // Base case: reached bottom row
    if r == target_row {
        return 1;
    }

    let next_r = r + 1;
    let next_cell = manifold[next_r][c];

    let result = if next_cell != '^' {
        // Continue straight down
        count_paths(manifold, (next_r, c), target_row, memo)
    } else {
        // Hit a splitter, sum paths from both branches
        let mut total = 0;

        // Left branch
        if manifold[next_r][c - 1] != '^' {
            total += count_paths(manifold, (next_r, c - 1), target_row, memo);
        }

        // Right branch
        if manifold[next_r][c + 1] != '^' {
            total += count_paths(manifold, (next_r, c + 1), target_row, memo);
        }

        total
    };

    memo.insert(pos, result);
    result
}

fn solve_p2(input: &str) -> usize {
    let manifold = parse_grid(input);
    let start = find_start(&manifold);
    let target_row = manifold.len() - 1;
    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();
    count_paths(&manifold, start, target_row, &mut memo)
}

fn main() {
    let input = read_input(7);

    let start = std::time::Instant::now();
    let answer = solve_p1(&input);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:.1?}");

    let start = std::time::Instant::now();
    let answer = solve_p2(&input);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, elapsed: {elapsed:.1?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_with_test_input() {
        let input = read_test_input(7);
        let answer = solve_p1(&input);
        assert_eq!(answer, 21);
        let answer = solve_p2(&input);
        assert_eq!(answer, 40);
    }
}
