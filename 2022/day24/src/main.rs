use std::collections::{HashSet, VecDeque};

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    (a * b) / gcd(a, b)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Blizzard {
    row: i32,
    col: i32,
    dir: Direction,
}

#[derive(Debug, Clone)]
struct Valley {
    width: i32,
    height: i32,
    blizzards: Vec<Blizzard>,
    start: (i32, i32),
    goal: (i32, i32),
    blizzard_cache: Vec<HashSet<(i32, i32)>>,
    cycle_length: i32,
}

impl Valley {
    fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
        let height = lines.len() as i32;
        let width = lines[0].len() as i32;

        let mut blizzards = Vec::new();

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let dir = match ch {
                    '^' => Some(Direction::Up),
                    'v' => Some(Direction::Down),
                    '<' => Some(Direction::Left),
                    '>' => Some(Direction::Right),
                    _ => None,
                };

                if let Some(dir) = dir {
                    blizzards.push(Blizzard {
                        row: row as i32,
                        col: col as i32,
                        dir,
                    });
                }
            }
        }

        // Find start (top row, open position)
        let start_col = lines[0].chars().position(|c| c == '.').unwrap() as i32;
        let start = (0, start_col);

        // Find goal (bottom row, open position)
        let goal_col = lines[height as usize - 1]
            .chars()
            .position(|c| c == '.')
            .unwrap() as i32;
        let goal = (height - 1, goal_col);

        // Blizzards cycle with period = LCM of inner dimensions
        let inner_width = width - 2;
        let inner_height = height - 2;
        let cycle_length = lcm(inner_width, inner_height);

        let mut valley = Valley {
            width,
            height,
            blizzards,
            start,
            goal,
            blizzard_cache: Vec::new(),
            cycle_length,
        };

        // Pre-compute blizzard positions for each time in the cycle
        valley.blizzard_cache = (0..cycle_length)
            .map(|t| valley.compute_blizzards_at_time(t))
            .collect();

        valley
    }

    fn compute_blizzards_at_time(&self, time: i32) -> HashSet<(i32, i32)> {
        let mut positions = HashSet::new();
        for blizzard in &self.blizzards {
            let (new_row, new_col) = match blizzard.dir {
                Direction::Up => {
                    let inner_height = self.height - 2;
                    let start_row = blizzard.row - 1;
                    let new_row = ((start_row - time).rem_euclid(inner_height)) + 1;
                    (new_row, blizzard.col)
                }
                Direction::Down => {
                    let inner_height = self.height - 2;
                    let start_row = blizzard.row - 1;
                    let new_row = ((start_row + time).rem_euclid(inner_height)) + 1;
                    (new_row, blizzard.col)
                }
                Direction::Left => {
                    let inner_width = self.width - 2;
                    let start_col = blizzard.col - 1;
                    let new_col = ((start_col - time).rem_euclid(inner_width)) + 1;
                    (blizzard.row, new_col)
                }
                Direction::Right => {
                    let inner_width = self.width - 2;
                    let start_col = blizzard.col - 1;
                    let new_col = ((start_col + time).rem_euclid(inner_width)) + 1;
                    (blizzard.row, new_col)
                }
            };
            positions.insert((new_row, new_col));
        }
        positions
    }

    fn has_blizzard_at(&self, row: i32, col: i32, time: i32) -> bool {
        let cycle_time = time % self.cycle_length;
        self.blizzard_cache[cycle_time as usize].contains(&(row, col))
    }

    fn is_valid_position(&self, row: i32, col: i32, time: i32) -> bool {
        // Check if it's the start or goal position
        if (row, col) == self.start || (row, col) == self.goal {
            return true;
        }

        // Check bounds (must be inside walls)
        if row <= 0 || row >= self.height - 1 || col <= 0 || col >= self.width - 1 {
            return false;
        }

        // Check for blizzards
        !self.has_blizzard_at(row, col, time)
    }

    fn find_shortest_path(&self, start: (i32, i32), goal: (i32, i32), start_time: i32) -> i32 {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((start.0, start.1, start_time));
        visited.insert((start.0, start.1, start_time % self.cycle_length));

        let moves = [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)]; // wait, right, left, down, up

        while let Some((row, col, time)) = queue.pop_front() {
            if (row, col) == goal {
                return time;
            }

            let next_time = time + 1;

            for (dr, dc) in moves {
                let new_row = row + dr;
                let new_col = col + dc;

                if self.is_valid_position(new_row, new_col, next_time) {
                    let state = (new_row, new_col, next_time % self.cycle_length);
                    if !visited.contains(&state) {
                        visited.insert(state);
                        queue.push_back((new_row, new_col, next_time));
                    }
                }
            }
        }

        -1 // No path found
    }
}

fn solve_p1(input: &str) -> i32 {
    let valley = Valley::parse(input);
    valley.find_shortest_path(valley.start, valley.goal, 0)
}

fn solve_p2(input: &str) -> i32 {
    let valley = Valley::parse(input);
    let time1 = valley.find_shortest_path(valley.start, valley.goal, 0);
    let time2 = valley.find_shortest_path(valley.goal, valley.start, time1);
    valley.find_shortest_path(valley.start, valley.goal, time2)
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
        assert_eq!(answer, 18);
        let answer = solve_p2(&input);
        assert_eq!(answer, 54);
    }
}
