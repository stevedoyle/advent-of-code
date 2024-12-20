use pathfinding::matrix::Matrix;
use regex::Regex;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Step {
    Forward(usize),
    TurnLeft,
    TurnRight,
}

#[derive(Debug)]
struct Robot {
    pos: (usize, usize),
    facing: Direction,
}

impl Robot {
    fn new(pos: (usize, usize), facing: Direction) -> Self {
        Self { pos, facing }
    }
}

fn parse_input(input: &str) -> (Matrix<char>, Vec<Step>) {
    let (map, path) = input.split_once("\n\n").unwrap();
    let map_rows: Vec<Vec<char>> = map
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let max_cols = map_rows.iter().map(|row| row.len()).max().unwrap();
    // Extend rows to max_cols length filling with ' '
    let map_rows: Vec<Vec<char>> = map_rows
        .iter()
        .map(|row| {
            let mut row = row.clone();
            row.resize(max_cols, ' ');
            row
        })
        .collect();
    let map = Matrix::from_rows(map_rows).unwrap();

    // Parse the numbers and L or R from "10R5L5R10L4R5L5" into Vec<Step>
    let pattern = Regex::new(r"(\d+|[LR])").unwrap();
    let path = pattern
        .find_iter(path)
        .map(|mat| match mat.as_str() {
            "L" => Step::TurnLeft,
            "R" => Step::TurnRight,
            num => Step::Forward(num.parse().unwrap()),
        })
        .collect();

    (map, path)
}

#[allow(dead_code)]
fn print_map(map: &Matrix<char>) {
    for row in map.iter() {
        for ch in row {
            print!("{}", ch);
        }
        println!();
    }
}

fn wrap_right(map: &Matrix<char>, pos: (usize, usize)) -> (usize, usize) {
    let (row, _) = pos;
    let map_row = map.iter().nth(row).unwrap();
    let first_non_space_col = map_row.iter().position(|x| *x != ' ').unwrap();
    let first_non_space = (row, first_non_space_col);
    if map[first_non_space] == '.' {
        return first_non_space;
    }
    pos
}

fn wrap_left(map: &Matrix<char>, pos: (usize, usize)) -> (usize, usize) {
    let (row, _) = pos;
    let map_row = map.iter().nth(row).unwrap();
    let last_non_space_col = map_row.iter().rposition(|x| *x != ' ').unwrap();
    let last_non_space = (row, last_non_space_col);
    if map[last_non_space] == '.' {
        return last_non_space;
    }
    pos
}

fn wrap_up(map: &Matrix<char>, pos: (usize, usize)) -> (usize, usize) {
    let (_, col) = pos;
    let map_col = map.column_iter().nth(col).unwrap();
    let first_non_space_row = map_col.iter().rposition(|&x| *x != ' ').unwrap();
    let first_non_space = (first_non_space_row, col);
    if map[first_non_space] == '.' {
        return first_non_space;
    }
    pos
}

fn wrap_down(map: &Matrix<char>, pos: (usize, usize)) -> (usize, usize) {
    let (_, col) = pos;
    let map_col = map.column_iter().nth(col).unwrap();
    let last_non_space_row = map_col.iter().position(|&x| *x != ' ').unwrap();
    let last_non_space = (last_non_space_row, col);
    if map[last_non_space] == '.' {
        return last_non_space;
    }
    pos
}

fn move_left(grid: &Matrix<char>, robot: &mut Robot) {
    let (r, c) = robot.pos;
    if c > 0 {
        match grid[(r, c - 1)] {
            '.' => robot.pos = (r, c - 1),
            '#' => (),
            ' ' => robot.pos = wrap_left(grid, robot.pos),
            _ => panic!("Invalid grid character"),
        }
    } else {
        robot.pos = wrap_left(grid, robot.pos);
    }
}

fn move_right(grid: &Matrix<char>, robot: &mut Robot) {
    let (r, c) = robot.pos;
    if c + 1 < grid.columns {
        match grid[(r, c + 1)] {
            '.' => robot.pos = (r, c + 1),
            '#' => (),
            ' ' => robot.pos = wrap_right(grid, robot.pos),
            _ => panic!("Invalid grid character"),
        }
    } else {
        robot.pos = wrap_right(grid, robot.pos);
    }
}

fn move_up(grid: &Matrix<char>, robot: &mut Robot) {
    let (r, c) = robot.pos;
    if r > 0 {
        match grid[(r - 1, c)] {
            '.' => robot.pos = (r - 1, c),
            '#' => (),
            ' ' => {
                let (r, c) = wrap_up(grid, robot.pos);
                robot.pos = (r, c);
            }
            _ => panic!("Invalid grid character"),
        }
    } else {
        let (r, c) = wrap_up(grid, robot.pos);
        robot.pos = (r, c);
    }
}

fn move_down(grid: &Matrix<char>, robot: &mut Robot) {
    let (r, c) = robot.pos;
    if r + 1 < grid.rows {
        match grid[(r + 1, c)] {
            '.' => robot.pos = (r + 1, c),
            '#' => (),
            ' ' => {
                let (r, c) = wrap_down(grid, robot.pos);
                robot.pos = (r, c);
            }
            _ => panic!("Invalid grid character"),
        }
    } else {
        let (r, c) = wrap_down(grid, robot.pos);
        robot.pos = (r, c);
    }
}

fn move_forward(map: &Matrix<char>, robot: &mut Robot, n: usize) {
    for _ in 0..n {
        match robot.facing {
            Direction::Right => move_right(map, robot),
            Direction::Left => move_left(map, robot),
            Direction::Up => move_up(map, robot),
            Direction::Down => move_down(map, robot),
        }
    }
}

fn turn_left(robot: &mut Robot) {
    robot.facing = match robot.facing {
        Direction::Right => Direction::Up,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
        Direction::Up => Direction::Left,
    }
}

fn turn_right(robot: &mut Robot) {
    robot.facing = match robot.facing {
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Up => Direction::Right,
    }
}

fn move_robot(map: &Matrix<char>, robot: &mut Robot, step: Step) {
    match step {
        Step::Forward(n) => move_forward(map, robot, n),
        Step::TurnLeft => turn_left(robot),
        Step::TurnRight => turn_right(robot),
    }
}

fn cube_face(pos: (usize, usize), bounds: (usize, usize)) -> (usize, (usize, usize)) {
    let group_size = bounds.0 / 3;
    let row_group = pos.0 / group_size;
    let row_group_offset = pos.0 % group_size;
    let col_group = pos.1 / group_size;
    let col_group_offset = pos.1 % group_size;
    match (row_group, col_group) {
        (0, _) => (1, (row_group_offset, col_group_offset)),
        (1, 0) => (2, (row_group_offset, col_group_offset)),
        (1, 1) => (3, (row_group_offset, col_group_offset)),
        (1, 2) => (4, (row_group_offset, col_group_offset)),
        (2, 2) => (5, (row_group_offset, col_group_offset)),
        (2, 3) => (6, (row_group_offset, col_group_offset)),
        _ => panic!("Invalid cube face"),
    }
}

fn wrap_right_cube(map: &Matrix<char>, pos: (usize, usize)) -> (usize, usize) {
    let (row, _) = pos;
    let cube_face = cube_face(pos, (map.rows / 3, map.columns / 4));
    let (next_face, dir, (new_grp_row, new_grp_col)) = match cube_face.0 {
        1 => (6, Direction::Left, (0, map.columns - cube_face.1.0 - 1)),
        4 => (6, Direction::Down, (0, map.columns - cube_face.1.0 - 1)),
        6 => (1, Direction::Left, (0, 0)),
        _ => panic!("Invalid cube face"),
    };
    let map_row = map.iter().nth(row).unwrap();
    let first_non_space_col = map_row.iter().position(|x| *x != ' ').unwrap();
    let first_non_space = (row, first_non_space_col);
    if map[first_non_space] == '.' {
        return first_non_space;
    }
    pos
}

fn wrap_left_cube(map: &Matrix<char>, pos: (usize, usize)) -> (usize, usize) {
    let (row, _) = pos;
    let map_row = map.iter().nth(row).unwrap();
    let last_non_space_col = map_row.iter().rposition(|x| *x != ' ').unwrap();
    let last_non_space = (row, last_non_space_col);
    if map[last_non_space] == '.' {
        return last_non_space;
    }
    pos
}

fn wrap_up_cube(map: &Matrix<char>, pos: (usize, usize)) -> (usize, usize) {
    let (_, col) = pos;
    let map_col = map.column_iter().nth(col).unwrap();
    let first_non_space_row = map_col.iter().rposition(|&x| *x != ' ').unwrap();
    let first_non_space = (first_non_space_row, col);
    if map[first_non_space] == '.' {
        return first_non_space;
    }
    pos
}

fn wrap_down_cube(map: &Matrix<char>, pos: (usize, usize)) -> (usize, usize) {
    let (_, col) = pos;
    let map_col = map.column_iter().nth(col).unwrap();
    let last_non_space_row = map_col.iter().position(|&x| *x != ' ').unwrap();
    let last_non_space = (last_non_space_row, col);
    if map[last_non_space] == '.' {
        return last_non_space;
    }
    pos
}

fn move_left_cube(grid: &Matrix<char>, robot: &mut Robot) {
    let (r, c) = robot.pos;
    if c > 0 {
        match grid[(r, c - 1)] {
            '.' => robot.pos = (r, c - 1),
            '#' => (),
            ' ' => robot.pos = wrap_left_cube(grid, robot.pos),
            _ => panic!("Invalid grid character"),
        }
    } else {
        robot.pos = wrap_left_cube(grid, robot.pos);
    }
}

fn move_right_cube(grid: &Matrix<char>, robot: &mut Robot) {
    let (r, c) = robot.pos;
    if c + 1 < grid.columns {
        match grid[(r, c + 1)] {
            '.' => robot.pos = (r, c + 1),
            '#' => (),
            ' ' => robot.pos = wrap_right(grid, robot.pos),
            _ => panic!("Invalid grid character"),
        }
    } else {
        robot.pos = wrap_right_cube(grid, robot.pos);
    }
}

fn move_up_cube(grid: &Matrix<char>, robot: &mut Robot) {
    let (r, c) = robot.pos;
    if r > 0 {
        match grid[(r - 1, c)] {
            '.' => robot.pos = (r - 1, c),
            '#' => (),
            ' ' => {
                let (r, c) = wrap_up_cube(grid, robot.pos);
                robot.pos = (r, c);
            }
            _ => panic!("Invalid grid character"),
        }
    } else {
        let (r, c) = wrap_up_cube(grid, robot.pos);
        robot.pos = (r, c);
    }
}

fn move_down_cube(grid: &Matrix<char>, robot: &mut Robot) {
    let (r, c) = robot.pos;
    if r + 1 < grid.rows {
        match grid[(r + 1, c)] {
            '.' => robot.pos = (r + 1, c),
            '#' => (),
            ' ' => {
                let (r, c) = wrap_down_cube(grid, robot.pos);
                robot.pos = (r, c);
            }
            _ => panic!("Invalid grid character"),
        }
    } else {
        let (r, c) = wrap_down_cube(grid, robot.pos);
        robot.pos = (r, c);
    }
}

fn move_forward_cube(map: &Matrix<char>, robot: &mut Robot, n: usize) {
    for _ in 0..n {
        match robot.facing {
            Direction::Right => move_right_cube(map, robot),
            Direction::Left => move_left_cube(map, robot),
            Direction::Up => move_up_cube(map, robot),
            Direction::Down => move_down_cube(map, robot),
        }
    }
}

fn move_robot_on_cube(map: &Matrix<char>, robot: &mut Robot, step: Step) {
    match step {
        Step::Forward(n) => move_forward_cube(map, robot, n),
        Step::TurnLeft => turn_left(robot),
        Step::TurnRight => turn_right(robot),
    }
}

fn facing_to_num(facing: &Direction) -> usize {
    match facing {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    }
}
fn calc_password(robot: &Robot) -> usize {
    1000 * (robot.pos.0 + 1) + 4 * (robot.pos.1 + 1) + facing_to_num(&robot.facing)
}

fn solve_p1(input: &str) -> usize {
    let (map, path) = parse_input(input);
    let start = map.items().find(|(_, &ch)| ch == '.').unwrap().0;
    let mut robot = Robot::new(start, Direction::Right);
    for step in path {
        move_robot(&map, &mut robot, step);
    }
    calc_password(&robot)
}

fn solve_p2(input: &str) -> usize {
    let (map, path) = parse_input(input);
    // print_map(&map);
    // println!("{:?}", path);
    let start = map.items().find(|(_, &ch)| ch == '.').unwrap().0;
    // println!("start = {:?}", start);
    let mut robot = Robot::new(start, Direction::Right);
    for step in path {
        move_robot_on_cube(&map, &mut robot, step);
        // println!("Step: {:?} -> {:?}", step, robot);
    }
    // println!("{:?}", robot);
    calc_password(&robot)
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
        assert_eq!(answer, 6032);
        let answer = solve_p2(&input);
        assert_eq!(answer, 5031);
    }
}
