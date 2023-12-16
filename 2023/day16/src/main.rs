use anyhow::Result;
use grid::Grid;
use std::{collections::HashSet, fs};

fn parse_input(input: &str) -> Grid<char> {
    let mut grid = Grid::new(0, 0);

    for line in input.lines() {
        grid.push_row(line.trim().chars().collect());
    }
    grid
}

fn travel(grid: &Grid<char>, curr_pos: &Point, dir: Direction) -> (Option<Move>, Option<Move>) {
    let c = grid.get(curr_pos.1, curr_pos.0).unwrap();
    match c {
        '.' => {
            if let Some(pos) = get_next_pos(grid, curr_pos, dir) {
                (Some(Move(pos, dir)), None)
            } else {
                (None, None)
            }
        }
        '-' => horizontal_splitter(grid, curr_pos, dir),
        '|' => vertical_splitter(grid, curr_pos, dir),
        '/' | '\\' => mirror(grid, *c, curr_pos, dir),
        _ => panic!("Unrecognized symbol {c}"),
    }
}

fn horizontal_splitter(
    grid: &Grid<char>,
    pos: &Point,
    dir: Direction,
) -> (Option<Move>, Option<Move>) {
    match dir {
        Direction::Left | Direction::Right => {
            if let Some(next_pos) = get_next_pos(grid, pos, dir) {
                (Some(Move(next_pos, dir)), None)
            } else {
                (None, None)
            }
        }
        Direction::Up | Direction::Down => {
            let a = if let Some(next_pos) = get_next_pos(grid, pos, Direction::Left) {
                Some(Move(next_pos, Direction::Left))
            } else {
                None
            };
            let b = if let Some(next_pos) = get_next_pos(grid, pos, Direction::Right) {
                Some(Move(next_pos, Direction::Right))
            } else {
                None
            };
            (a, b)
        }
    }
}

fn vertical_splitter(
    grid: &Grid<char>,
    pos: &Point,
    dir: Direction,
) -> (Option<Move>, Option<Move>) {
    // println!("vertical_splitter(): {pos:?}, {dir:?}");
    match dir {
        Direction::Up | Direction::Down => {
            if let Some(next_pos) = get_next_pos(grid, pos, dir) {
                (Some(Move(next_pos, dir)), None)
            } else {
                (None, None)
            }
        }
        Direction::Left | Direction::Right => {
            let a = get_next_pos(grid, pos, Direction::Up)
                .map(|next_pos| Move(next_pos, Direction::Up));
            let b = get_next_pos(grid, pos, Direction::Down)
                .map(|next_pos| Move(next_pos, Direction::Down));
            // let a = if let Some(next_pos) = get_next_pos(grid, pos, Direction::Up) {
            //     Some(Move(next_pos, Direction::Up))
            // } else {
            //     None
            // };
            // let b = if let Some(next_pos) = get_next_pos(grid, pos, Direction::Down) {
            //     Some(Move(next_pos, Direction::Down))
            // } else {
            //     None
            // };
            (a, b)
        }
    }
}

fn mirror(grid: &Grid<char>, m: char, pos: &Point, dir: Direction) -> (Option<Move>, Option<Move>) {
    // println!("mirror(): {pos:?}, {dir:?}");
    let new_dir = match m {
        '/' => match dir {
            Direction::Right => Direction::Up,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        },
        '\\' => match dir {
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
        },
        _ => panic!("Unexpected mirror symbol {m}"),
    };
    if let Some(next_pos) = get_next_pos(grid, pos, new_dir) {
        (Some(Move(next_pos, new_dir)), None)
    } else {
        (None, None)
    }
}

fn get_next_pos(grid: &Grid<char>, curr_pos: &Point, dir: Direction) -> Option<Point> {
    // println!("get_next_pos(): {curr_pos:?}, {dir:?}");
    match dir {
        Direction::Up => match curr_pos {
            Point(_, 0) => None,
            _ => Some(Point(curr_pos.0, curr_pos.1 - 1)),
        },
        Direction::Down => {
            if curr_pos.1 == grid.rows() - 1 {
                None
            } else {
                Some(Point(curr_pos.0, curr_pos.1 + 1))
            }
        }
        Direction::Left => match curr_pos {
            Point(0, _) => None,
            _ => Some(Point(curr_pos.0 - 1, curr_pos.1)),
        },
        Direction::Right => {
            if curr_pos.0 == grid.cols() - 1 {
                None
            } else {
                Some(Point(curr_pos.0 + 1, curr_pos.1))
            }
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct Point(usize, usize);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Move(Point, Direction);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_and_track(grid: &Grid<char>, tracker: &mut Grid<char>, m: Move, cache: &mut HashSet<Move>) {
    let (pos, dir) = (m.0, m.1);
    *tracker.get_mut(pos.1, pos.0).unwrap() = '#';
    let moves = travel(grid, &pos, dir);
    if let Some(m) = moves.0 {
        if cache.contains(&m) {
            return;
        }
        cache.insert(m);
        move_and_track(grid, tracker, m, cache);
    }
    if let Some(m) = moves.1 {
        if cache.contains(&m) {
            return;
        }
        cache.insert(m);
        move_and_track(grid, tracker, m, cache);
    }
}

fn calc_energized(grid: &Grid<char>, start: &Point, dir: Direction) -> usize {
    let mut tracker: Grid<char> = Grid::new(grid.rows(), grid.cols());

    let mut cache: HashSet<Move> = HashSet::new();
    cache.insert(Move(*start, dir));

    move_and_track(grid, &mut tracker, Move(*start, dir), &mut cache);

    tracker.iter().filter(|&c| *c == '#').count()
}

fn max_energized(grid: &Grid<char>) -> usize {
    let m1 = (0..grid.cols())
        .map(|col| calc_energized(grid, &Point(col, 0), Direction::Down))
        .max()
        .unwrap();
    let m2 = (0..grid.cols())
        .map(|col| calc_energized(grid, &Point(col, grid.rows() - 1), Direction::Up))
        .max()
        .unwrap();
    let m3 = (0..grid.rows())
        .map(|row| calc_energized(grid, &Point(0, row), Direction::Right))
        .max()
        .unwrap();
    let m4 = (0..grid.cols())
        .map(|row| calc_energized(grid, &Point(grid.cols() - 1, row), Direction::Left))
        .max()
        .unwrap();

    *[m1, m2, m3, m4].iter().max().unwrap()
}

fn solve_p1(input: &str) -> usize {
    let grid = parse_input(input);
    calc_energized(&grid, &Point(0, 0), Direction::Right)
}

fn solve_p2(input: &str) -> usize {
    let grid = parse_input(input);
    max_energized(&grid)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let answer = solve_p1(&input);
    println!("Part 1: {answer}");
    let answer = solve_p2(&input);
    println!("Part 2: {answer}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        String::from(
            r".|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....",
        )
    }

    #[test]
    fn test_grid_next_pos() {
        let grid = parse_input(&get_test_input());
        dbg!(&grid);

        assert_eq!(
            get_next_pos(&grid, &Point(0, 0), Direction::Right),
            Some(Point(1, 0))
        );
        assert_eq!(get_next_pos(&grid, &Point(0, 1), Direction::Left), None);
        assert_eq!(
            get_next_pos(&grid, &Point(1, 1), Direction::Left),
            Some(Point(0, 1))
        );
        assert_eq!(get_next_pos(&grid, &Point(1, 0), Direction::Up), None);
        assert_eq!(
            get_next_pos(&grid, &Point(1, 1), Direction::Up),
            Some(Point(1, 0))
        );
        assert_eq!(
            get_next_pos(&grid, &Point(1, 1), Direction::Down),
            Some(Point(1, 2))
        );
        assert_eq!(
            get_next_pos(&grid, &Point(1, grid.rows() - 1), Direction::Down),
            None
        );
        assert_eq!(
            get_next_pos(&grid, &Point(0, 1), Direction::Up),
            Some(Point(0, 0))
        );
        assert_eq!(
            get_next_pos(&grid, &Point(0, 1), Direction::Down),
            Some(Point(0, 2))
        );
    }

    #[test]
    fn test_travel() {
        let grid = parse_input(&get_test_input());
        assert_eq!(*grid.get(0, 0).unwrap(), '.');
        let moves = travel(&grid, &Point(0, 0), Direction::Right);
        assert_eq!(moves.0, Some(Move(Point(1, 0), Direction::Right)));
        assert_eq!(moves.1, None);

        assert_eq!(*grid.get(0, 1).unwrap(), '|');
        let moves = travel(&grid, &Point(1, 0), Direction::Right);
        assert_eq!(moves.0, None);
        assert_eq!(moves.1, Some(Move(Point(1, 1), Direction::Down)));

        assert_eq!(*grid.get(1, 0).unwrap(), '|');
        let moves = travel(&grid, &Point(0, 1), Direction::Right);
        assert_eq!(moves.0, Some(Move(Point(0, 0), Direction::Up)));
        assert_eq!(moves.1, Some(Move(Point(0, 2), Direction::Down)));

        dbg!(&grid);
        assert_eq!(*grid.get(0, 5).unwrap(), '\\');
        let moves = travel(&grid, &Point(5, 0), Direction::Right);
        assert_eq!(moves.0, Some(Move(Point(5, 1), Direction::Down)));
        assert_eq!(moves.1, None);
    }

    #[test]
    fn solve_with_test_input() {
        let input = get_test_input();
        let answer = solve_p1(&input);
        assert_eq!(answer, 46);
        let answer = solve_p2(&input);
        assert_eq!(answer, 51);
    }

    #[test]
    fn solve_with_real_input() {
        let input = fs::read_to_string("input.txt").unwrap();
        let answer = solve_p1(&input);
        assert_eq!(answer, 7996);
        let answer = solve_p2(&input);
        assert_eq!(answer, 8239);
    }
}
