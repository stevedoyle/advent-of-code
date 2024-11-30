use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Cube { x, y, z }
    }

    // The list of cubes surrounding this cube
    fn neighbors(self) -> Vec<Cube> {
        [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ]
        .iter()
        .map(|(dx, dy, dz)| Cube::new(self.x + dx, self.y + dy, self.z + dz))
        .collect()
    }
}

impl FromStr for Cube {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<i32> = s.split(',').map(|s| s.parse().unwrap()).collect();
        Ok(Cube::new(parts[0], parts[1], parts[2]))
    }
}

fn parse_input(input: &str) -> Vec<Cube> {
    input
        .lines()
        .map(|line| Cube::from_str(line).unwrap())
        .collect()
}

fn solve_p1(input: &str) -> i32 {
    let cubes = parse_input(input);
    let mut exposed = 0;
    for cube in cubes.iter() {
        for n in cube.neighbors() {
            if !cubes.contains(&n) {
                exposed += 1;
            }
        }
    }
    exposed
}

fn solve_p2(input: &str) -> i32 {
    let cubes = parse_input(input);
    let max_bound = Cube::new(
        cubes.iter().max_by_key(|c| c.x).unwrap().x + 1,
        cubes.iter().max_by_key(|c| c.y).unwrap().y + 1,
        cubes.iter().max_by_key(|c| c.z).unwrap().z + 1,
    );

    let min_bound = Cube::new(
        cubes.iter().min_by_key(|c| c.x).unwrap().x - 1,
        cubes.iter().min_by_key(|c| c.y).unwrap().y - 1,
        cubes.iter().min_by_key(|c| c.z).unwrap().z - 1,
    );

    let mut seen: HashSet<Cube> = HashSet::new();
    let mut queue = vec![max_bound];
    let mut exposed_outside = 0;

    while let Some(cube) = queue.pop() {
        if cubes.contains(&cube) {
            exposed_outside += 1;
            continue;
        }
        if seen.contains(&cube) {
            continue;
        }
        seen.insert(cube);
        for n in cube.neighbors() {
            if n.x < min_bound.x || n.y < min_bound.y || n.z < min_bound.z {
                continue;
            }
            if n.x > max_bound.x || n.y > max_bound.y || n.z > max_bound.z {
                continue;
            }
            queue.push(n);
        }
    }

    exposed_outside
}

fn main() {
    let input = include_str!("../input.txt");
    let answer = solve_p1(input);
    println!("Part 1: {answer}");
    let answer = solve_p2(input);
    println!("Part 2: {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1("1,1,1\n2,1,1");
        assert_eq!(answer, 10);
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 64);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 58);
    }
}
