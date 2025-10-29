use std::collections::HashMap;

struct Point {
    x: i32,
    y: i32,
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn get_steps(&self) -> (i32, i32) {
        let x_step = if self.end.x > self.start.x {
            1
        } else if self.end.x < self.start.x {
            -1
        } else {
            0
        };
        let y_step = if self.end.y > self.start.y {
            1
        } else if self.end.y < self.start.y {
            -1
        } else {
            0
        };
        (x_step, y_step)
    }

    fn length(&self) -> i32 {
        std::cmp::max(
            (self.end.x - self.start.x).abs(),
            (self.end.y - self.start.y).abs(),
        )
    }
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once(" -> ").unwrap();
            let start = parse_point(start);
            let end = parse_point(end);
            Line { start, end }
        })
        .collect()
}

fn parse_point(input: &str) -> Point {
    let (x, y) = input.split_once(",").unwrap();
    Point {
        x: x.parse().unwrap(),
        y: y.parse().unwrap(),
    }
}

fn is_horizontal_or_vertical(line: &Line) -> bool {
    is_horizontal(line) || is_vertical(line)
}

fn is_horizontal(line: &Line) -> bool {
    line.start.y == line.end.y
}

fn is_vertical(line: &Line) -> bool {
    line.start.x == line.end.x
}

fn solve_p1(input: &str) -> i32 {
    let lines = parse_input(input);
    // Build a grid and count overlaps
    let mut grid = HashMap::new();
    for line in lines.iter() {
        if is_horizontal_or_vertical(line) {
            let (x_step, y_step) = line.get_steps();
            let length = line.length();
            for i in 0..=length {
                let x = line.start.x + i * x_step;
                let y = line.start.y + i * y_step;
                *grid.entry((x, y)).or_insert(0) += 1;
            }
        }
    }
    grid.values().filter(|&&count| count > 1).count() as i32
}

fn solve_p2(input: &str) -> i32 {
    let lines = parse_input(input);
    // Build a grid and count overlaps for all lines - horizontal, vertical, and diagonal
    let mut grid = HashMap::new();
    for line in lines.iter() {
        let (x_step, y_step) = line.get_steps();
        let length = line.length();
        for i in 0..=length {
            let x = line.start.x + i * x_step;
            let y = line.start.y + i * y_step;
            *grid.entry((x, y)).or_insert(0) += 1;
        }
    }
    grid.values().filter(|&&count| count > 1).count() as i32
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

    const TEST_INPUT: &str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_solve_with_test_input() {
        let input = TEST_INPUT;
        let answer = solve_p1(&input);
        assert_eq!(answer, 5);
        let answer = solve_p2(&input);
        assert_eq!(answer, 12);
    }
}
