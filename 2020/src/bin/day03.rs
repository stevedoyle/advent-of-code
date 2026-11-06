use aoc2020::*;

const DAY: u8 = 3;

fn parse_input(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn traverse_slope(slope: &Grid<u8>, right: i64, down: i64) -> usize {
    let step = Point::new(right, down);
    let mut pos = Point::ORIGIN;
    let mut tree_count = 0;
    while pos.y < slope.height {
        if slope[Point::new(pos.x % slope.width, pos.y)] == b'#' {
            tree_count += 1;
        }
        pos += step;
    }
    tree_count
}

fn solve_p1(input: &str) -> usize {
    let slope = parse_input(input);
    traverse_slope(&slope, 3, 1)
}

fn solve_p2(input: &str) -> usize {
    let slope = parse_input(input);
    let mut product = 1;
    for (right, down) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let count = traverse_slope(&slope, *right, *down);
        product *= count;
    }
    product
}

fn main() {
    run(DAY, solve_p1, solve_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve_p1(&read_test_input(DAY, 1)), 7);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_p2(&read_test_input(DAY, 1)), 336);
    }
}
