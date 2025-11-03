use pathfinding::matrix::Matrix;

fn parse_input(input: &str) -> Matrix<usize> {
    Matrix::from_rows(input.lines().map(|line| {
        line.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>()
    }))
    .unwrap()
}

fn solve_p1(input: &str) -> i32 {
    let grid = parse_input(input);
    // Find low points
    let low_points = grid
        .keys()
        .filter(|&idx| {
            let current = grid[idx];
            grid.neighbours(idx, false)
                .all(|n_idx| grid[n_idx] > current)
        })
        .collect::<Vec<_>>();
    let risk_level: i32 = low_points.iter().map(|&idx| grid[idx] as i32 + 1).sum();
    risk_level
}

fn solve_p2(input: &str) -> i32 {
    let grid = parse_input(input);
    let mut basin_sizes = Vec::new();
    let mut visited = std::collections::HashSet::new();
    for idx in grid.keys() {
        if grid[idx] == 9 || visited.contains(&idx) {
            continue;
        }
        // Perform flood fill to find basin size
        let mut to_visit = vec![idx];
        let mut basin_size = 0;
        while let Some(current) = to_visit.pop() {
            if visited.contains(&current) || grid[current] == 9 {
                continue;
            }
            visited.insert(current);
            basin_size += 1;
            for n_idx in grid.neighbours(current, false) {
                if !visited.contains(&n_idx) && grid[n_idx] != 9 {
                    to_visit.push(n_idx);
                }
            }
        }
        basin_sizes.push(basin_size);
    }
    basin_sizes.sort_unstable_by(|a, b| b.cmp(a));
    let largest_three_product: i32 = basin_sizes.iter().take(3).product::<usize>() as i32;
    largest_three_product
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

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
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let answer = solve_p1(&input);
        assert_eq!(answer, 15);
        let answer = solve_p2(&input);
        assert_eq!(answer, 1134);
    }
}
