fn parse_input(input: &str) -> Vec<isize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn solve_p1(data: &[isize]) -> isize {
    solve(data, 1, 1)
}

fn solve_p2(data: &[isize]) -> isize {
    solve(data, 811589153, 10)
}

fn solve(data: &[isize], multiple: usize, iterations: usize) -> isize {
    let mut mixed = data
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, v)| (i, v * multiple as isize))
        .collect::<Vec<_>>();
    let orig = mixed.clone();
    let mut cyc = orig.iter().cycle();
    let zero_tuple = (data.iter().position(|x| x == &0).unwrap(), 0);
    let datalen = data.len();
    let wraplen = data.len() - 1;

    for _ in 0..iterations * datalen {
        let curr = cyc.next().unwrap();
        let old_idx = mixed.iter().position(|x| x == curr).unwrap();
        mixed.remove(old_idx);
        let mut new_idx = (old_idx as isize + curr.1) % wraplen as isize;
        if new_idx < 0 {
            new_idx += wraplen as isize;
        }
        mixed.insert(new_idx as usize, *curr);
    }

    let zero_idx_tuple = mixed.iter().position(|x| x == &zero_tuple).unwrap();
    let mut sum = 0;
    for i in [1000, 2000, 3000] {
        let idx = (zero_idx_tuple + i) % datalen;
        sum += mixed[idx].1;
    }
    sum
}

fn main() {
    let input = include_str!("../input.txt");
    let data = parse_input(input);

    let start = std::time::Instant::now();
    let answer = solve_p1(&data);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:.1?}");

    let start = std::time::Instant::now();
    let answer = solve_p2(&data);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, elapsed: {elapsed:.1?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
2
-3
3
-2
0
4
";

    #[test]
    fn test_solve_with_test_input() {
        let data = parse_input(INPUT);
        let answer = solve_p1(&data);
        assert_eq!(answer, 3);
        let answer = solve_p2(&data);
        assert_eq!(answer, 1623178306);
    }
}
