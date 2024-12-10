use pathfinding::{matrix::Matrix, prelude::dijkstra_all};

fn parse_input(input: &str) -> Matrix<u32> {
    Matrix::from_rows(input.lines().map(|line| {
        line.chars()
            .map(|ch| ch.to_digit(10).unwrap())
            .collect::<Vec<_>>()
    }))
    .unwrap()
}

#[allow(dead_code)]
fn display_map(map: &Matrix<u32>) {
    for row in map {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn successors(pos: (usize, usize), map: &Matrix<u32>) -> Vec<((usize, usize), u32)> {
    let val_at_pos = map[pos];
    map.neighbours(pos, false)
        .filter_map(|n| {
            let val = map[n];
            if val == val_at_pos + 1 {
                Some((n, 1))
            } else {
                None
            }
        })
        .collect()
}

fn find_all_paths(map: &Matrix<u32>, start: (usize, usize)) -> Vec<Vec<(usize, usize)>> {
    let mut paths = vec![];
    let mut stack = vec![(vec![start], start)];
    while let Some((path, pos)) = stack.pop() {
        if map[pos] == 9 {
            paths.push(path);
            continue;
        }
        for (next_pos, _) in successors(pos, map) {
            if !path.contains(&next_pos) {
                let mut new_path = path.clone();
                new_path.push(next_pos);
                stack.push((new_path, next_pos));
            }
        }
    }
    paths
}

fn solve_p1(input: &str) -> usize {
    let map = parse_input(input);
    let zeros = map
        .items()
        .filter(|(_, &val)| val == 0)
        .map(|(pos, _)| pos)
        .collect::<Vec<_>>();

    let mut trail_count = 0;
    for start in zeros {
        let reachables = dijkstra_all(&start, |&p| successors(p, &map));
        trail_count += reachables.keys().filter(|&k| map[*k] == 9).count();
    }
    trail_count
}

fn solve_p2(input: &str) -> usize {
    let map = parse_input(input);
    let zeros = map
        .items()
        .filter(|(_, &val)| val == 0)
        .map(|(pos, _)| pos)
        .collect::<Vec<_>>();

    let mut trail_rating = 0;
    for start in zeros {
        let paths = find_all_paths(&map, start);
        trail_rating += paths.len();
    }
    trail_rating
}

fn main() {
    let input = include_str!("../input.txt");

    let start = std::time::Instant::now();
    let answer = solve_p1(input);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:.1?}");

    let start = std::time::Instant::now();
    let answer = solve_p2(input);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, elapsed: {elapsed:.1?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_with_test_input() {
        let input = std::fs::read_to_string("test.txt").unwrap();
        let answer = solve_p1(&input);
        assert_eq!(answer, 36);
        let answer = solve_p2(&input);
        assert_eq!(answer, 81);
    }
}
