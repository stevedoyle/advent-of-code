use std::collections::HashMap;
use std::str::FromStr;

use aoc2025::*;

struct Region {
    area: usize,
    shape_counts: Vec<usize>,
}

impl FromStr for Region {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Example line: "12x5: 1 0 1 0 3 2"
        let parts: Vec<&str> = s.split(": ").collect();
        let dimensions: Vec<usize> = parts[0].split('x').map(|n| n.parse().unwrap()).collect();
        let width = dimensions[0];
        let height = dimensions[1];
        let shapes: Vec<usize> = parts[1].split(' ').map(|n| n.parse().unwrap()).collect();

        Ok(Region {
            area: width * height,
            shape_counts: shapes,
        })
    }
}

struct Shape {
    id: usize,
    area: usize,
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once(":").unwrap();
        let id = parts.0.trim().parse().unwrap();
        let area = parts.1.chars().filter(|&c| c == '#').count();
        Ok(Shape { id, area })
    }
}

fn parse_input(input: &str) -> (HashMap<usize, usize>, Vec<Region>) {
    let mut shapes = HashMap::new();
    let mut regions = Vec::new();

    let parts = input.split("\n\n").collect::<Vec<_>>();
    for part in parts {
        if part.contains('x') {
            for line in part.lines() {
                let region: Region = line.parse().unwrap();
                regions.push(region);
            }
        } else {
            let shape: Shape = part.parse().unwrap();
            shapes.insert(shape.id, shape.area);
        }
    }

    (shapes, regions)
}

fn solve_p1(input: &str) -> usize {
    let (shapes, regions) = parse_input(input);

    regions
        .iter()
        .filter(|region| {
            let total_area: usize = region
                .shape_counts
                .iter()
                .enumerate()
                .map(|(shape_id, &count)| shapes.get(&shape_id).unwrap_or(&0) * count)
                .sum();
            total_area <= region.area
        })
        .count()
}

fn main() {
    let input = read_input(12);

    let start = std::time::Instant::now();
    let answer = solve_p1(&input);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:.1?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_with_test_input() {
        let input = read_test_input(12);
        let answer = solve_p1(&input);
        assert_eq!(answer, 3);
    }
}
