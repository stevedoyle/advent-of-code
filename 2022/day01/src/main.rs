fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .filter(|line| !line.is_empty())
                .map(|entry| entry.trim().parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

fn solve_p1(input: &str) -> usize {
    let list = parse_input(input);
    list.iter().map(|inner| inner.iter().sum()).max().unwrap()
}

fn solve_p2(input: &str) -> usize {
    let list = parse_input(input);
    let mut totals: Vec<usize> = list.iter().map(|inner| inner.iter().sum()).collect();
    totals.sort();
    totals.iter().rev().take(3).sum()
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

    const INPUT: &str = "
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000";

    #[test]
    fn test_parse_input() {
        let calorie_list = parse_input(INPUT);
        assert_eq!(calorie_list.len(), 5);
        assert_eq!(calorie_list[0].len(), 3);
        assert_eq!(calorie_list[4].len(), 1);
        assert_eq!(calorie_list[4][0], 10000);
    }

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 24000);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 45000);
    }

    #[test]
    fn test_solve() {
        let input = include_str!("../input.txt");
        let answer = solve_p1(input);
        assert_eq!(answer, 70720);
        let answer = solve_p2(input);
        assert_eq!(answer, 207148);
    }
}
