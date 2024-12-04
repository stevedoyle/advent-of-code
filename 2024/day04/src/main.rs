use pathfinding::matrix::Matrix;

fn parse_input(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|line| line.chars().collect::<Vec<_>>())).unwrap()
}

fn count_xmas(data: &Matrix<char>, start: (usize, usize)) -> i32 {
    let mut count = 0;

    let target = "XMAS".chars().collect::<Vec<_>>();
    let tlen = target.len();
    let (row, col) = start;

    let directions = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (1, 1),
        (1, -1),
        (-1, 0),
        (-1, -1),
        (-1, 1),
    ];

    for dir in directions {
        let mut found = true;
        for i in 0..tlen {
            let r = row as i32 + dir.0 * i as i32;
            let c = col as i32 + dir.1 * i as i32;
            if r < 0 || r >= data.rows as i32 || c < 0 || c >= data.columns as i32 {
                found = false;
                break;
            }
            if data[(r as usize, c as usize)] != target[i] {
                found = false;
                break;
            }
        }
        if found {
            count += 1;
        }
    }
    count
}

fn is_x_mas(data: &Matrix<char>, start: (usize, usize)) -> bool {
    let (row, col) = start;
    if row == 0 || row == data.rows - 1 || col == 0 || col == data.columns - 1 {
        return false;
    }

    let mut target = "MAS".chars().collect::<Vec<_>>();
    target.sort_unstable();
    let mut a = vec![
        data[(row - 1, col - 1)],
        data[(row, col)],
        data[(row + 1, col + 1)],
    ];
    a.sort_unstable();
    let mut b = vec![
        data[(row + 1, col - 1)],
        data[(row, col)],
        data[(row - 1, col + 1)],
    ];
    b.sort_unstable();

    target == a && target == b
}

fn solve_p1(input: &str) -> i32 {
    let data = parse_input(input);
    let mut count = 0;
    for row in 0..data.rows {
        for col in 0..data.columns {
            if data[(row, col)] == 'X' {
                count += count_xmas(&data, (row, col))
            }
        }
    }
    count
}

fn solve_p2(input: &str) -> i32 {
    let data = parse_input(input);
    let mut count = 0;
    for row in 0..data.rows {
        for col in 0..data.columns {
            if data[(row, col)] == 'A' {
                if is_x_mas(&data, (row, col)) {
                    count += 1;
                }
            }
        }
    }
    count
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

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 18);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 9);
    }
}
