use aoc2015::*;

const DAY: u8 = 5;

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn is_nice_p1(s: &str) -> bool {
    let vowels = "aeiou";
    let vowel_count = s.chars().filter(|c| vowels.contains(*c)).count();
    let has_three_vowels = vowel_count >= 3;
    let has_double = s.chars().zip(s.chars().skip(1)).any(|(a, b)| a == b);
    let forbidden = ["ab", "cd", "pq", "xy"];
    let has_forbidden = forbidden.iter().any(|&f| s.contains(f));
    has_three_vowels && has_double && !has_forbidden
}

fn is_nice_p2(s: &str) -> bool {
    let has_pair = s.chars().zip(s.chars().skip(1)).enumerate().any(|(i, (a, b))| {
        s.chars().skip(i + 2).zip(s.chars().skip(i + 3)).any(|(c, d)| a == c && b == d)
    });
    let has_repeat = s.chars().zip(s.chars().skip(2)).any(|(a, b)| a == b);
    has_pair && has_repeat
}

fn solve_p1(input: &str) -> usize {
    let data = parse_input(input);
    data.iter().filter(|&&s| is_nice_p1(s)).count()
}

fn solve_p2(input: &str) -> usize {
    let data = parse_input(input);
    data.iter().filter(|&&s| is_nice_p2(s)).count()
}

fn main() {
    run(DAY, solve_p1, solve_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve_p1("ugknbfddgicrmopn"), 1);
        assert_eq!(solve_p1("aaa"), 1);
        assert_eq!(solve_p1("jchzalrnumimnmhp"), 0);
        assert_eq!(solve_p1("haegwjzuvuyypxyu"), 0);
        assert_eq!(solve_p1("dvszwmarrgswjxmb"), 0);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_p2("qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(solve_p2("xxyxx"), 1);
        assert_eq!(solve_p2("uurcxstgmygtbstg"), 0);
        assert_eq!(solve_p2("ieodomkazucvgmuy"), 0);
    }
}
