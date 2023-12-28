use gcd::Gcd;
use std::{fmt::Debug, fmt::Formatter, ops::RangeInclusive, str::FromStr};

// Solutions heavily influenced by
// https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2023/day24.rs

#[derive(Default, PartialEq, Eq, PartialOrd, Ord)]
struct Hailstone {
    x: isize,
    y: isize,
    z: isize,
    vx: isize,
    vy: isize,
    vz: isize,
}

impl Debug for Hailstone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {} @ {}, {}, {}",
            self.x, self.y, self.z, self.vx, self.vy, self.vz
        )
    }
}

impl FromStr for Hailstone {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (positions, vectors) = s.split_once('@').unwrap();
        let ps: Vec<isize> = positions
            .split(',')
            .map(|p| p.trim().parse().unwrap())
            .collect();
        let vs: Vec<isize> = vectors
            .split(',')
            .map(|p| p.trim().parse().unwrap())
            .collect();
        let hailstone = Hailstone {
            x: ps[0],
            y: ps[1],
            z: ps[2],
            vx: vs[0],
            vy: vs[1],
            vz: vs[2],
        };
        Ok(hailstone)
    }
}

#[derive(Debug)]
struct ParseError;

fn parse_input(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

// We find the intersection for each pair of hailstones by solving a pair of linear simultaneous
// equations in 2 unknowns:
//
// * `a` and `g` are the x positions of the pair of hailstones.
// * `b` and `h` are the y positions.
// * `d` and `j` are the x velocities.
// * `e` and `k` are the y velocities.
// * Let `t` and `u` be the times that the first and second hailstone respectively are at the
//   intersection point.
//
// Then we can write:
//
// * `a + dt = g + ju` => `dt - ju = g - a`
// * `b + et = h + ku` => `et - ku = h - b`
//
// In matrix form:
//
// ``
//     | d  -j ||t| = | g - a |
//     | e  -k ||u|   | h - b |
// ```
//
// Solve by finding the inverse of the 2x2 matrix and premultiplying both sides. The inverse is:
//
// ```
//    ______1______ | -k  j |
//    d(-k) - (-j)e | -e  d |
// ```
//
fn intersect(h1: &Hailstone, h2: &Hailstone, range: &RangeInclusive<i64>) -> bool {
    let a = h1.x as f64;
    let b = h1.y as f64;
    let d = h1.vx as f64;
    let e = h1.vy as f64;
    let g = h2.x as f64;
    let h = h2.y as f64;
    let j = h2.vx as f64;
    let k = h2.vy as f64;

    let determinant = e * j - d * k;
    if determinant == 0.0 {
        return false;
    }

    // Invert the 2x2 matrix then multiply by the respective columns to      find the times.
    let t = (j * (h - b) - k * (g - a)) / determinant;
    let u = (d * (h - b) - e * (g - a)) / determinant;

    let x = a + t * d;
    let y = b + t * e;

    t > 0.0 && u > 0.0 && bounds(&(x, y), range)
}

fn bounds(p: &(f64, f64), range: &RangeInclusive<i64>) -> bool {
    range.contains(&(p.0 as i64)) && range.contains(&(p.1 as i64))
}

fn solve_p1(input: &str, range: RangeInclusive<i64>) -> usize {
    let hailstones = parse_input(input);
    let mut result = 0usize;
    for h1 in 1..hailstones.len() {
        for h2 in 0..h1 {
            if intersect(&hailstones[h1], &hailstones[h2], &range) {
                result += 1;
            }
        }
    }
    result
}

fn solve_p2(input: &str) -> i128 {
    let hailstones = parse_input(input);

    // Find the intersection point of the first threee hailstones.
    let a = hailstones[0].x as i128;
    let b = hailstones[0].y as i128;
    let c = hailstones[0].z as i128;
    let d = hailstones[0].vx as i128;
    let e = hailstones[0].vy as i128;
    let f = hailstones[0].vz as i128;

    let g = hailstones[1].x as i128;
    let h = hailstones[1].y as i128;
    let i = hailstones[1].z as i128;
    let j = hailstones[1].vx as i128;
    let k = hailstones[1].vy as i128;
    let l = hailstones[1].vz as i128;

    let m = hailstones[2].x as i128;
    let n = hailstones[2].y as i128;
    let o = hailstones[2].z as i128;
    let p = hailstones[2].vx as i128;
    let q = hailstones[2].vy as i128;
    let r = hailstones[2].vz as i128;

    // Coefficients for the 6 simulataneous linear equations.
    // Columns are px, py, pz, vx, vy, vz of the rock equal to a constant.
    let mut matrix = [
        [
            0,
            l - f,
            e - k,
            0,
            c - i,
            h - b,
            e * c - b * f + h * l - k * i,
        ],
        [
            0,
            r - f,
            e - q,
            0,
            c - o,
            n - b,
            e * c - b * f + n * r - q * o,
        ],
        [
            f - l,
            0,
            j - d,
            i - c,
            0,
            a - g,
            a * f - d * c + j * i - g * l,
        ],
        [
            f - r,
            0,
            p - d,
            o - c,
            0,
            a - m,
            a * f - d * c + p * o - m * r,
        ],
        [
            k - e,
            d - j,
            0,
            b - h,
            g - a,
            0,
            d * b - a * e + g * k - j * h,
        ],
        [
            q - e,
            d - p,
            0,
            b - n,
            m - a,
            0,
            d * b - a * e + m * q - p * n,
        ],
    ];

    // Use Gaussian elimination to solve for the 6 unknowns.
    // Forward elimination, processing columns from left to right.
    // This will leave a matrix in row echelon form.
    for pivot in 0..6 {
        // Make leading coefficient of each row positive to make subsequent calculations easier.
        for row in &mut matrix[pivot..] {
            if row[pivot] < 0 {
                // Flip signs of each coefficient.
                row.iter_mut().for_each(|n| *n = -*n);
            }
        }

        loop {
            // Reduce by GCD each time otherwise coefficients will overflow even a `i128`.
            for row in &mut matrix[pivot..] {
                let mut factor: u128 = 0;

                for &next in &row[pivot..] {
                    if next != 0 {
                        if factor == 0 {
                            factor = next.unsigned_abs();
                        } else {
                            factor = factor.gcd(next.unsigned_abs());
                        }
                    }
                }

                row[pivot..].iter_mut().for_each(|c| *c /= factor as i128);
            }

            let column = matrix.map(|row| row[pivot]);

            // If only one non-zero coefficient remaining in the column then we're done.
            if column[pivot..].iter().filter(|&&c| c > 0).count() == 1 {
                // Move this row into the pivot location
                let index = column.iter().rposition(|&c| c > 0).unwrap();
                matrix.swap(pivot, index);
                break;
            }

            // Find the row with the lowest non-zero leading coefficient.
            let min = *column[pivot..].iter().filter(|&&c| c > 0).min().unwrap();
            let index = column.iter().rposition(|&c| c == min).unwrap();

            // Subtract as many multiples of this minimum row from each other row as possible
            // to shrink the coefficients of our column towards zero.
            for row in pivot..6 {
                if row != index && column[row] != 0 {
                    let factor = column[row] / min;

                    for col in pivot..7 {
                        matrix[row][col] -= factor * matrix[index][col];
                    }
                }
            }
        }
    }

    // Back substitution, processing columns from right to left.
    // This will leave the matrix in reduced row echelon form.
    // The solved unknowns are then in the 7th column.
    for pivot in (0..6).rev() {
        // We're explicitly told that the results are integers so integer division is safe
        // and will not mangle result.
        matrix[pivot][6] /= matrix[pivot][pivot];

        for row in 0..pivot {
            matrix[row][6] -= matrix[pivot][6] * matrix[row][pivot];
        }
    }

    // x + y + z
    matrix[0][6] + matrix[1][6] + matrix[2][6]
}

fn main() {
    let input = include_str!("../input.txt");
    let range = 200_000_000_000_000..=400_000_000_000_000;
    let answer = solve_p1(input, range);
    println!("Part 1: {answer}");
    let answer = solve_p2(input);
    println!("Part 2: {answer:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    19, 13, 30 @ -2,  1, -2
    18, 19, 22 @ -1, -1, -2
    20, 25, 34 @ -2, -2, -4
    12, 31, 28 @ -1, -2, -1
    20, 19, 15 @  1, -5, -3";

    #[test]
    fn test_parse_input() {
        let hailstones = parse_input(INPUT);
        assert_eq!(hailstones.len(), 5);
        assert_eq!(
            hailstones[0],
            Hailstone {
                x: 19,
                y: 13,
                z: 30,
                vx: -2,
                vy: 1,
                vz: -2
            }
        );
        assert_eq!(
            hailstones[4],
            Hailstone {
                x: 20,
                y: 19,
                z: 15,
                vx: 1,
                vy: -5,
                vz: -3
            }
        );
    }

    #[test]
    fn test_intersect() {
        let h1 = Hailstone::from_str("19, 13, 30 @ -2,  1, -2").unwrap();
        let h2 = Hailstone::from_str("18, 19, 22 @ -1, -1, -2").unwrap();
        let result = intersect(&h2, &h1, &(7..=27));
        assert!(result);
    }

    #[test]
    fn test_solve_p1() {
        let answer = solve_p1(INPUT, 7..=27);
        assert_eq!(answer, 2)
    }

    #[test]
    fn test_solve_p2() {
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 24 + 13 + 10);
    }

    #[test]
    fn test_solve() {
        let input = include_str!("../input.txt");
        let range = 200_000_000_000_000..=400_000_000_000_000;
        let answer = solve_p1(input, range);
        assert_eq!(answer, 19523);
        let answer = solve_p2(input);
        assert_eq!(answer, 566373506408017);
    }
}
