use maplit::hashmap;

#[derive(Debug)]
struct Snafu {
    value: isize,
}

impl std::str::FromStr for Snafu {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = hashmap! {'=' => -2, '-' => -1, '0' => 0, '1' => 1, '2' => 2};
        let value = s.chars().fold(0, |acc, c| (acc * 5) + map[&c]);
        Ok(Snafu { value })
    }
}

fn to_snafu(value: isize) -> String {
    let digits = ["=", "-", "0", "1", "2"];
    let mut encoded = String::new();
    let mut value = value;
    while value != 0 {
        let idx = (value + 2) % 5;
        let c = digits[idx as usize];
        encoded.push_str(c);
        if idx < 2 {
            value += 5;
        }
        value /= 5;
    }
    encoded.chars().rev().collect()
}

fn parse_input(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|line| line.parse::<Snafu>().unwrap().value)
        .collect()
}

fn solve_p1(input: &str) -> String {
    let data = parse_input(input);
    let value = data.iter().sum();
    to_snafu(value)
}

fn solve_p2(_input: &str) -> i32 {
    0
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

    const INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, "2=-1=0");
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_parse_input() {
        assert_eq!("21".parse::<Snafu>().unwrap().value, 11);
        assert_eq!("1=-0-2".parse::<Snafu>().unwrap().value, 1747);
        assert_eq!("1121-1110-1=0".parse::<Snafu>().unwrap().value, 314159265);
    }

    #[test]
    fn test_snafu_display() {
        assert_eq!(to_snafu(11).to_string(), "21");
        assert_eq!(to_snafu(1747).to_string(), "1=-0-2");
        assert_eq!(to_snafu(314159265).to_string(), "1121-1110-1=0");
    }
}
