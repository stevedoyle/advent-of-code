use aoc2025::*;

fn parse_input(input: &str) -> Grid<char> {
    parse_grid(input)
}

fn find_moveable_tps(grid: &Grid<char>) -> Vec<(usize, usize)> {
    grid::positions(grid)
        .filter(|&(r, c)| grid[r][c] == '@')
        .filter(|&(r, c)| {
            grid::valid_neighbors8(grid, r as isize, c as isize)
                .iter()
                .filter(|(nr, nc)| grid[*nr][*nc] == '@')
                .count()
                < 4
        })
        .collect()
}

fn remove_tps(grid: &mut Grid<char>, tps: &[(usize, usize)]) {
    for &(r, c) in tps {
        grid[r][c] = '.';
    }
}

fn solve_p1(input: &str) -> usize {
    let grid = parse_input(input);
    find_moveable_tps(&grid).len()
}

fn solve_p2(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut total = 0;
    loop {
        let moveable_tps = find_moveable_tps(&grid);
        if moveable_tps.is_empty() {
            break;
        }
        total += moveable_tps.len();
        remove_tps(&mut grid, &moveable_tps);
    }
    total
}

fn main() {
    let input = read_input(4);

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
        let input = read_test_input(4);
        let answer = solve_p1(&input);
        assert_eq!(answer, 13);
        let answer = solve_p2(&input);
        assert_eq!(answer, 43);
    }
}
