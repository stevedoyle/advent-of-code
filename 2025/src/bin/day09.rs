use aoc2025::*;
use itertools::Itertools;
use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x: isize = parts.next().unwrap().trim().parse().unwrap();
            let y: isize = parts.next().unwrap().trim().parse().unwrap();
            Point::new(x, y)
        })
        .collect()
}

fn area(p1: &Point, p2: &Point) -> usize {
    ((p2.x - p1.x).abs() as usize + 1) * ((p2.y - p1.y).abs() as usize + 1)
}

fn solve_p1(input: &str) -> usize {
    let tiles = parse_input(input);
    tiles
        .iter()
        .combinations(2)
        .map(|v| area(v[0], v[1]))
        .max()
        .unwrap_or(0)
}

/// Check if a point is on a line segment
fn is_on_segment(point: &Point, start: &Point, end: &Point) -> bool {
    let dx = end.x - start.x;
    let dy = end.y - start.y;

    // Check if point is within bounding box of segment
    if point.x < start.x.min(end.x) || point.x > start.x.max(end.x) {
        return false;
    }
    if point.y < start.y.min(end.y) || point.y > start.y.max(end.y) {
        return false;
    }

    // Check if point is collinear with segment
    // Using cross product: (point - start) × (end - start) = 0
    let cross = (point.x - start.x) * dy - (point.y - start.y) * dx;
    cross == 0
}

/// Check if a point is inside or on the boundary of a polygon using ray casting algorithm
fn is_inside_polygon(point: &Point, vertices: &[Point]) -> bool {
    let n = vertices.len();

    // First check if point is on any edge
    for i in 0..n {
        let j = (i + 1) % n;
        if is_on_segment(point, &vertices[i], &vertices[j]) {
            return true;
        }
    }

    // Ray casting for interior points
    let mut inside = false;
    for i in 0..n {
        let j = (i + 1) % n;
        let vi = vertices[i];
        let vj = vertices[j];

        // Check if ray from point to the right crosses edge (vi, vj)
        if ((vi.y > point.y) != (vj.y > point.y))
            && (point.x < (vj.x - vi.x) * (point.y - vi.y) / (vj.y - vi.y) + vi.x)
        {
            inside = !inside;
        }
    }

    inside
}

fn solve_p2(input: &str) -> usize {
    let vertices = parse_input(input);

    // Generate all pairs with their potential area, sorted descending
    let mut pairs: Vec<(usize, usize, usize)> = Vec::new();
    for i in 0..vertices.len() {
        for j in (i + 1)..vertices.len() {
            let p1 = &vertices[i];
            let p2 = &vertices[j];
            let width = (p1.x - p2.x).abs() as usize + 1;
            let height = (p1.y - p2.y).abs() as usize + 1;
            let potential_area = width * height;
            pairs.push((i, j, potential_area));
        }
    }

    // Sort by potential area descending
    pairs.sort_by_key(|&(_, _, area)| std::cmp::Reverse(area));

    let mut max_area = 0;
    let mut cache: HashMap<Point, bool> = HashMap::new();

    // Try pairs in order of descending potential area
    for (i, j, potential_area) in pairs {
        // Early exit: if potential area can't beat max, we're done
        if potential_area <= max_area {
            break;
        }

        let p1 = &vertices[i];
        let p2 = &vertices[j];

        let rect_min_x = p1.x.min(p2.x);
        let rect_max_x = p1.x.max(p2.x);
        let rect_min_y = p1.y.min(p2.y);
        let rect_max_y = p1.y.max(p2.y);

        let check_point = |p: &Point, cache: &mut HashMap<Point, bool>| -> bool {
            *cache.entry(*p).or_insert_with(|| is_inside_polygon(p, &vertices))
        };

        // Check all 4 corners first (they may not be the original vertices)
        if !check_point(&Point::new(rect_min_x, rect_min_y), &mut cache) {
            continue;
        }
        if !check_point(&Point::new(rect_max_x, rect_min_y), &mut cache) {
            continue;
        }
        if !check_point(&Point::new(rect_min_x, rect_max_y), &mut cache) {
            continue;
        }
        if !check_point(&Point::new(rect_max_x, rect_max_y), &mut cache) {
            continue;
        }

        let mut valid = true;

        // Check top and bottom edges (excluding corners)
        for x in (rect_min_x + 1)..rect_max_x {
            if !check_point(&Point::new(x, rect_min_y), &mut cache)
                || !check_point(&Point::new(x, rect_max_y), &mut cache)
            {
                valid = false;
                break;
            }
        }

        if !valid {
            continue;
        }

        // Check left and right edges (skip corners already checked)
        for y in (rect_min_y + 1)..rect_max_y {
            if !check_point(&Point::new(rect_min_x, y), &mut cache)
                || !check_point(&Point::new(rect_max_x, y), &mut cache)
            {
                valid = false;
                break;
            }
        }

        if valid {
            max_area = potential_area;
        }
    }

    max_area
}

fn main() {
    let input = read_input(9);

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
        let input = read_test_input(9);
        let answer = solve_p1(&input);
        assert_eq!(answer, 50);
        let answer = solve_p2(&input);
        assert_eq!(answer, 24);
    }
}
