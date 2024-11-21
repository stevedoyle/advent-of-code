use std::collections::HashSet;

fn solve_p1(input: &str) -> usize {
    find_substring(input, 4)
}

fn solve_p2(input: &str) -> usize {
    find_substring(input, 14)
}

fn find_substring(input: &str, length: usize) -> usize {
    let chars = input.chars().collect::<Vec<char>>();
    for i in (length - 1)..input.len() {
        let s: HashSet<char> = HashSet::from_iter(chars[(i - (length - 1))..=i].iter().cloned());
        if s.len() == length {
            return i + 1;
        }
    }
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

    #[test]
    fn test_solve_p1() {
        assert_eq!(solve_p1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(solve_p1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve_p1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve_p1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve_p1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_solve_p2() {
        assert_eq!(solve_p2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solve_p2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solve_p2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(solve_p2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(solve_p2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
