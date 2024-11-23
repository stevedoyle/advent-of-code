use std::collections::HashSet;

fn solve_p1(input: &str) -> i32 {
    let mut rope = vec![(0, 0); 2];
    solve(input, &mut rope)
}

fn solve_p2(input: &str) -> i32 {
    let mut rope = vec![(0, 0); 10];
    solve(input, &mut rope)
}

fn solve(input: &str, rope: &mut [(i32, i32)]) -> i32 {
    let mut visited = HashSet::new();

    visited.insert(*rope.last().unwrap());

    for line in input.lines() {
        let (dir, dist) = line.split_once(' ').unwrap();
        let dist = dist.parse::<i32>().unwrap();

        match dir {
            "R" => visited.extend(move_rope_right(rope, dist)),
            "L" => visited.extend(move_rope_left(rope, dist)),
            "U" => visited.extend(move_rope_up(rope, dist)),
            "D" => visited.extend(move_rope_down(rope, dist)),
            _ => panic!("Unknown direction"),
        }
    }
    visited.len() as i32
}

fn is_touching(head: (i32, i32), tail: (i32, i32)) -> bool {
    (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1
}

fn move_rope_right(rope: &mut [(i32, i32)], dist: i32) -> HashSet<(i32, i32)> {
    (0..dist)
        .flat_map(|_| move_rope_one_step(rope, (0, 1)).into_iter())
        .collect()
}

fn move_rope_left(rope: &mut [(i32, i32)], dist: i32) -> HashSet<(i32, i32)> {
    (0..dist)
        .flat_map(|_| move_rope_one_step(rope, (0, -1)).into_iter())
        .collect()
}

fn move_rope_up(rope: &mut [(i32, i32)], dist: i32) -> HashSet<(i32, i32)> {
    (0..dist)
        .flat_map(|_| move_rope_one_step(rope, (1, 0)).into_iter())
        .collect()
}

fn move_rope_down(rope: &mut [(i32, i32)], dist: i32) -> HashSet<(i32, i32)> {
    (0..dist)
        .flat_map(|_| move_rope_one_step(rope, (-1, 0)).into_iter())
        .collect()
}

fn move_rope_one_step(rope: &mut [(i32, i32)], dist: (i32, i32)) -> HashSet<(i32, i32)> {
    let mut visited = HashSet::new();

    let mut head = rope[0];
    head.0 += dist.0;
    head.1 += dist.1;
    rope[0] = head;

    let mut prev_knot = rope[0];

    for knot in rope.iter_mut().skip(1) {
        if is_touching(prev_knot, *knot) {
            break;
        }

        let diff = (prev_knot.0 - knot.0, prev_knot.1 - knot.1);

        match diff.0.cmp(&0) {
            std::cmp::Ordering::Greater => knot.0 += 1,
            std::cmp::Ordering::Less => knot.0 -= 1,
            std::cmp::Ordering::Equal => (),
        }

        match diff.1.cmp(&0) {
            std::cmp::Ordering::Greater => knot.1 += 1,
            std::cmp::Ordering::Less => knot.1 -= 1,
            std::cmp::Ordering::Equal => (),
        }

        prev_knot = *knot;
    }
    visited.insert(*rope.last().unwrap());
    visited
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

    #[test]
    fn test_solve_with_test_input() {
        const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 13);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_solve_p2_with_test_input() {
        const INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 36);
    }

    #[test]
    fn test_move_rope_one_step() {
        let mut rope = vec![(0, 0); 2];
        move_rope_one_step(&mut rope, (1, 0));
        assert_eq!(rope, vec![(1, 0), (0, 0)]);

        move_rope_one_step(&mut rope, (0, 1));
        assert_eq!(rope, vec![(1, 1), (0, 0)]);

        move_rope_one_step(&mut rope, (0, 1));
        assert_eq!(rope, vec![(1, 2), (1, 1)]);

        move_rope_one_step(&mut rope, (0, 1));
        assert_eq!(rope, vec![(1, 3), (1, 2)]);

        move_rope_one_step(&mut rope, (1, 0));
        assert_eq!(rope, vec![(2, 3), (1, 2)]);

        move_rope_one_step(&mut rope, (1, 0));
        assert_eq!(rope, vec![(3, 3), (2, 3)]);

        move_rope_one_step(&mut rope, (-1, 0));
        assert_eq!(rope, vec![(2, 3), (2, 3)]);

        move_rope_one_step(&mut rope, (-1, 0));
        assert_eq!(rope, vec![(1, 3), (2, 3)]);

        move_rope_one_step(&mut rope, (0, -1));
        assert_eq!(rope, vec![(1, 2), (2, 3)]);

        move_rope_one_step(&mut rope, (0, -1));
        assert_eq!(rope, vec![(1, 1), (1, 2)]);
    }
}
