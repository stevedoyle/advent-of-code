use std::collections::HashSet;

use aoc2020::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate3D {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate4D {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Coordinate4D {
    fn new(coord: &Coordinate3D) -> Self {
        Coordinate4D {
            x: coord.x,
            y: coord.y,
            z: coord.z,
            w: 0,
        }
    }

    fn neighbors(&self) -> Vec<Coordinate4D> {
        let mut result = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    for dw in -1..=1 {
                        if dx != 0 || dy != 0 || dz != 0 || dw != 0 {
                            result.push(Coordinate4D {
                                x: self.x + dx,
                                y: self.y + dy,
                                z: self.z + dz,
                                w: self.w + dw,
                            });
                        }
                    }
                }
            }
        }
        result
    }
}

fn parse_input(input: &str) -> HashSet<Coordinate3D> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| Coordinate3D {
                    x: x as i32,
                    y: y as i32,
                    z: 0,
                })
        })
        .collect()
}

fn neighbors(coord: &Coordinate3D) -> Vec<Coordinate3D> {
    let mut result = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                if dx != 0 || dy != 0 || dz != 0 {
                    result.push(Coordinate3D {
                        x: coord.x + dx,
                        y: coord.y + dy,
                        z: coord.z + dz,
                    });
                }
            }
        }
    }
    result
}

fn solve_p1(input: &str) -> usize {
    let mut space = parse_input(input);
    for _cycle in 0..6 {
        let mut new_space = HashSet::new();
        let mut to_check = HashSet::new();

        for coord in &space {
            to_check.insert(*coord);
            for neighbor in neighbors(coord) {
                to_check.insert(neighbor);
            }
        }

        for coord in to_check {
            let active_neighbors = neighbors(&coord)
                .iter()
                .filter(|n| space.contains(n))
                .count();
            if space.contains(&coord) {
                if active_neighbors == 2 || active_neighbors == 3 {
                    new_space.insert(coord);
                }
            } else {
                if active_neighbors == 3 {
                    new_space.insert(coord);
                }
            }
        }

        space = new_space;
    }
    space.len()
}

fn solve_p2(input: &str) -> usize {
    let space3d = parse_input(input);
    let mut space4d: HashSet<Coordinate4D> = space3d.iter().map(|c| Coordinate4D::new(c)).collect();
    for _cycle in 0..6 {
        let mut new_space = HashSet::new();
        let mut to_check = HashSet::new();
        for coord in &space4d {
            to_check.insert(*coord);
            for neighbor in coord.neighbors() {
                to_check.insert(neighbor);
            }
        }
        for coord in to_check {
            let active_neighbors = coord
                .neighbors()
                .iter()
                .filter(|n| space4d.contains(n))
                .count();
            if space4d.contains(&coord) {
                if active_neighbors == 2 || active_neighbors == 3 {
                    new_space.insert(coord);
                }
            } else {
                if active_neighbors == 3 {
                    new_space.insert(coord);
                }
            }
        }
        space4d = new_space;
    }
    space4d.len()
}

fn main() {
    let input = read_input(17);

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
        let input = read_test_input(17);
        let answer = solve_p1(&input);
        assert_eq!(answer, 112);
        let answer = solve_p2(&input);
        assert_eq!(answer, 848);
    }
}
