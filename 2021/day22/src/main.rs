use std::str::FromStr;

enum State {
    On,
    Off,
}

struct Cuboid {
    x_range: (i32, i32),
    y_range: (i32, i32),
    z_range: (i32, i32),
    state: State,
}

impl FromStr for Cuboid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Example line: "on x=10..12,y=10..12,z=10..12"
        let parts: Vec<&str> = s.split_whitespace().collect();
        let state = match parts[0] {
            "on" => State::On,
            "off" => State::Off,
            _ => return Err(()),
        };

        let ranges: Vec<&str> = parts[1].split(',').collect();
        let x_range: Vec<i32> = ranges[0][2..]
            .split("..")
            .map(|n| n.parse().unwrap())
            .collect();
        let y_range: Vec<i32> = ranges[1][2..]
            .split("..")
            .map(|n| n.parse().unwrap())
            .collect();
        let z_range: Vec<i32> = ranges[2][2..]
            .split("..")
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(Cuboid {
            x_range: (x_range[0], x_range[1]),
            y_range: (y_range[0], y_range[1]),
            z_range: (z_range[0], z_range[1]),
            state,
        })
    }
}

fn parse_input(input: &str) -> Vec<Cuboid> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[allow(clippy::needless_range_loop)]
fn solve_p1(input: &str) -> usize {
    let cuboids = parse_input(input);
    let mut reactor: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 101]; 101]; 101];
    for cuboid in cuboids {
        // Only clamp the start to 0, and end to 100
        // If completely outside, the range will be invalid (start > end) and loop won't execute
        let x_start = (cuboid.x_range.0 + 50).max(0);
        let x_end = (cuboid.x_range.1 + 50).min(100);
        let y_start = (cuboid.y_range.0 + 50).max(0);
        let y_end = (cuboid.y_range.1 + 50).min(100);
        let z_start = (cuboid.z_range.0 + 50).max(0);
        let z_end = (cuboid.z_range.1 + 50).min(100);

        // Skip if cuboid doesn't overlap with initialization area
        if x_start > x_end || y_start > y_end || z_start > z_end {
            continue;
        }

        let x_start = x_start as usize;
        let x_end = x_end as usize;
        let y_start = y_start as usize;
        let y_end = y_end as usize;
        let z_start = z_start as usize;
        let z_end = z_end as usize;

        for x in x_start..=x_end {
            for y in y_start..=y_end {
                for z in z_start..=z_end {
                    match cuboid.state {
                        State::On => reactor[x][y][z] = true,
                        State::Off => reactor[x][y][z] = false,
                    }
                }
            }
        }
    }
    let mut count = 0;
    for x in 0..=100 {
        for y in 0..=100 {
            for z in 0..=100 {
                if reactor[x][y][z] {
                    count += 1;
                }
            }
        }
    }
    count
}

fn solve_p2(input: &str) -> i64 {
    let cuboids = parse_input(input);

    #[derive(Clone, Copy, Debug)]
    struct Region {
        x: (i32, i32),
        y: (i32, i32),
        z: (i32, i32),
    }

    impl Region {
        fn volume(&self) -> i64 {
            let x_len = (self.x.1 - self.x.0 + 1) as i64;
            let y_len = (self.y.1 - self.y.0 + 1) as i64;
            let z_len = (self.z.1 - self.z.0 + 1) as i64;
            x_len * y_len * z_len
        }

        fn intersect(&self, other: &Region) -> Option<Region> {
            let x_start = self.x.0.max(other.x.0);
            let x_end = self.x.1.min(other.x.1);
            let y_start = self.y.0.max(other.y.0);
            let y_end = self.y.1.min(other.y.1);
            let z_start = self.z.0.max(other.z.0);
            let z_end = self.z.1.min(other.z.1);

            if x_start <= x_end && y_start <= y_end && z_start <= z_end {
                Some(Region {
                    x: (x_start, x_end),
                    y: (y_start, y_end),
                    z: (z_start, z_end),
                })
            } else {
                None
            }
        }
    }

    // Track regions with their sign (positive for on, negative for compensation)
    let mut regions: Vec<(Region, i64)> = Vec::new();

    for cuboid in cuboids {
        let new_region = Region {
            x: cuboid.x_range,
            y: cuboid.y_range,
            z: cuboid.z_range,
        };

        let mut to_add = Vec::new();

        // For each existing region, compute intersection with new region
        for &(region, sign) in &regions {
            if let Some(intersection) = region.intersect(&new_region) {
                // Add the intersection with opposite sign to cancel out overlap
                to_add.push((intersection, -sign));
            }
        }

        // Add the compensation regions
        regions.extend(to_add);

        // If turning on, add the new region with positive sign
        if matches!(cuboid.state, State::On) {
            regions.push((new_region, 1));
        }
    }

    // Sum all regions weighted by their sign
    regions
        .iter()
        .map(|(region, sign)| region.volume() * sign)
        .sum()
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
        assert_eq!(answer, 39);
        let input = std::fs::read_to_string("test_input1.txt").unwrap();
        let answer = solve_p1(&input);
        assert_eq!(answer, 590784);
        let input = std::fs::read_to_string("test_input2.txt").unwrap();
        let answer = solve_p2(&input);
        assert_eq!(answer, 2758514936282235);
    }
}
