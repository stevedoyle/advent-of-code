use std::{collections::HashMap, error::Error, fs};

fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| String::from(line.trim()))
        .collect()
}

fn parse_row(row: &str) -> (String, Vec<usize>) {
    let (springs, rhs) = row.split_once(' ').unwrap();
    let target = rhs
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    (String::from(springs), target)
}

fn get_solution_count(springs: &str, target: &[usize]) -> usize {
    let mut cache: HashMap<(Vec<char>, usize, usize), usize> = HashMap::new();
    let mut spring_chars: Vec<char> = springs.chars().collect();
    spring_chars.push('.');
    get_number_of_possible_solutions(&spring_chars, target, 0, &mut cache)
}

fn get_number_of_possible_solutions(
    word: &[char],
    constraints: &[usize],
    damaged_in_group: usize,
    cache: &mut HashMap<(Vec<char>, usize, usize), usize>,
) -> usize {
    if let Some(x) = cache.get(&(word.to_vec(), constraints.len(), damaged_in_group)) {
        return *x;
    }
    let res = get_number_of_possible_solutions_memoized(word, constraints, damaged_in_group, cache);
    cache.insert((word.to_vec(), constraints.len(), damaged_in_group), res);
    res
}

fn get_number_of_possible_solutions_memoized(
    word: &[char],
    constraints: &[usize],
    damaged_in_group: usize,
    cache: &mut HashMap<(Vec<char>, usize, usize), usize>,
) -> usize {
    if word.is_empty() {
        if constraints.is_empty() && damaged_in_group == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    let damaged = word.iter().filter(|c| **c == '#').count();
    let total_damaged_remaining = constraints.iter().sum();
    if damaged > total_damaged_remaining {
        return 0;
    }

    match word[0] {
        '?' => ['.', '#'].into_iter().fold(0, |acc, new_letter| {
            let mut new_word: Vec<char> = vec![new_letter];
            new_word.append(&mut word[1..].to_vec());
            acc + get_number_of_possible_solutions(&new_word, constraints, damaged_in_group, cache)
        }),
        '.' => {
            let mut t = 0;
            if !constraints.is_empty() && constraints[0] == damaged_in_group {
                t = get_number_of_possible_solutions(&word[1..], &constraints[1..], 0, cache);
            }
            if damaged_in_group == 0 {
                t += get_number_of_possible_solutions(&word[1..], constraints, 0, cache)
            }
            t
        }
        '#' => {
            get_number_of_possible_solutions(&word[1..], constraints, damaged_in_group + 1, cache)
        }
        _ => panic!("Unsupported character"),
    }
}

fn unfold(pair: &(String, Vec<usize>)) -> (String, Vec<usize>) {
    let mut unfolded_str: String = String::from(&pair.0);
    let mut unfolded_target: Vec<usize> = pair.1.to_vec();
    for _ in 0..4 {
        unfolded_str.push('?');
        unfolded_str.push_str(&pair.0);
        unfolded_target.extend_from_slice(&pair.1);
    }
    (unfolded_str, unfolded_target)
}

fn solve_p1(input: &str) -> usize {
    let rows = parse_input(input);
    rows.iter()
        .map(|row| {
            let (springs, target) = parse_row(row);
            get_solution_count(&springs, &target)
        })
        .sum()
}

fn solve_p2(input: &str) -> usize {
    let rows = parse_input(input);
    rows.iter()
        .map(|row| {
            let (springs, target) = unfold(&parse_row(row));
            get_solution_count(&springs, &target)
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let sum_of_counts = solve_p1(&input);
    println!("Part 1: {sum_of_counts}");

    let sum_of_counts = solve_p2(&input);
    println!("Part 2: {sum_of_counts}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1",
        )
    }

    #[test]
    fn test_parsing() {
        let rows = parse_input(&get_input());
        assert_eq!(rows.len(), 6);
        let (springs, target) = parse_row(&rows[0]);
        assert_eq!(springs, "???.###");
        assert_eq!(target, vec![1, 1, 3]);

        let (springs, target) = parse_row(&rows[1]);
        assert_eq!(springs, ".??..??...?##.");
        assert_eq!(target, vec![1, 1, 3]);
    }

    #[test]
    fn test_get_solution_count() {
        let (springs, target) = parse_row("???.### 1,1,3");
        assert_eq!(springs, "???.###");
        assert_eq!(target, vec![1, 1, 3]);
        let count = get_solution_count(&springs, &target);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(&get_input());
        assert_eq!(answer, 21);
        let answer = solve_p2(&get_input());
        assert_eq!(answer, 525152);
    }

    #[test]
    fn test_solve() {
        let input = fs::read_to_string("input.txt").unwrap();
        let answer = solve_p1(&input);
        assert_eq!(answer, 7599);
        let answer = solve_p2(&input);
        assert_eq!(answer, 15454556629917);
    }

    #[test]
    fn test_unfold() {
        let input = "???.### 1,1,3";
        let parsed = parse_row(input);
        let unfolded = unfold(&parsed);
        assert_eq!(unfolded.0, "???.###????.###????.###????.###????.###");
        assert_eq!(
            unfolded.1,
            vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]
        );
    }
}
