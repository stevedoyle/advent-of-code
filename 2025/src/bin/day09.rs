use aoc2025::*;
use itertools::Itertools;

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
    ((p2.x - p1.x).unsigned_abs() + 1) * ((p2.y - p1.y).unsigned_abs() + 1)
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

/// Get normalized edges from vertices (min_x, min_y, max_x, max_y)
fn get_normalized_edges(vertices: &[Point]) -> Vec<(isize, isize, isize, isize)> {
    let mut edges = Vec::with_capacity(vertices.len());

    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();
        let p1 = vertices[i];
        let p2 = vertices[j];
        edges.push((
            p1.x.min(p2.x),
            p1.y.min(p2.y),
            p1.x.max(p2.x),
            p1.y.max(p2.y),
        ));
    }

    edges
}

/// Check if rectangle does NOT have any polygon edges cutting through it
/// Uses bounding box overlap detection - if any edge overlaps, return false
fn no_edge_intersects(
    edges: &[(isize, isize, isize, isize)],
    min_x: isize,
    min_y: isize,
    max_x: isize,
    max_y: isize,
) -> bool {
    for &(e_min_x, e_min_y, e_max_x, e_max_y) in edges {
        // Check if edge bounding box overlaps with rectangle
        if min_x < e_max_x && max_x > e_min_x && min_y < e_max_y && max_y > e_min_y {
            return false;
        }
    }
    true
}

fn solve_p2(input: &str) -> usize {
    let vertices = parse_input(input);

    // Pre-compute normalized edges
    let edges = get_normalized_edges(&vertices);

    // Generate all pairs with their potential area, sorted descending
    let mut pairs: Vec<(usize, usize, usize)> = Vec::new();
    for i in 0..vertices.len() {
        for j in (i + 1)..vertices.len() {
            let p1 = &vertices[i];
            let p2 = &vertices[j];
            let width = (p1.x - p2.x).unsigned_abs() + 1;
            let height = (p1.y - p2.y).unsigned_abs() + 1;
            let potential_area = width * height;
            pairs.push((i, j, potential_area));
        }
    }

    // Sort by potential area descending
    pairs.sort_by_key(|&(_, _, area)| std::cmp::Reverse(area));

    let mut max_area = 0;

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

        // Fast check: verify no polygon edges cut through the rectangle
        if !no_edge_intersects(&edges, rect_min_x, rect_min_y, rect_max_x, rect_max_y) {
            continue;
        }

        max_area = potential_area;
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
