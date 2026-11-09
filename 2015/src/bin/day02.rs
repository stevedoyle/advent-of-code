use aoc2015::*;

const DAY: u8 = 2;

struct Box {
    l: usize,
    w: usize,
    h: usize,
}

impl Box {
    fn from_str(s: &str) -> Self {
        let parts: Vec<usize> = s.split('x').map(|x| x.parse().unwrap()).collect();
        Self {
            l: parts[0],
            w: parts[1],
            h: parts[2],
        }
    }

    fn surface_area(&self) -> usize {
        2 * (self.l * self.w + self.w * self.h + self.h * self.l)
    }

    fn smallest_side_area(&self) -> usize {
        let mut sides = [self.l * self.w, self.w * self.h, self.h * self.l];
        sides.sort();
        sides[0]
    }

    fn volume(&self) -> usize {
        self.l * self.w * self.h
    }

    fn smallest_perimeter(&self) -> usize {
        let mut sides = [self.l, self.w, self.h];
        sides.sort();
        2 * (sides[0] + sides[1])
    }
}

fn parse_input(input: &str) -> Vec<Box> {
    input.lines().map(Box::from_str).collect()
}

fn solve_p1(input: &str) -> usize {
    let boxes = parse_input(input);
    boxes
        .iter()
        .map(|b| b.surface_area() + b.smallest_side_area())
        .sum()
}

fn solve_p2(input: &str) -> usize {
    let boxes = parse_input(input);
    boxes
        .iter()
        .map(|b| b.smallest_perimeter() + b.volume())
        .sum()
}

fn main() {
    run(DAY, solve_p1, solve_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve_p1("2x3x4"), 58);
        assert_eq!(solve_p1("1x1x10"), 43);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_p2("2x3x4"), 34);
        assert_eq!(solve_p2("1x1x10"), 14);
    }
}
