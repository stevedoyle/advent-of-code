use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::ops::Range;

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    range: i32,
}

impl Sensor {
    fn new(x: i32, y: i32, range: i32) -> Self {
        Self { x, y, range }
    }

    fn in_range(&self, x: i32, y: i32) -> bool {
        manhattan_distance(self.x, self.y, x, y) <= self.range
    }

    /// Returns 4 lines that represent the perimiter of the sensor's range. The perimiter is
    /// defined as just beyond the range of the sensor so any coordinates that lie on the
    /// perimiter are outside of the sensor's range.
    fn perimiter(&self) -> Vec<Line> {
        let a = Line::new((self.x, self.y + self.range + 1), 1);
        let b = Line::new((self.x, self.y - self.range - 1), 1);
        let c = Line::new((self.x - self.range - 1, self.y), -1);
        let d = Line::new((self.x + self.range + 1, self.y), -1);
        vec![a, b, c, d]
    }

    fn coverage_on_row(&self, row: i32) -> Range<i32> {
        let y_dist = (self.y - row).abs();
        if y_dist > self.range {
            return 0..0;
        }
        let x_dist = self.range - y_dist;

        self.x - x_dist..self.x + x_dist + 1
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Line {
    slope: i32,
    y_intercept: i32,
}

impl Line {
    fn new(point: (i32, i32), slope: i32) -> Self {
        let y_intercept = point.1 - slope * point.0;
        Self { slope, y_intercept }
    }
}

fn manhattan_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn is_uncovered(point: (i32, i32), sensors: &[Sensor]) -> bool {
    sensors
        .iter()
        .all(|sensor| !sensor.in_range(point.0, point.1))
}

fn parse_input(input: &str) -> (Vec<Sensor>, Vec<(i32, i32)>) {
    let mut sensors = Vec::new();
    let mut beacons = Vec::new();

    let re = Regex::new(r".+x=(-?\d+), y=(-?\d+): .+x=(-?\d+), y=(-?\d+)").unwrap();

    input.lines().for_each(|line| {
        let parts = re.captures(line).unwrap();
        let x_sensor = parts.get(1).unwrap().as_str().parse().unwrap();
        let y_sensor = parts.get(2).unwrap().as_str().parse().unwrap();
        let x_beacon = parts.get(3).unwrap().as_str().parse().unwrap();
        let y_beacon = parts.get(4).unwrap().as_str().parse().unwrap();
        sensors.push(Sensor::new(
            x_sensor,
            y_sensor,
            manhattan_distance(x_sensor, y_sensor, x_beacon, y_beacon),
        ));
        beacons.push((x_beacon, y_beacon));
    });
    (sensors, beacons)
}

fn solve_p1(input: &str, target_row: i32) -> i32 {
    let (sensors, beacons) = parse_input(input);
    let mut covered = Vec::new();
    sensors.iter().for_each(|sensor| {
        covered.push(sensor.coverage_on_row(target_row));
    });
    let mut covered: HashSet<i32> = covered.iter().flat_map(|range| range.clone()).collect();
    beacons
        .iter()
        .filter(|beacon| beacon.1 == target_row)
        .for_each(|beacon| {
            covered.remove(&beacon.0);
        });
    covered.len() as i32
}

fn solve_p2(input: &str, x_bound: usize, y_bound: usize) -> usize {
    let (sensors, _) = parse_input(input);
    let lines = sensors
        .iter()
        .flat_map(|sensor| sensor.perimiter())
        .collect::<Vec<_>>();
    let mut line_counts = HashMap::new(); // Line -> count
    lines.iter().for_each(|line| {
        line_counts
            .entry(line)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });
    // Ignore lines that only appear once as we are looking for a point that is intersected by 4
    // lines (2 rising and 2 falling).
    let lines = line_counts
        .iter()
        .filter(|(_, count)| **count > 1)
        .map(|(line, _)| line)
        .collect::<Vec<_>>();

    let intersection_points = lines
        .iter()
        .flat_map(|line1| {
            lines.iter().filter_map(move |line2| {
                if line1.slope == line2.slope {
                    return None;
                }
                let x = (line2.y_intercept - line1.y_intercept) / (line1.slope - line2.slope);
                let y = line1.slope * x + line1.y_intercept;
                if x < 0 || x >= x_bound as i32 || y < 0 || y >= y_bound as i32 {
                    return None;
                }
                Some((x, y))
            })
        })
        .collect::<Vec<_>>();

    for point in &intersection_points {
        if is_uncovered(*point, &sensors) {
            return point.0 as usize * 4_000_000 + point.1 as usize;
        }
    }
    0
}

fn main() {
    let input = include_str!("../input.txt");
    let answer = solve_p1(input, 2_000_000);
    println!("Part 1: {answer}");
    let answer = solve_p2(input, 4_000_000, 4_000_000);
    println!("Part 2: {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT, 10);
        assert_eq!(answer, 26);
        let answer = solve_p2(INPUT, 20, 20);
        assert_eq!(answer, 56000011);
    }
}
