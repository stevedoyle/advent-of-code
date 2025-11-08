struct TargetArea {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

fn parse_input(input: &str) -> TargetArea {
    // Example input: "target area: x=20..30, y=-10..-5"
    let (x_range, y_range) = input
        .trim()
        .strip_prefix("target area: ")
        .unwrap()
        .split_once(", ")
        .unwrap();
    let (xmin, xmax) = x_range
        .trim()
        .strip_prefix("x=")
        .unwrap()
        .split_once("..")
        .unwrap();
    let (ymin, ymax) = y_range
        .trim()
        .strip_prefix("y=")
        .unwrap()
        .split_once("..")
        .unwrap();
    let (xmin, xmax, ymin, ymax) = (
        xmin.parse().unwrap(),
        xmax.parse().unwrap(),
        ymin.parse().unwrap(),
        ymax.parse().unwrap(),
    );
    TargetArea {
        xmin,
        xmax,
        ymin,
        ymax,
    }
}

fn in_target_area(pos: (i32, i32), target: &TargetArea) -> bool {
    pos.0 >= target.xmin && pos.0 <= target.xmax && pos.1 >= target.ymin && pos.1 <= target.ymax
}

fn beyond_target_area(pos: (i32, i32), target: &TargetArea) -> bool {
    pos.0 > target.xmax || pos.1 < target.ymin
}

fn solve_p1(input: &str) -> i32 {
    let target = parse_input(input);
    let start = (0, 0);
    let mut highest_y = 0;
    // Upper bound for vy: when probe returns to y=0, velocity is -vy
    // Next step will be at y = -(vy + 1), which must be >= ymin
    // So vy <= -ymin - 1
    let max_vy = -target.ymin - 1;
    for vx in 1..=target.xmax {
        for vy in target.ymin..=max_vy {
            let mut pos = start;
            let mut max_y = 0;
            let mut vx_sim = vx;
            let mut vy_sim = vy;
            while pos.0 <= target.xmax && pos.1 >= target.ymin {
                pos.0 += vx_sim;
                pos.1 += vy_sim;
                max_y = max_y.max(pos.1);
                if in_target_area(pos, &target) {
                    highest_y = highest_y.max(max_y);
                    break;
                }
                if beyond_target_area(pos, &target) {
                    break;
                }
                vx_sim = (vx_sim - 1).max(0);
                vy_sim -= 1;
            }
        }
    }
    highest_y
}

fn solve_p2(input: &str) -> usize {
    let target = parse_input(input);
    let start = (0, 0);
    let mut hits = 0;
    // Upper bound for vy: when probe returns to y=0, velocity is -vy
    // Next step will be at y = -(vy + 1), which must be >= ymin
    // So vy <= -ymin - 1
    let max_vy = -target.ymin - 1;
    for vx in 1..=target.xmax {
        for vy in target.ymin..=max_vy {
            let mut pos = start;
            let mut max_y = 0;
            let mut vx_sim = vx;
            let mut vy_sim = vy;
            while pos.0 <= target.xmax && pos.1 >= target.ymin {
                pos.0 += vx_sim;
                pos.1 += vy_sim;
                max_y = max_y.max(pos.1);
                if in_target_area(pos, &target) {
                    hits += 1;
                    break;
                }
                if beyond_target_area(pos, &target) {
                    break;
                }
                vx_sim = (vx_sim - 1).max(0);
                vy_sim -= 1;
            }
        }
    }
    hits
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

    const TEST_INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_solve_with_test_input() {
        let input = TEST_INPUT;
        let answer = solve_p1(&input);
        assert_eq!(answer, 45);
        let answer = solve_p2(&input);
        assert_eq!(answer, 112);
    }
}
