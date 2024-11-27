use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<(usize, usize)>> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let mut coords = point.split(",");
                    (
                        coords.next().unwrap().parse().unwrap(),
                        coords.next().unwrap().parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect()
}

fn fill_sand(grid: &mut [Vec<char>], source: (usize, usize)) -> i32 {
    let y_bound = grid.len() - 1;
    let mut count = 0;
    let mut done = false;

    loop {
        // One iteration of the outer loop per grain of sand
        let mut sand_pos = source;
        let curr_count = count;

        loop {
            if sand_pos.1 >= y_bound {
                break;
            }

            // Iterate until thee grain of sand comes to rest
            let down = (sand_pos.0, sand_pos.1 + 1);
            if grid[down.1][down.0] == '.' {
                sand_pos = down;
                continue;
            }

            let down_left = (sand_pos.0 - 1, sand_pos.1 + 1);
            if grid[down_left.1][down_left.0] == '.' {
                sand_pos = down_left;
                continue;
            }

            let down_right = (sand_pos.0 + 1, sand_pos.1 + 1);
            if grid[down_right.1][down_right.0] == '.' {
                sand_pos = down_right;
                continue;
            }

            // The grain of sand has come to rest
            grid[sand_pos.1][sand_pos.0] = 'o';
            count += 1;
            if sand_pos == source {
                done = true;
            }
            break;
        }
        if done || count == curr_count {
            break;
        }
    }

    count
}

fn solve_p1(input: &str) -> i32 {
    let paths = parse_input(input);
    let sand_source = (500, 0);
    let bottom_row = paths
        .iter()
        .map(|path| path.iter().map(|(_, y)| y).max().unwrap())
        .max()
        .unwrap();
    let right_col = paths
        .iter()
        .map(|path| path.iter().map(|(x, _)| x).max().unwrap())
        .max()
        .unwrap();

    let mut grid = vec![vec!['.'; right_col + 1]; bottom_row + 1];
    paths.iter().for_each(|path| {
        path.iter()
            .tuple_windows()
            .for_each(|((x1, y1), (x2, y2))| {
                for row in grid.iter_mut().take(*y2.max(y1) + 1).skip(*y1.min(y2)) {
                    for cell in row.iter_mut().take(*x2.max(x1) + 1).skip(*x1.min(x2)) {
                        *cell = '#';
                    }
                }
            });
    });
    fill_sand(&mut grid, sand_source)
}

fn solve_p2(input: &str) -> i32 {
    let paths = parse_input(input);
    let sand_source = (500, 0);
    let bottom_row = paths
        .iter()
        .map(|path| path.iter().map(|(_, y)| y).max().unwrap())
        .max()
        .unwrap()
        + 2;
    let right_col = paths
        .iter()
        .map(|path| path.iter().map(|(x, _)| x).max().unwrap())
        .max()
        .unwrap()
        + bottom_row
        + 1;

    let mut grid = vec![vec!['.'; right_col + 1]; bottom_row + 1];
    paths.iter().for_each(|path| {
        path.iter()
            .tuple_windows()
            .for_each(|((x1, y1), (x2, y2))| {
                for row in grid.iter_mut().take(*y2.max(y1) + 1).skip(*y1.min(y2)) {
                    for cell in row.iter_mut().take(*x2.max(x1) + 1).skip(*x1.min(x2)) {
                        *cell = '#';
                    }
                }
            });
    });
    for x in 0..grid[0].len() {
        grid[bottom_row][x] = '#';
    }
    fill_sand(&mut grid, sand_source)
}

fn main() {
    let input = include_str!("../input.txt");
    let answer = solve_p1(input);
    println!("Part 1: {answer}");
    let answer = solve_p2(input);
    println!("Part 2: {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 24);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 93);
    }
}
