fn sig_strength_adder(x: i32, cycle: i32) -> i32 {
    if cycle >= 20 && (cycle - 20) % 40 == 0 {
        return x * cycle;
    }
    0
}

fn solve_p1(input: &str) -> i32 {
    let mut x = 1;
    let mut cycle = 0;
    let mut signal_strength = 0;

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let op = parts.next().unwrap();

        match op {
            "addx" => {
                let arg = parts.next().unwrap().parse::<i32>().unwrap();
                cycle += 1;
                signal_strength += sig_strength_adder(x, cycle);
                cycle += 1;
                signal_strength += sig_strength_adder(x, cycle);
                x += arg;
            }
            "noop" => {
                cycle += 1;
                signal_strength += sig_strength_adder(x, cycle);
            }
            _ => panic!("Unknown op: {op}"),
        }
    }
    signal_strength
}

fn pixel_value(x: i32, cycle: i32) -> char {
    let cursor = (cycle - 1) % (40 * 6);
    let col = cursor % 40;

    if (0.max(col - 1)..=col + 1).contains(&x) {
        return '#';
    }
    '.'
}

fn solve_p2(input: &str) -> String {
    let mut x = 1;
    let mut cycle = 0;
    //let mut screen = vec![vec!['.'; 40]; 6];
    let mut screen = vec![];

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let op = parts.next().unwrap();

        match op {
            "addx" => {
                let arg = parts.next().unwrap().parse::<i32>().unwrap();
                cycle += 1;
                screen.push(pixel_value(x, cycle));
                cycle += 1;
                screen.push(pixel_value(x, cycle));
                x += arg;
            }
            "noop" => {
                cycle += 1;
                screen.push(pixel_value(x, cycle));
            }
            _ => panic!("Unknown op: {op}"),
        }
    }
    let result = screen
        .chunks(40)
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    result
}

fn main() {
    let input = include_str!("../input.txt");
    let answer = solve_p1(input);
    println!("Part 1: {answer}");
    let answer = solve_p2(input);
    println!("Part 2:\n{answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 13140);
        let answer = solve_p2(INPUT);
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(answer, expected);
    }
}
