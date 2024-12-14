use std::str::FromStr;

use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}

impl Robot {
    fn step(&mut self, bounds: (usize, usize)) {
        self.step_n(1, bounds);
    }

    fn step_n(&mut self, n: usize, bounds: (usize, usize)) {
        self.p.0 += self.v.0 * n as i32;
        self.p.1 += self.v.1 * n as i32;
        self.p.0 = self.p.0.rem_euclid(bounds.0 as i32);
        self.p.1 = self.p.1.rem_euclid(bounds.1 as i32);
    }
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        let captures = pattern.captures(s).unwrap();
        let p = (
            captures.get(1).unwrap().as_str().parse().unwrap(),
            captures.get(2).unwrap().as_str().parse().unwrap(),
        );
        let v = (
            captures.get(3).unwrap().as_str().parse().unwrap(),
            captures.get(4).unwrap().as_str().parse().unwrap(),
        );
        Ok(Robot { p, v })
    }
}

fn safety_factor(robots: &[Robot], bounds: (usize, usize)) -> usize {
    let mid_x = bounds.0 as i32 / 2;
    let mid_y = bounds.1 as i32 / 2;

    let quadrants = robots.iter().fold([0; 4], |mut acc, robot| {
        if robot.p.0 < mid_x && robot.p.1 < mid_y {
            acc[0] += 1;
        } else if robot.p.0 > mid_x && robot.p.1 < mid_y {
            acc[1] += 1;
        } else if robot.p.0 < mid_x && robot.p.1 > mid_y {
            acc[2] += 1;
        } else if robot.p.0 > mid_x && robot.p.1 > mid_y {
            acc[3] += 1;
        }
        acc
    });

    quadrants.iter().product()
}

fn parse_input(input: &str) -> Vec<Robot> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn solve_p1(robots: &[Robot], bounds: (usize, usize)) -> usize {
    let mut robots = robots.to_vec();
    robots.iter_mut().for_each(|robot| {
        robot.step_n(100, bounds);
    });

    safety_factor(&robots, bounds)
}

fn solve_p2(robots: &[Robot], bounds: (usize, usize)) -> usize {
    let mut robots = robots.to_vec();

    let mut seconds = 0;
    let mut min_safety = safety_factor(&robots, bounds);
    let mut min_safety_seconds = 0;

    for _ in 0..(bounds.0 * bounds.1) {
        seconds += 1;
        robots.iter_mut().for_each(|robot| {
            robot.step(bounds);
        });
        let safety = safety_factor(&robots, bounds);
        if safety < min_safety {
            min_safety = safety;
            min_safety_seconds = seconds;
        }
    }
    min_safety_seconds
}

fn format_robots(robots: &[Robot], bounds: (usize, usize)) -> String {
    let mut grid = vec![vec!['.'; bounds.0]; bounds.1];
    for robot in robots {
        grid[robot.p.1 as usize][robot.p.0 as usize] = '#';
    }
    grid.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let robots = parse_input(&input);
    let bounds = (101, 103);

    let start = std::time::Instant::now();
    let answer = solve_p1(&robots, bounds);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:.1?}");

    let start = std::time::Instant::now();
    let answer = solve_p2(&robots, bounds);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, elapsed: {elapsed:.1?}");

    let mut robots = robots.to_vec();
    robots.iter_mut().for_each(|robot| {
        robot.step_n(answer, bounds);
    });
    println!("{}", format_robots(&robots, bounds));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_with_test_input() {
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let robots = parse_input(&input);
        let answer = solve_p1(&robots, (11, 7));
        assert_eq!(answer, 12);
    }

    #[test]
    fn test_robot_step_n() {
        let mut robot = Robot {
            p: (0, 0),
            v: (1, 1),
        };
        robot.step_n(1, (10, 10));
        assert_eq!(robot.p, (1, 1));
        robot.step_n(8, (10, 10));
        assert_eq!(robot.p, (9, 9));
        robot.step_n(1, (10, 10));
        assert_eq!(robot.p, (0, 0));
    }

    #[test]
    fn test_robot_step() {
        let mut robot = Robot {
            p: (0, 0),
            v: (1, 1),
        };
        robot.step((10, 10));
        assert_eq!(robot.p, (1, 1));
    }
}
