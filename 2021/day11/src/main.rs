use pathfinding::matrix::Matrix;

fn parse_input(input: &str) -> Matrix<usize> {
    Matrix::from_rows(input.lines().map(|line| {
        line.trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>()
    }))
    .unwrap()
}

fn process(octos: &mut Matrix<usize>) -> usize {
    let mut flashes = 0;
    let rows = octos.rows;
    let cols = octos.columns;
    let mut to_flash = Vec::new();

    for idx in octos.keys() {
        octos[idx] += 1;
        if octos[idx] > 9 {
            to_flash.push(idx);
        }
    }

    let mut flashed = Matrix::new(rows, cols, false);
    while let Some((r, c)) = to_flash.pop() {
        if flashed[(r, c)] {
            continue;
        }
        flashed[(r, c)] = true;
        flashes += 1;

        let neighbours = octos.neighbours((r, c), true);
        for (nr, nc) in neighbours {
            octos[(nr, nc)] += 1;
            if octos[(nr, nc)] > 9 && !flashed[(nr, nc)] {
                to_flash.push((nr, nc));
            }
        }
    }

    for idx in flashed.keys() {
        if flashed[idx] {
            octos[idx] = 0;
        }
    }

    flashes
}

fn solve_p1(input: &str) -> usize {
    let mut octos = parse_input(input);
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += process(&mut octos);
    }
    flashes
}

fn solve_p2(input: &str) -> usize {
    let mut octos = parse_input(input);
    let mut flashes = 0;
    let mut step = 0;
    let total_octos = octos.rows * octos.columns;
    while flashes < total_octos {
        flashes = process(&mut octos);
        step += 1;
    }
    step
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
        assert_eq!(answer, 1656);
        let answer = solve_p2(&input);
        assert_eq!(answer, 195);
    }
}
