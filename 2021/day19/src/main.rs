use std::collections::HashSet;

type Point3D = (i32, i32, i32);

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Point3D>,
}

// Generate all 24 possible rotations of a 3D point
#[inline]
fn rotate_point((x, y, z): Point3D, rotation: usize) -> Point3D {
    match rotation {
        0 => (x, y, z),
        1 => (x, z, -y),
        2 => (x, -y, -z),
        3 => (x, -z, y),
        4 => (-x, -y, z),
        5 => (-x, z, y),
        6 => (-x, y, -z),
        7 => (-x, -z, -y),
        8 => (y, -x, z),
        9 => (y, z, x),
        10 => (y, x, -z),
        11 => (y, -z, -x),
        12 => (-y, x, z),
        13 => (-y, z, -x),
        14 => (-y, -x, -z),
        15 => (-y, -z, x),
        16 => (z, y, -x),
        17 => (z, -x, -y),
        18 => (z, -y, x),
        19 => (z, x, y),
        20 => (-z, -y, -x),
        21 => (-z, -x, y),
        22 => (-z, y, x),
        23 => (-z, x, -y),
        _ => panic!("Invalid rotation index"),
    }
}

#[inline]
fn try_match_scanners(scanner1: &Scanner, scanner2: &Scanner) -> Option<(Point3D, Scanner)> {
    // Convert scanner1 beacons to HashSet for O(1) lookup
    let scanner1_set: HashSet<Point3D> = scanner1.beacons.iter().cloned().collect();

    for rotation in 0..24 {
        // Rotate beacons directly without creating intermediate Scanner
        let rotated_beacons: Vec<Point3D> = scanner2
            .beacons
            .iter()
            .map(|&beacon| rotate_point(beacon, rotation))
            .collect();

        // Try pairs of beacons as potential matches (limit to first 15 for efficiency)
        for &beacon1 in scanner1.beacons.iter().take(15) {
            for &beacon2 in rotated_beacons.iter().take(15) {
                let translation = (
                    beacon1.0 - beacon2.0,
                    beacon1.1 - beacon2.1,
                    beacon1.2 - beacon2.2,
                );

                // Apply translation and count overlaps efficiently with early termination
                let mut overlap_count = 0;
                let mut checked = 0;
                for &rotated_beacon in &rotated_beacons {
                    let transformed_beacon = (
                        rotated_beacon.0 + translation.0,
                        rotated_beacon.1 + translation.1,
                        rotated_beacon.2 + translation.2,
                    );

                    if scanner1_set.contains(&transformed_beacon) {
                        overlap_count += 1;
                        if overlap_count >= 12 {
                            // Early termination - we found enough overlaps
                            let transformed_beacons: Vec<Point3D> = rotated_beacons
                                .iter()
                                .map(|&(x, y, z)| {
                                    (x + translation.0, y + translation.1, z + translation.2)
                                })
                                .collect();
                            return Some((
                                translation,
                                Scanner {
                                    beacons: transformed_beacons,
                                },
                            ));
                        }
                    }

                    checked += 1;
                    // Early exit if we can't possibly reach 12 overlaps
                    if overlap_count + (rotated_beacons.len() - checked) < 12 {
                        break;
                    }
                }
            }
        }
    }
    None
}

fn parse_input(input: &str) -> Vec<Scanner> {
    let mut scanners = Vec::new();
    let mut current_beacons = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if line.starts_with("---") {
            if !current_beacons.is_empty() {
                scanners.push(Scanner {
                    beacons: current_beacons,
                });
                current_beacons = Vec::new();
            }
        } else {
            let coords: Vec<i32> = line.split(",").map(|s| s.trim().parse().unwrap()).collect();
            current_beacons.push((coords[0], coords[1], coords[2]));
        }
    }

    if !current_beacons.is_empty() {
        scanners.push(Scanner {
            beacons: current_beacons,
        });
    }

    scanners
}

fn solve_both_parts(input: &str) -> (usize, i32) {
    let mut scanners = parse_input(input);

    if scanners.is_empty() {
        return (0, 0);
    }

    let mut aligned_scanners = vec![scanners.remove(0)]; // Start with scanner 0 as reference
    let mut scanner_positions = vec![(0, 0, 0)]; // Position of scanner 0
    let mut unique_beacons = HashSet::new();

    // Add beacons from the first scanner
    for beacon in &aligned_scanners[0].beacons {
        unique_beacons.insert(*beacon);
    }

    while !scanners.is_empty() {
        let mut matched = false;

        for i in 0..scanners.len() {
            let mut found_match = false;

            // Try to match against any aligned scanner
            for aligned_scanner in &aligned_scanners {
                if let Some((translation, transformed_scanner)) =
                    try_match_scanners(aligned_scanner, &scanners[i])
                {
                    // Add beacons from the newly aligned scanner
                    for beacon in &transformed_scanner.beacons {
                        unique_beacons.insert(*beacon);
                    }

                    aligned_scanners.push(transformed_scanner);
                    scanner_positions.push(translation);
                    scanners.remove(i);
                    found_match = true;
                    matched = true;
                    break;
                }
            }

            if found_match {
                break;
            }
        }

        if !matched {
            // Unable to align remaining scanners
            break;
        }
    }

    // Calculate part 2: largest Manhattan distance
    let mut max_distance = 0;
    for i in 0..scanner_positions.len() {
        for j in i + 1..scanner_positions.len() {
            let pos1 = scanner_positions[i];
            let pos2 = scanner_positions[j];
            let distance =
                (pos1.0 - pos2.0).abs() + (pos1.1 - pos2.1).abs() + (pos1.2 - pos2.2).abs();
            max_distance = max_distance.max(distance);
        }
    }

    (unique_beacons.len(), max_distance)
}

fn solve_p1(input: &str) -> usize {
    solve_both_parts(input).0
}

fn solve_p2(input: &str) -> i32 {
    solve_both_parts(input).1
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
        assert_eq!(answer, 79);
        let answer = solve_p2(&input);
        assert_eq!(answer, 3621);
    }
}
