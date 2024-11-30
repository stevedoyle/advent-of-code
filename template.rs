fn solve_p1(input: &str) -> i32 {
    0
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

    const INPUT: &str = "";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 157);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 0);
    }
}
