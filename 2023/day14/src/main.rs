use std::{error::Error, fs};

use grid::Grid;
use itertools::Itertools;

fn parse_input(input: &str) -> Grid<char> {
    let data: Vec<Vec<char>> = input
        .lines()
        .filter(|&line| !line.trim().is_empty())
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect();

    let ncols = data[0].len();
    Grid::from_vec(data.into_iter().flatten().collect(), ncols)
}

fn preprocess(grid: &Grid<char>) -> Vec<String> {
    grid.iter_cols()
        .map(|col| col.collect::<String>())
        .collect()
}

fn sort_rocks(rocks: &str) -> String {
    rocks
        .split('#')
        .map(|part| part.chars().sorted().rev().collect::<String>())
        .join("#")
}

fn calculate_load(rocks: &str) -> usize {
    let max = rocks.len();
    rocks
        .char_indices()
        .fold(0, |acc, (idx, c)| acc + (max - idx) * rock_weight(c))
}

fn rock_weight(c: char) -> usize {
    match c {
        'O' => 1,
        _ => 0,
    }
}

fn solve_p1(input: &str) -> usize {
    // The problem becomes a straightforward sorting problem if each colum of rocks is treated as a
    // string. Split it on #, sort it, rejoin with # and then calculate the load. Easy =)
    let platform = preprocess(&parse_input(input));
    platform
        .iter()
        .map(|rocks| sort_rocks(rocks))
        .map(|rocks| calculate_load(&rocks))
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let load = solve_p1(&input);
    println!("Part 1: {load}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        String::from(
            "O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....",
        )
    }

    #[test]
    fn test_parse_input() {
        let input = get_test_input();
        let grid = parse_input(&input);
        assert_eq!(grid.rows(), 10);
        assert_eq!(grid.cols(), 10);
    }

    #[test]
    fn test_preprocess() {
        let input = get_test_input();
        let grid = parse_input(&input);
        let cols = preprocess(&grid);
        assert_eq!(cols.len(), 10);
        assert_eq!(cols[0], "OO.O.O..##");
    }

    #[test]
    fn test_sort_rocks() {
        assert_eq!(sort_rocks("OO.O.O..##"), "OOOO....##");
        assert_eq!(sort_rocks("...OO....O"), "OOO.......");
        assert_eq!(sort_rocks(".O.O#....O"), "OO..#O....");
        assert_eq!(sort_rocks(".O.O#.#..O"), "OO..#.#O..");
    }

    #[test]
    fn test_calculate_load() {
        assert_eq!(calculate_load("OOOO....##"), 34);
        assert_eq!(calculate_load("O#OO.....#"), 25);
        assert_eq!(calculate_load(".#OO.....O"), 16);
    }

    #[test]
    fn test_solve_with_test_input() {
        let input = get_test_input();
        let answer = solve_p1(&input);
        assert_eq!(answer, 136);
    }

    #[test]
    fn test_solve() {
        let input = fs::read_to_string("input.txt").unwrap();
        let answer = solve_p1(&input);
        assert_eq!(answer, 106648);
    }
}
