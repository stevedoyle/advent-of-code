use core::str;

struct Game {
    numbers: Vec<i32>,
    boards: Vec<Board>,
}

struct Board {
    // Add fields as necessary
    numbers: Vec<Vec<i32>>,
    marked: Vec<Vec<bool>>,
}

fn parse_input(input: &str) -> Game {
    let mut lines = input.lines();
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut boards = Vec::new();
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            continue;
        }
        let mut board = Board {
            numbers: Vec::new(),
            marked: vec![vec![false; 5]; 5],
        };
        board.numbers.push(
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        );
        for _ in 0..4 {
            let line = lines.next().unwrap();
            board.numbers.push(
                line.split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect(),
            );
        }
        boards.push(board);
    }
    Game { numbers, boards }
}

fn winning_condition_met(board: &Board) -> bool {
    // Check rows
    for i in 0..5 {
        if board.marked[i].iter().all(|&marked| marked) {
            return true;
        }
    }
    // Check columns
    for j in 0..5 {
        if (0..5).all(|i| board.marked[i][j]) {
            return true;
        }
    }
    false
}

fn get_unmarked_sum(board: &Board) -> i32 {
    board
        .numbers
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(j, _)| !board.marked[i][*j])
                .map(|(_, &num)| num)
                .sum::<i32>()
        })
        .sum()
}

fn solve_p1(input: &str) -> i32 {
    let mut game = parse_input(input);
    for &number in &game.numbers {
        for board in &mut game.boards {
            for i in 0..5 {
                for j in 0..5 {
                    if board.numbers[i][j] == number {
                        board.marked[i][j] = true;
                    }
                }
            }
            // Check for win condition here
            if winning_condition_met(board) {
                let unmarked_sum = get_unmarked_sum(board);
                return unmarked_sum * number;
            }
        }
    }
    0
}

fn solve_p2(input: &str) -> i32 {
    let mut game = parse_input(input);
    let mut winning_boards = vec![false; game.boards.len()];
    let mut last_score = 0;
    for &number in &game.numbers {
        for (b_idx, board) in game.boards.iter_mut().enumerate() {
            if winning_boards[b_idx] {
                continue;
            }
            for i in 0..5 {
                for j in 0..5 {
                    if board.numbers[i][j] == number {
                        board.marked[i][j] = true;
                    }
                }
            }
            if winning_condition_met(board) {
                winning_boards[b_idx] = true;
                let unmarked_sum = get_unmarked_sum(board);
                last_score = unmarked_sum * number;
            }
        }
    }
    last_score
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
        assert_eq!(answer, 4512);
        let answer = solve_p2(&input);
        assert_eq!(answer, 1924);
    }
}
