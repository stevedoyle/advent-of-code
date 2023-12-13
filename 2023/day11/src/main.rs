use grid::Grid;
use itertools::Itertools;
use std::{
    cmp::{max, min},
    error::Error,
    fs,
};

fn parse_input(input: &str) -> Grid<char> {
    let data: Vec<Vec<char>> = input
        .lines()
        .filter(|&line| !line.trim().is_empty())
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect();

    let ncols = data[0].len();
    Grid::from_vec(data.into_iter().flatten().collect(), ncols)
}

fn get_empty_rows(grid: &Grid<char>) -> Vec<usize> {
    grid.iter_rows()
        .enumerate()
        .filter(|(_, row)| !row.clone().any(|c| *c == '#'))
        .map(|(idx, _)| idx)
        .collect()
}

fn get_empty_cols(grid: &Grid<char>) -> Vec<usize> {
    grid.iter_cols()
        .enumerate()
        .filter(|(_, col)| !col.clone().any(|c| *c == '#'))
        .map(|(idx, _)| idx)
        .collect()
}

fn get_galaxies(grid: &Grid<char>) -> Vec<Point> {
    grid.indexed_iter()
        .filter(|((_, _), &symbol)| symbol == '#')
        .map(|((r, c), _)| Point(c, r))
        .collect()
}

fn distance(a: &Point, b: &Point) -> usize {
    let (x1, y1) = (a.0 as i32, a.1 as i32);
    let (x2, y2) = (b.0 as i32, b.1 as i32);
    let distance = (x2 - x1).abs() + (y2 - y1).abs();
    distance as usize
}

fn distance_with_expansion(
    a: &Point,
    b: &Point,
    empty_rows: &[usize],
    empty_cols: &[usize],
    expansion_factor: usize,
) -> usize {
    let (rows, cols) = count_between(a, b, empty_rows, empty_cols);
    let n_empty = rows + cols;
    distance(a, b) + (n_empty * (expansion_factor - 1))
}

fn count_between(
    a: &Point,
    b: &Point,
    empty_rows: &[usize],
    empty_cols: &[usize],
) -> (usize, usize) {
    let cols = empty_cols
        .iter()
        .filter(|&&x| min(a.0, b.0) < x && x < max(a.0, b.0))
        .count();

    let rows = empty_rows
        .iter()
        .filter(|&&y| min(a.1, b.1) < y && y < max(a.1, b.1))
        .count();

    (rows, cols)
}

fn solve(input: &str, expansion_factor: usize) -> usize {
    let grid = parse_input(input);
    let empty_rows = get_empty_rows(&grid);
    let empty_cols = get_empty_cols(&grid);
    let galaxies = get_galaxies(&grid);
    galaxies
        .iter()
        .combinations(2)
        .map(|x| distance_with_expansion(x[0], x[1], &empty_rows, &empty_cols, expansion_factor))
        .sum()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point(usize, usize);

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let sum_of_distances = solve(&input, 2);
    println!("Part 1: {sum_of_distances}");

    let sum_of_distances = solve(&input, 1_000_000);
    println!("Part 2: {sum_of_distances}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....",
        )
    }

    #[test]
    fn test_get_galaxies() {
        let grid = parse_input(&get_input());
        let galaxies = get_galaxies(&grid);
        dbg!(&grid);
        assert_eq!(galaxies.len(), 9);
        assert_eq!(galaxies[0], Point(3, 0));
        assert_eq!(galaxies[8], Point(4, 9));
    }

    #[test]
    fn test_distance() {
        let p1 = Point(3, 0);
        let p2 = Point(4, 9);
        assert_eq!(distance(&p1, &p2), 10)
    }

    #[test]
    fn test_get_empty_counts() {
        let grid = parse_input(&get_input());
        let empty_rows = get_empty_rows(&grid);
        assert_eq!(empty_rows.len(), 2);
        let empty_cols = get_empty_cols(&grid);
        assert_eq!(empty_cols.len(), 3);
    }

    #[test]
    fn test_count_between() {
        let grid = parse_input(&get_input());
        let empty_rows = get_empty_rows(&grid);
        assert_eq!(empty_rows.len(), 2);
        assert_eq!(empty_rows, vec![3, 7]);
        let empty_cols = get_empty_cols(&grid);
        assert_eq!(empty_cols.len(), 3);
        assert_eq!(empty_cols, vec![2, 5, 8]);
        let p1 = Point(3, 0);
        let p2 = Point(4, 9);
        let (nrows, ncols) = count_between(&p1, &p2, &empty_rows, &empty_cols);
        assert_eq!(nrows, 2);
        assert_eq!(ncols, 0);

        let p1 = Point(0, 2);
        let p2 = Point(4, 9);
        let (nrows, ncols) = count_between(&p1, &p2, &empty_rows, &empty_cols);
        assert_eq!(nrows, 2);
        assert_eq!(ncols, 1);
    }

    #[test]
    fn test_distance_with_expansion() {
        let p1 = Point(3, 0);
        let p2 = Point(4, 9);
        let empty_rows = vec![3, 7];
        let empty_cols = vec![2, 5, 8];
        let d = distance_with_expansion(&p1, &p2, &empty_rows, &empty_cols, 1);
        assert_eq!(d, 10);
        let d = distance_with_expansion(&p1, &p2, &empty_rows, &empty_cols, 10);
        assert_eq!(d, 28);

        let p1 = Point(0, 2);
        let p2 = Point(4, 9);
        let empty_rows = vec![3, 7];
        let empty_cols = vec![2, 5, 8];
        let d = distance_with_expansion(&p1, &p2, &empty_rows, &empty_cols, 1);
        assert_eq!(d, 11);
        let d = distance_with_expansion(&p1, &p2, &empty_rows, &empty_cols, 10);
        assert_eq!(d, 38);
        let d = distance_with_expansion(&p1, &p2, &empty_rows, &empty_cols, 100);
        assert_eq!(d, 308);
    }

    #[test]
    fn test_solve_p1() {
        let sum_of_distances = solve(&get_input(), 2);
        assert_eq!(sum_of_distances, 374);
    }

    #[test]
    fn test_solve_p2() {
        let sum_of_distances = solve(&get_input(), 10);
        assert_eq!(sum_of_distances, 1030);

        let sum_of_distances = solve(&get_input(), 100);
        assert_eq!(sum_of_distances, 8410);
    }

    #[test]
    fn test_full() {
        let input = fs::read_to_string("input.txt").unwrap();
        let part1 = solve(&input, 2);
        assert_eq!(part1, 9795148);
        let part2 = solve(&input, 1_000_000);
        assert_eq!(part2, 650672493820);
    }
}
