use std::{error::Error, fs};

#[derive(Debug)]
struct Grid(Vec<Vec<char>>);

impl std::ops::Deref for Grid {
    type Target = Vec<Vec<char>>;
    fn deref(&self) -> &Vec<Vec<char>> {
        &self.0
    }
}

impl std::ops::DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Vec<Vec<char>> {
        &mut self.0
    }
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        Grid(
            s.lines()
                .filter(|&line| !line.trim().is_empty())
                .map(|line| line.trim().chars().collect::<Vec<char>>())
                .collect(),
        )
    }
}

impl Grid {
    fn find_start(&self) -> Option<Point> {
        for (y, row) in self.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                if col == 'S' {
                    return Some(Point(x, y));
                }
            }
        }
        None
    }

    fn walk(&self) -> Vec<Point> {
        let start = match self.find_start() {
            Some(start) => start,
            _ => return vec![],
        };

        let mut paths: Vec<Vec<Point>> = Vec::new();
        let start_next_points = self.next_pos(&start, None);

        for p in start_next_points {
            let mut path = Vec::new();

            let mut curr = p;
            let mut prev = start;
            path.push(curr);

            loop {
                // print!("{curr:?} -> ");
                let next = self.next_pos(&curr, Some(prev));
                if next.is_empty() {
                    // println!("{curr:?}");
                    break;
                }
                prev = curr;
                curr = next[0];
                path.push(curr);

                if curr == start {
                    paths.push(path);
                    // println!("{curr:?}");
                    break;
                }
            }
        }
        if paths.is_empty() {
            dbg!("No path!");
            return vec![];
        }
        paths
            .into_iter()
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
    }

    fn next_pos(&self, pos: &Point, prev: Option<Point>) -> Vec<Point> {
        let dirs = match self[pos.1][pos.0] {
            '|' => vec![Direction::North, Direction::South],
            '-' => vec![Direction::West, Direction::East],
            'L' => vec![Direction::North, Direction::East],
            'J' => vec![Direction::North, Direction::West],
            '7' => vec![Direction::South, Direction::West],
            'F' => vec![Direction::South, Direction::East],
            'S' => vec![
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ],
            _ => vec![],
        };
        let candidates = self.dirs_to_points(&dirs, pos);
        let mut candidates = if prev.is_some() {
            candidates
                .into_iter()
                .filter(|&x| Some(x) != prev)
                .collect()
        } else {
            candidates
        };
        if self[pos.1][pos.0] == 'S' {
            let north = self.dir_to_point(Direction::North, pos);
            if let Some(p) = north {
                if !"|7F".contains(self[p.1][p.0]) {
                    candidates.retain(|x| x != &p);
                }
            }
            let south = self.dir_to_point(Direction::South, pos);
            if let Some(p) = south {
                if !"|LJ".contains(self[p.1][p.0]) {
                    candidates.retain(|x| x != &p);
                }
            }
            let east = self.dir_to_point(Direction::East, pos);
            if let Some(p) = east {
                if !"-J7".contains(self[p.1][p.0]) {
                    candidates.retain(|x| x != &p);
                }
            }
            let west = self.dir_to_point(Direction::West, pos);
            if let Some(p) = west {
                if !"-LF".contains(self[p.1][p.0]) {
                    candidates.retain(|x| x != &p);
                }
            }
        }
        candidates
    }

    fn dirs_to_points(&self, dirs: &[Direction], pos: &Point) -> Vec<Point> {
        dirs.iter()
            .filter_map(|d| self.dir_to_point(*d, pos))
            .collect()
    }

    fn dir_to_point(&self, dir: Direction, pos: &Point) -> Option<Point> {
        match dir {
            Direction::North => match pos.1 {
                0 => None,
                _ => Some(Point(pos.0, pos.1 - 1)),
            },
            Direction::South => {
                if pos.1 < self[0].len() - 1 {
                    Some(Point(pos.0, pos.1 + 1))
                } else {
                    None
                }
            }
            Direction::East => {
                if pos.0 < self.len() - 1 {
                    Some(Point(pos.0 + 1, pos.1))
                } else {
                    None
                }
            }
            Direction::West => {
                if pos.0 > 0 {
                    Some(Point(pos.0 - 1, pos.1))
                } else {
                    None
                }
            }
        }
    }

    fn enclosed(&self, points: &[Point]) -> usize {
        let area = interior_area(points);

        // Picks Theorem - find the number of points in a shape given its area
        area - points.len() / 2 + 1
    }
}

fn interior_area(points: &[Point]) -> usize {
    let mut padded_points = Vec::from(points);
    padded_points.push(points[0]);

    let area: i32 = padded_points
        .iter()
        .zip(padded_points.iter().skip(1))
        .map(|(p1, p2)| (p1.0 as i32) * (p2.1 as i32) - (p2.0 as i32) * (p1.1 as i32))
        .sum();
    (area.abs() / 2) as usize
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Point(usize, usize);

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let grid = Grid::from(input.as_str());
    println!("Start: {:?}", grid.find_start());
    let longest_path = grid.walk();
    let furthest_away = longest_path.len() / 2;
    println!("Part 1: {furthest_away}");

    let num_interior_points = grid.enclosed(&longest_path);
    println!("Part 2: {num_interior_points}");

    Ok(())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_loop() {
        let input = "
            .....
            .S-7.
            .|.|.
            .L-J.
            .....";
        let grid = Grid::from(input);
        let start = grid.find_start();
        assert_eq!(start, Some(Point(1, 1)));
        let path = grid.walk();
        let step_count = path.len();
        assert_eq!(step_count, 8);
    }

    #[test]
    fn test_dir_to_point() {
        let input = "
            .....
            .S-7.
            .|.|.
            .L-J.
            .....";
        let grid = Grid::from(input);
        assert_eq!(
            grid.dir_to_point(Direction::East, &Point(1, 1)),
            Some(Point(2, 1))
        );
        assert_eq!(
            grid.dir_to_point(Direction::West, &Point(1, 1)),
            Some(Point(0, 1))
        );
        assert_eq!(
            grid.dir_to_point(Direction::North, &Point(1, 1)),
            Some(Point(1, 0))
        );
        assert_eq!(
            grid.dir_to_point(Direction::South, &Point(1, 1)),
            Some(Point(1, 2))
        );
    }

    #[test]
    fn test_enclosed() {
        let input = "
        .....
        7S-7.
        .|.|.
        .L-J.
        .....";
        let grid = Grid::from(input);
        let path = grid.walk();
        let enclosed_area = grid.enclosed(&path);
        assert_eq!(enclosed_area, 1);
    }

    #[test]
    fn test_next_pos() {
        let input = "
        .....
        7S-7.
        .|.|.
        .L-J.
        .....";
        let grid = Grid::from(input);
        let next = grid.next_pos(&Point(1, 1), None);
        assert_eq!(next.len(), 2);
        assert!(next.contains(&Point(2, 1)));
        assert!(next.contains(&Point(1, 2)));
    }
}
