use std::{
    collections::{HashMap, HashSet},
    fs,
};

use pathfinding::matrix::Matrix;

fn parse_input(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|line| line.chars().collect::<Vec<_>>())).unwrap()
}

fn get_antinodes(
    grid: &Matrix<char>,
    first: (usize, usize),
    second: (usize, usize),
) -> Vec<(usize, usize)> {
    let diff = (
        second.0 as isize - first.0 as isize,
        second.1 as isize - first.1 as isize,
    );
    let mut antinodes = Vec::new();

    antinodes.extend(grid.move_in_direction(first, (-diff.0, -diff.1)));
    antinodes.extend(grid.move_in_direction(second, diff));
    antinodes
}

fn get_antinodes_with_harmonics(
    grid: &Matrix<char>,
    first: (usize, usize),
    second: (usize, usize),
) -> Vec<(usize, usize)> {
    let diff = (
        second.0 as isize - first.0 as isize,
        second.1 as isize - first.1 as isize,
    );
    let mut antinodes = Vec::new();

    antinodes.push(first);
    antinodes.push(second);
    antinodes.extend(grid.in_direction(first, diff));
    antinodes.extend(grid.in_direction(first, (-diff.0, -diff.1)));
    antinodes
}

fn solve_p1(input: &str) -> usize {
    let grid = parse_input(input);
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    grid.items().for_each(|((row, col), &ch)| {
        if ch != '.' {
            antennas.entry(ch).or_default().push((row, col));
        }
    });
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    antennas.iter().for_each(|(_, positions)| {
        let num_antennas = positions.len();
        for i in 0..num_antennas {
            for j in i + 1..num_antennas {
                antinodes.extend(get_antinodes(&grid, positions[i], positions[j]).iter());
            }
        }
    });
    antinodes.len()
}

fn solve_p2(input: &str) -> usize {
    let grid = parse_input(input);
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    grid.items().for_each(|((row, col), &ch)| {
        if ch != '.' {
            antennas.entry(ch).or_default().push((row, col));
        }
    });
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    antennas.iter().for_each(|(_, positions)| {
        let num_antennas = positions.len();
        for i in 0..num_antennas {
            for j in i + 1..num_antennas {
                antinodes
                    .extend(get_antinodes_with_harmonics(&grid, positions[i], positions[j]).iter());
            }
        }
    });
    antinodes.len()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

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
    use std::fs;

    use super::*;

    #[test]
    fn test_solve_with_test_input() {
        let input = fs::read_to_string("test.txt").unwrap();
        let answer = solve_p1(&input);
        assert_eq!(answer, 14);

        let simple_test = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";
        let answer = solve_p2(simple_test);
        assert_eq!(answer, 9);
        let answer = solve_p2(&input);
        assert_eq!(answer, 34);
    }
}
