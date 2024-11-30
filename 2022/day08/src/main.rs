use pathfinding::prelude::Matrix;

fn parse_input(input: &str) -> Matrix<u32> {
    Matrix::from_rows(input.lines().map(|line| {
        line.trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>()
    }))
    .unwrap()
}

fn solve_p1(input: &str) -> usize {
    let grid = parse_input(input);
    let nrows = grid.rows;
    let ncols = grid.columns;

    let mut visible = Matrix::new(nrows, ncols, 0);

    let mut count = (nrows * 2) + (ncols * 2) - 4;

    for row in 1..nrows - 1 {
        let mut max_from_left = grid[(row, 0)];
        for col in 1..ncols - 1 {
            let curr = grid[(row, col)];
            if curr > max_from_left {
                max_from_left = curr;
                visible[(row, col)] = 1;
                count += 1;
            }
        }

        let mut max_from_right = grid[(row, ncols - 1)];
        for col in (1..ncols - 1).rev() {
            let curr = grid[(row, col)];
            if curr > max_from_right {
                max_from_right = curr;
                if visible[(row, col)] != 1 {
                    visible[(row, col)] = 1;
                    count += 1;
                }
            }
        }
    }

    for col in 1..ncols - 1 {
        let mut max_from_top = grid[(0, col)];
        for row in 1..nrows - 1 {
            let curr = grid[(row, col)];
            if curr > max_from_top {
                max_from_top = curr;
                if visible[(row, col)] != 1 {
                    visible[(row, col)] = 1;
                    count += 1;
                }
            }
        }

        let mut max_from_bottom = grid[(nrows - 1, col)];
        for row in (1..nrows - 1).rev() {
            let curr = grid[(row, col)];
            if curr > max_from_bottom {
                max_from_bottom = curr;
                if visible[(row, col)] != 1 {
                    visible[(row, col)] = 1;
                    count += 1;
                }
            }
        }
    }
    count
}

fn solve_p2(input: &str) -> usize {
    let grid = parse_input(input);
    let nrows = grid.rows;
    let ncols = grid.columns;

    let mut max_score = 0;

    for row in 0..nrows {
        for col in 0..ncols {
            let score = calc_score(&grid, (row, col));
            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}

fn calc_score(grid: &Matrix<u32>, (row, col): (usize, usize)) -> usize {
    let nrows = grid.rows;
    let ncols = grid.columns;

    let mut score = 0;
    let curr = grid[(row, col)];

    for i in (0..row).rev() {
        score += 1;
        if grid[(i, col)] >= curr {
            break;
        }
    }

    let mut scenic_score = score;
    score = 0;

    for i in (row + 1)..nrows {
        score += 1;
        if grid[(i, col)] >= curr {
            break;
        }
    }

    scenic_score *= score;
    score = 0;

    for i in (0..col).rev() {
        score += 1;
        if grid[(row, i)] >= curr {
            break;
        }
    }

    scenic_score *= score;
    score = 0;

    for i in (col + 1)..ncols {
        score += 1;
        if grid[(row, i)] >= curr {
            break;
        }
    }

    scenic_score *= score;

    scenic_score
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

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 21);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 8);
    }
}
