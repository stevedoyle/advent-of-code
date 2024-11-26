use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone)]
enum PacketData {
    Number(i32),
    List(Vec<PacketData>),
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PacketData::Number(a), PacketData::Number(b)) => a.cmp(b),
            (PacketData::List(a), PacketData::List(b)) => a.cmp(b),
            (PacketData::Number(a), PacketData::List(b)) => vec![PacketData::Number(*a)].cmp(b),
            (PacketData::List(a), PacketData::Number(b)) => a.cmp(&vec![PacketData::Number(*b)]),
        }
    }
}

fn solve_p1(input: &str) -> usize {
    let pkts = parse_input(input);
    pkts.chunks_exact(2)
        .enumerate()
        .map(|(i, chunk)| if chunk[0] <= chunk[1] { i + 1 } else { 0 })
        .sum()
}

fn solve_p2(input: &str) -> usize {
    let mut pkts = parse_input(input);
    let divider1 = parse_packet("[[2]]");
    let divider2 = parse_packet("[[6]]");
    pkts.push(divider1.clone());
    pkts.push(divider2.clone());
    pkts.sort();
    pkts.iter()
        .enumerate()
        .filter(|(_, p)| **p == divider1 || **p == divider2)
        .fold(1, |acc, (i, _)| acc * (i + 1))
}

fn parse_input(input: &str) -> Vec<Vec<PacketData>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_packet)
        .collect()
}

fn parse_packet(s: &str) -> Vec<PacketData> {
    let mut data = Vec::new();
    let mut stack = Vec::new();
    let mut i = 0;
    // Ignore the first and last brackets
    let s = s.trim();
    let s = &s[1..s.len() - 1];
    while i < s.len() {
        match s.chars().nth(i).unwrap() {
            '[' => {
                stack.push(data);
                data = Vec::new();
                i += 1;
            }
            ']' => {
                let mut list = stack.pop().unwrap();
                list.push(PacketData::List(data));
                data = list;
                i += 1;
            }
            ',' => {
                i += 1;
            }
            _ => {
                let (number, j) = parse_number(s, i);
                data.push(PacketData::Number(number));
                i = j;
            }
        }
    }
    data
}

fn parse_number(s: &str, i: usize) -> (i32, usize) {
    let mut j = i;
    while j < s.len() && s.chars().nth(j).unwrap().is_ascii_digit() {
        j += 1;
    }
    (s[i..j].parse().unwrap(), j)
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

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 13);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 140);
    }

    #[test]
    fn test_parse_packet() {
        let packet = parse_packet("[1,2,3]");
        assert_eq!(
            packet,
            vec![
                PacketData::Number(1),
                PacketData::Number(2),
                PacketData::Number(3)
            ]
        );

        let packet = parse_packet("[1,[2,3]]");
        assert_eq!(
            packet,
            vec![
                PacketData::Number(1),
                PacketData::List(vec![PacketData::Number(2), PacketData::Number(3)])
            ]
        );

        let packet = parse_packet("[1,[2,[3,4]]]");
        assert_eq!(
            packet,
            vec![
                PacketData::Number(1),
                PacketData::List(vec![
                    PacketData::Number(2),
                    PacketData::List(vec![PacketData::Number(3), PacketData::Number(4)])
                ])
            ]
        );

        let packet = parse_packet("[1,[2,[3,[4,5]]]]");
        assert_eq!(
            packet,
            vec![
                PacketData::Number(1),
                PacketData::List(vec![
                    PacketData::Number(2),
                    PacketData::List(vec![
                        PacketData::Number(3),
                        PacketData::List(vec![PacketData::Number(4), PacketData::Number(5)])
                    ])
                ])
            ]
        );
    }

    #[test]
    fn test_parse_number() {
        let (number, j) = parse_number("123", 0);
        assert_eq!(number, 123);
        assert_eq!(j, 3);

        let (number, j) = parse_number("123", 1);
        assert_eq!(number, 23);
        assert_eq!(j, 3);

        let (number, j) = parse_number("123", 2);
        assert_eq!(number, 3);
        assert_eq!(j, 3);
    }
}
