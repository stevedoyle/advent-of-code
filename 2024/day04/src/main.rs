use pathfinding::matrix::Matrix;

fn parse_input(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|line| line.chars().collect::<Vec<_>>())).unwrap()
}

fn count_xmas(data: &Matrix<char>, start: (usize, usize)) -> usize {
    let mut count = 0;

    let target = "XMAS".chars().collect::<Vec<_>>();
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
        for (i, &item) in target.iter().enumerate() {
            let r = row as i32 + dir.0 * i as i32;
            let c = col as i32 + dir.1 * i as i32;
            if r < 0 || r >= data.rows as i32 || c < 0 || c >= data.columns as i32 {
                found = false;
                break;
            }
            if data[(r as usize, c as usize)] != item {
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

fn solve_p1(data: &Matrix<char>) -> usize {
    data.items()
        .filter(|(_, &ch)| ch == 'X')
        .map(|(pos, _)| count_xmas(data, pos))
        .sum()
}

fn solve_p2(data: &Matrix<char>) -> usize {
    data.items()
        .filter(|((r, c), &ch)| ch == 'A' && is_x_mas(data, (*r, *c)))
        .count()
}

fn main() {
    let input = include_str!("../input.txt");
    let data = parse_input(input);

    let start = std::time::Instant::now();
    let answer = solve_p1(&data);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed time: {elapsed:.2?}");

    let start = std::time::Instant::now();
    let answer = solve_p2(&data);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, elapsed time: {elapsed:.2?}");
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
        let data = parse_input(INPUT);
        let answer = solve_p1(&data);
        assert_eq!(answer, 18);
        let answer = solve_p2(&data);
        assert_eq!(answer, 9);
    }
}
