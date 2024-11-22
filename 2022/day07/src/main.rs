use std::collections::HashMap;

fn parse_input(input: &str) -> HashMap<String, usize> {
    let mut fs = HashMap::new();
    let mut dir_stack = vec!['/'.to_string()];
    let mut curr_path = "/".to_string();

    for line in input.lines() {
        match line {
            _ if line.starts_with("$ cd ..") => {
                dir_stack.pop();
                curr_path = dir_stack.last().unwrap().clone();
            }
            _ if line.starts_with("$ cd /") => {
                dir_stack = vec!['/'.to_string()];
                curr_path = "/".to_string();
                fs.insert(curr_path.clone(), 0);
            }
            _ if line.starts_with("$ cd ") => {
                let dir = line.split_whitespace().last().unwrap();
                curr_path = curr_path + dir + "/";
                fs.entry(curr_path.clone()).or_insert(0);
                dir_stack.push(curr_path.clone());
            }
            _ if line.starts_with("$ ls") => {}
            _ if line.starts_with("dir") => {}
            _ => {
                let mut parts = line.split_whitespace();
                let size: usize = parts.next().unwrap().parse().unwrap();
                dir_stack.iter_mut().for_each(|d| {
                    *fs.get_mut(d).unwrap() += size;
                });
            }
        }
    }

    fs
}

fn solve_p1(input: &str) -> usize {
    let fs = parse_input(input);
    fs.iter()
        .filter(|(_, &v)| v <= 100_000)
        .map(|(_, &v)| v)
        .sum()
}

fn solve_p2(input: &str) -> usize {
    let fs = parse_input(input);
    let target_free_space = 30_000_000;
    let curr_free_space = 70_000_000 - fs.values().max().unwrap();
    let min_size_to_delete = target_free_space - curr_free_space;

    fs.iter()
        .filter(|(_, &v)| v >= min_size_to_delete)
        .map(|(_, &v)| v)
        .min()
        .unwrap()
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

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 95437);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 24_933_642);
    }
}
