use std::str::FromStr;

use aoc2025::*;

#[derive(Debug, Clone, Copy)]
struct Coord3D {
    x: isize,
    y: isize,
    z: isize,
}

impl FromStr for Coord3D {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split(',').collect();
        if parts.len() != 3 {
            return Err(());
        }
        let x = parts[0].parse::<isize>().map_err(|_| ())?;
        let y = parts[1].parse::<isize>().map_err(|_| ())?;
        let z = parts[2].parse::<isize>().map_err(|_| ())?;
        Ok(Coord3D { x, y, z })
    }
}

fn parse_input(input: &str) -> Vec<Coord3D> {
    parse_lines(input)
}

fn distance(a: &Coord3D, b: &Coord3D) -> isize {
    // This is not the Euclidean distance, but instead the squared distance is used to avoid
    // floating point operations.
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    dx * dx + dy * dy + dz * dz
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    num_components: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            num_components: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            if self.size[root_x] < self.size[root_y] {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            } else {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
            self.num_components -= 1;
            true // Components were merged
        } else {
            false // Already in the same component
        }
    }

    fn num_components(&self) -> usize {
        self.num_components
    }

    fn component_sizes(&mut self) -> Vec<usize> {
        use std::collections::HashMap;
        let mut sizes = HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            *sizes.entry(root).or_insert(0) += 1;
        }
        sizes.into_values().collect()
    }
}

fn pairwise_distances(boxes: &[Coord3D]) -> Vec<((usize, usize), isize)> {
    let n = boxes.len();
    let mut distances = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            let dist = distance(&boxes[i], &boxes[j]);
            distances.push(((i, j), dist));
        }
    }
    distances.sort_by(|a, b| a.1.cmp(&b.1));
    distances
}

fn find_circuit_sizes(boxes: &[Coord3D], num_connections: usize) -> Vec<usize> {
    let distances = pairwise_distances(boxes);

    // Build connected components using Union-Find
    let mut uf = UnionFind::new(boxes.len());
    for &((i, j), _) in distances.iter().take(num_connections) {
        uf.union(i, j);
    }

    uf.component_sizes()
}

fn solve_p1(input: &str, num_connections: usize) -> usize {
    let boxes = parse_input(input);
    let mut circuit_sizes = find_circuit_sizes(&boxes, num_connections);
    circuit_sizes.sort_unstable_by(|a, b| b.cmp(a));
    circuit_sizes.iter().take(3).product()
}

fn solve_p2(input: &str) -> usize {
    let boxes = parse_input(input);
    let distances = pairwise_distances(&boxes);

    // Build connected components using Union-Find
    let mut uf = UnionFind::new(boxes.len());
    for &((i, j), _) in distances.iter() {
        uf.union(i, j);
        if uf.num_components() == 1 {
            return boxes[i].x as usize * boxes[j].x as usize;
        }
    }
    0
}

fn main() {
    let input = read_input(8);

    let start = std::time::Instant::now();
    let answer = solve_p1(&input, 1000);
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
        let input = read_test_input(8);
        let answer = solve_p1(&input, 10);
        assert_eq!(answer, 40);
        let answer = solve_p2(&input);
        assert_eq!(answer, 25272);
    }
}
