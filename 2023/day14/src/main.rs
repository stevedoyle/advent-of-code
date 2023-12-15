use log::debug;
use std::{error::Error, fs};

use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|&line| !line.trim().is_empty())
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect()
}

fn sort_rocks(rocks: &[char], reverse_sort: bool) -> Vec<char> {
    if reverse_sort {
        rocks
            .iter()
            .collect::<String>()
            .split('#')
            .map(|part| part.chars().sorted().rev().collect::<String>())
            .join("#")
            .chars()
            .collect()
    } else {
        rocks
            .iter()
            .collect::<String>()
            .split('#')
            .map(|part| part.chars().sorted().collect::<String>())
            .join("#")
            .chars()
            .collect()
    }
}

fn calculate_load(rocks: &Vec<Vec<char>>) -> usize {
    transpose(rocks).iter().map(calculate_col_load).sum()
}

fn calculate_col_load(rocks: &Vec<char>) -> usize {
    let max = rocks.len();
    rocks
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, c)| acc + (max - idx) * rock_weight(*c))
}

fn rock_weight(c: char) -> usize {
    match c {
        'O' => 1,
        _ => 0,
    }
}

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn sort_by_row(platform: &[Vec<char>], reverse_sort: bool) -> Vec<Vec<char>> {
    platform
        .iter()
        .map(|row| sort_rocks(row, reverse_sort))
        .collect()
}

fn sort_by_col(platform: &Vec<Vec<char>>, reverse_sort: bool) -> Vec<Vec<char>> {
    let transposed: Vec<Vec<char>> = transpose(platform)
        .iter()
        .map(|row| sort_rocks(row, reverse_sort))
        .collect::<Vec<Vec<char>>>();
    transpose(&transposed)
}

fn cycle(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut cycled = sort_by_col(platform, true);
    cycled = sort_by_row(&cycled, true);
    cycled = sort_by_col(&cycled, false);
    sort_by_row(&cycled, false)
}

fn solve_p1(input: &str) -> usize {
    // The problem becomes a straightforward sorting problem if each colum of rocks is treated as a
    // string. Split it on #, sort it, rejoin with # and then calculate the load. Easy =)
    // let platform = preprocess(&parse_input(input));
    let platform = transpose(&parse_input(input));
    platform
        .iter()
        .map(|rocks| sort_rocks(rocks, true))
        .map(|rocks| calculate_col_load(&rocks))
        .sum()
}

fn solve_p2(input: &str) -> usize {
    let iteration_count = 1_000_000_000;

    let platform = parse_input(input);

    let mut cache: Vec<Vec<Vec<char>>> = Vec::with_capacity(1000);

    let mut tmp = platform.clone();

    for i in 0..iteration_count {
        tmp = cycle(&tmp);

        if cache.contains(&tmp) {
            // cycle detected
            let previous_idx = cache.iter().find_position(|x| **x == tmp).unwrap().0;
            let period = i - previous_idx;
            let target_idx = previous_idx - 1 + ((iteration_count - previous_idx) % period);

            debug!("Curr: {i}, Prev: {previous_idx}, Period: {period}, Target: {target_idx}");
            return calculate_load(&cache[target_idx]);
        } else {
            cache.push(tmp.clone());
        }
    }
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let load = solve_p1(&input);
    println!("Part 1: {load}");

    let load = solve_p2(&input);
    println!("Part 2: {load}");

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
        assert_eq!(grid.len(), 10);
        assert_eq!(grid[0].len(), 10);
    }

    fn str_to_vec(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    #[test]
    fn test_sort_rocks() {
        assert_eq!(
            sort_rocks(&str_to_vec("OO.O.O..##"), true),
            str_to_vec("OOOO....##")
        );
        assert_eq!(
            sort_rocks(&str_to_vec("...OO....O"), true),
            str_to_vec("OOO.......")
        );

        assert_eq!(
            sort_rocks(&str_to_vec(".O.O#....O"), true),
            str_to_vec("OO..#O....")
        );
        assert_eq!(
            sort_rocks(&str_to_vec(".O.O#.#..O"), true),
            str_to_vec("OO..#.#O..")
        );
    }

    #[test]
    fn test_calculate_col_load() {
        assert_eq!(calculate_col_load(&str_to_vec("OOOO....##")), 34);
        assert_eq!(calculate_col_load(&str_to_vec("O#OO.....#")), 25);
        assert_eq!(calculate_col_load(&str_to_vec(".#OO.....O")), 16);
    }

    #[test]
    fn test_calculate_load() {}

    #[test]
    fn test_solve_with_test_input() {
        let input = get_test_input();
        let answer = solve_p1(&input);
        assert_eq!(answer, 136);

        let answer = solve_p2(&input);
        assert_eq!(answer, 64);
    }

    #[test]
    fn test_solve() {
        let input = fs::read_to_string("input.txt").unwrap();
        let answer = solve_p1(&input);
        assert_eq!(answer, 106648);
        let answer = solve_p2(&input);
        assert_eq!(answer, 87700);
    }

    #[test]
    fn test_cycle() {
        let input = get_test_input();
        let platform = parse_input(&input);
        let cycled = cycle(&platform);
        let expected = parse_input(
            ".....#....
        ....#...O#
        ...OO##...
        .OO#......
        .....OOO#.
        .O#...O#.#
        ....O#....
        ......OOOO
        #...O###..
        #..OO#....",
        );
        assert_eq!(cycled, expected);

        let cycled = cycle(&cycle(&cycled));
        let expected = parse_input(
            ".....#....
        ....#...O#
        .....##...
        ..O#......
        .....OOO#.
        .O#...O#.#
        ....O#...O
        .......OOO
        #...O###.O
        #.OOO#...O",
        );
        assert_eq!(cycled, expected);
    }

    #[test]
    fn test_cycle_mini() {
        let input = "...\n.O.\n...";
        let platform = parse_input(&input);
        let cycled = cycle(&platform);
        let expected = parse_input("...\n...\n..O");
        assert_eq!(cycled, expected);
    }

    #[test]
    fn test_sort_by_row() {
        let input = "...\n.O.\n...";
        let platform = parse_input(input);
        let sorted = sort_by_row(&platform, true);
        let expected = parse_input("...\nO..\n...");
        assert_eq!(sorted, expected);

        let sorted = sort_by_row(&platform, false);
        let expected = parse_input("...\n..O\n...");
        assert_eq!(sorted, expected);
    }

    #[test]
    fn test_sort_by_col() {
        let input = "...\n.O.\n...";
        let platform = parse_input(input);
        let sorted = sort_by_col(&platform, true);
        let expected = parse_input(".O.\n...\n...");
        assert_eq!(sorted, expected);

        let sorted = sort_by_col(&platform, false);
        let expected = parse_input("...\n...\n.O.");
        assert_eq!(sorted, expected);

        let input = "...\n0..\n...";
        let platform = parse_input(input);
        let sorted = sort_by_col(&platform, true);
        let expected = parse_input("0..\n...\n...");
        assert_eq!(sorted, expected);
        let sorted = sort_by_col(&platform, false);
        let expected = parse_input("...\n...\n0..");
        assert_eq!(sorted, expected);
    }
}
