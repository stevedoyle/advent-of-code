use core::str;
use std::collections::HashSet;

struct FoldInstruction {
    axis: String,
    value: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

fn parse_input(input: &str) -> (Vec<Coord>, Vec<FoldInstruction>) {
    let mut points = Vec::new();
    let mut folds = Vec::new();

    for line in input.lines() {
        if let Some(fold_part) = line.strip_prefix("fold along ") {
            let parts: Vec<&str> = fold_part.split('=').collect();
            let axis = parts[0];
            let value = parts[1].parse().unwrap();
            folds.push(FoldInstruction {
                axis: axis.to_string(),
                value,
            });
        } else if !line.trim().is_empty() {
            let coords: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
            points.push(Coord {
                x: coords[0],
                y: coords[1],
            });
        }
    }

    (points, folds)
}

fn fold_paper(paper: &HashSet<Coord>, fold: &FoldInstruction) -> HashSet<Coord> {
    let mut new_paper = HashSet::new();

    for point in paper {
        let new_point = match fold.axis.as_str() {
            "x" => {
                if point.x > fold.value {
                    Coord {
                        x: fold.value - (point.x - fold.value),
                        y: point.y,
                    }
                } else {
                    *point
                }
            }
            "y" => {
                if point.y > fold.value {
                    Coord {
                        x: point.x,
                        y: fold.value - (point.y - fold.value),
                    }
                } else {
                    *point
                }
            }
            _ => unreachable!(),
        };
        new_paper.insert(new_point);
    }

    new_paper
}

fn print_paper(paper: &HashSet<Coord>) {
    let max_x = paper.iter().map(|p| p.x).max().unwrap();
    let max_y = paper.iter().map(|p| p.y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if paper.contains(&Coord { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn solve_p1(input: &str) -> i32 {
    let (points, folds) = parse_input(input);
    let mut paper = HashSet::new();

    for point in points {
        paper.insert(point);
    }

    let fold = folds.first().unwrap();
    paper = fold_paper(&paper, fold);
    paper.len() as i32
}

fn solve_p2(input: &str) -> i32 {
    let (points, folds) = parse_input(input);
    let mut paper = HashSet::new();

    for point in points {
        paper.insert(point);
    }

    for fold in folds {
        paper = fold_paper(&paper, &fold);
    }

    print_paper(&paper);

    0
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
        assert_eq!(answer, 17);
        let answer = solve_p2(&input);
        assert_eq!(answer, 0);
    }
}
