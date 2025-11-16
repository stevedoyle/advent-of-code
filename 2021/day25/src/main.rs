use pathfinding::matrix::Matrix;

fn parse_input(input: &str) -> Matrix<char> {
    Matrix::from_rows(
        input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    )
    .unwrap()
}

fn solve_p1(input: &str) -> usize {
    let mut grid = parse_input(input);
    let rows = grid.rows;
    let cols = grid.columns;
    let mut step = 0;

    loop {
        step += 1;
        let mut moved = false;

        // Move east-facing sea cucumbers (>)
        let mut new_grid = grid.clone();
        for r in 0..rows {
            for c in 0..cols {
                if grid[(r, c)] == '>' {
                    let next_c = (c + 1) % cols;
                    if grid[(r, next_c)] == '.' {
                        new_grid[(r, c)] = '.';
                        new_grid[(r, next_c)] = '>';
                        moved = true;
                    }
                }
            }
        }
        grid = new_grid;

        // Move south-facing sea cucumbers (v)
        let mut new_grid = grid.clone();
        for r in 0..rows {
            for c in 0..cols {
                if grid[(r, c)] == 'v' {
                    let next_r = (r + 1) % rows;
                    if grid[(next_r, c)] == '.' {
                        new_grid[(r, c)] = '.';
                        new_grid[(next_r, c)] = 'v';
                        moved = true;
                    }
                }
            }
        }
        grid = new_grid;

        if !moved {
            break;
        }
    }

    step
}

fn solve_p2(_input: &str) -> &str {
    // Day 25 Part 2 is always a freebie - no implementation needed
    "Merry Christmas!"
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
        assert_eq!(answer, 58);
    }
}
