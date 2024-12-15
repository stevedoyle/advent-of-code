use core::panic;

use pathfinding::matrix::Matrix;

fn parse_input(input: &str) -> (Matrix<char>, String) {
    let (grid_input, movement_input) = input.split_once("\n\n").unwrap();
    let grid = Matrix::from_rows(
        grid_input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>()),
    )
    .unwrap();
    (grid, movement_input.trim().to_string())
}

fn move_object(grid: &mut Matrix<char>, pos: &mut (usize, usize), dir: (i32, i32)) -> bool {
    let contents = grid[*pos];

    let (row, col) = *pos;
    let (new_row, new_col) = (row as i32 + dir.0, col as i32 + dir.1);
    let (new_row, new_col) = (new_row as usize, new_col as usize);

    if grid[(new_row, new_col)] == '#' {
        return false;
    }
    if grid[(new_row, new_col)] == '.'
        || (['O', '[', ']'].contains(&grid[(new_row, new_col)])
            && move_object(grid, &mut (new_row, new_col), dir))
    {
        grid[(row, col)] = '.';
        grid[(new_row, new_col)] = contents;
        *pos = (new_row, new_col);
        return true;
    }
    false
}

fn can_move(grid: &Matrix<char>, pos: (usize, usize), dir: (i32, i32)) -> bool {
    let (row, col) = pos;
    let (new_row, new_col) = (row as i32 + dir.0, col as i32 + dir.1);
    let (new_row, new_col) = (new_row as usize, new_col as usize);
    if grid[(new_row, new_col)] == '.' {
        return true;
    }
    if grid[(new_row, new_col)] == '#' {
        return false;
    }
    if grid[(new_row, new_col)] == '[' {
        return can_move(grid, (new_row, new_col), dir)
            && can_move(grid, (new_row, new_col + 1), dir);
    }
    if grid[(new_row, new_col)] == ']' {
        return can_move(grid, (new_row, new_col), dir)
            && can_move(grid, (new_row, new_col - 1), dir);
    }
    false
}

fn move_big_object_vertical(
    grid: &mut Matrix<char>,
    pos: &mut (usize, usize),
    dir: (i32, i32),
) -> bool {
    if !can_move(grid, *pos, dir) {
        return false;
    }

    let contents = grid[*pos];
    let (row, col) = *pos;
    let (new_row, new_col) = (row as i32 + dir.0, col as i32 + dir.1);
    let (new_row, new_col) = (new_row as usize, new_col as usize);
    if grid[(new_row, new_col)] == '.' {
        grid[(row, col)] = '.';
        grid[(new_row, new_col)] = contents;
        *pos = (new_row, new_col);
    } else if grid[(new_row, new_col)] == '[' {
        move_big_object_vertical(grid, &mut (new_row, new_col), dir);
        move_big_object_vertical(grid, &mut (new_row, new_col + 1), dir);
        grid[(row, col)] = '.';
        grid[(new_row, new_col)] = contents;
        *pos = (new_row, new_col);
    } else if grid[(new_row, new_col)] == ']' {
        move_big_object_vertical(grid, &mut (new_row, new_col), dir);
        move_big_object_vertical(grid, &mut (new_row, new_col - 1), dir);
        grid[(row, col)] = '.';
        grid[(new_row, new_col)] = contents;
        *pos = (new_row, new_col);
    }
    true
}

fn solve_p1(input: &str) -> usize {
    let (mut grid, movement) = parse_input(input);
    let robot = grid.values().position(|x| x == &'@').unwrap();
    let mut robot = (robot / grid.columns, robot % grid.columns);

    for dir in movement.chars() {
        if dir == '\n' {
            continue;
        }
        match dir {
            '>' => move_object(&mut grid, &mut robot, (0, 1)),
            '<' => move_object(&mut grid, &mut robot, (0, -1)),
            '^' => move_object(&mut grid, &mut robot, (-1, 0)),
            'v' => move_object(&mut grid, &mut robot, (1, 0)),
            _ => panic!("Invalid direction"),
        };
    }

    grid.items()
        .filter(|(_, &x)| x == 'O')
        .map(|((row, col), _)| (row * 100) + col)
        .sum()
}

fn solve_p2(input: &str) -> usize {
    let (grid, movement) = parse_input(input);
    let mut grid = resize_grid(&grid);
    let robot = grid.values().position(|x| x == &'@').unwrap();
    let mut robot = (robot / grid.columns, robot % grid.columns);

    for dir in movement.chars() {
        if dir == '\n' {
            continue;
        }
        match dir {
            '>' => move_object(&mut grid, &mut robot, (0, 1)),
            '<' => move_object(&mut grid, &mut robot, (0, -1)),
            '^' => move_big_object_vertical(&mut grid, &mut robot, (-1, 0)),
            'v' => move_big_object_vertical(&mut grid, &mut robot, (1, 0)),
            _ => panic!("Invalid direction"),
        };
    }

    grid.items()
        .filter(|(_, &x)| x == '[')
        .map(|((row, col), _)| (row * 100) + col)
        .sum()
}

fn resize_grid(grid: &Matrix<char>) -> Matrix<char> {
    let mut new_rows = Vec::new();
    for row in 0..grid.rows {
        let mut new_row: Vec<char> = Vec::new();
        for col in 0..grid.columns {
            match grid[(row, col)] {
                '.' => new_row.extend(['.', '.']),
                '#' => new_row.extend(['#', '#']),
                'O' => new_row.extend(['[', ']']),
                '@' => new_row.extend(['@', '.']),
                _ => panic!("Invalid character"),
            }
        }
        new_rows.push(new_row);
    }
    Matrix::from_rows(new_rows).unwrap()
}

#[allow(dead_code)]
fn display_grid(grid: &Matrix<char>) {
    for row in 0..grid.rows {
        for col in 0..grid.columns {
            print!("{}", grid[(row, col)]);
        }
        println!();
    }
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
        let input = std::fs::read_to_string("test_input_small.txt").unwrap();
        let answer = solve_p1(&input);
        assert_eq!(answer, 2028);
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let answer = solve_p1(&input);
        assert_eq!(answer, 10092);
        let answer = solve_p2(&input);
        assert_eq!(answer, 9021);
    }
}
