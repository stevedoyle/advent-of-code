use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::once,
};

fn parse_input(input: &str) -> (HashMap<&str, HashSet<&str>>, HashSet<&str>) {
    let mut data: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut tnames = HashSet::new();

    input.lines().for_each(|line| {
        if line.trim().is_empty() {
            return;
        }
        let (lhs, rhs) = line.split_once('-').unwrap();
        data.entry(lhs).or_default().insert(rhs);
        data.entry(rhs).or_default().insert(lhs);
        if lhs.starts_with('t') {
            tnames.insert(lhs);
        }
        if rhs.starts_with('t') {
            tnames.insert(rhs);
        }
    });
    (data, tnames)
}

fn dfs<'a>(
    current: HashSet<&'a str>,
    mut potential: HashSet<&'a str>,
    mut processed: HashSet<&'a str>,
    connections: &HashMap<&str, HashSet<&'a str>>,
    result: &mut HashSet<&'a str>,
) {
    if potential.is_empty() && processed.is_empty() {
        if current.len() > result.len() {
            *result = current;
        }
        return;
    }
    for node in potential.clone() {
        let neighbors = &connections[node];
        dfs(
            current.iter().copied().chain(once(node)).collect(),
            potential.intersection(neighbors).copied().collect(),
            processed.intersection(neighbors).copied().collect(),
            connections,
            result,
        );
        potential.remove(node);
        processed.insert(node);
    }
}

fn solve_p1(input: &str) -> usize {
    let (connections, tnames) = parse_input(input);
    let mut total = HashSet::new();
    let mut queue: VecDeque<_> = tnames.into_iter().map(|n| (n, vec![n])).collect();
    while let Some((name, mut path)) = queue.pop_front() {
        if path.len() == 3 {
            path.sort_unstable();
            total.insert(path);
            continue;
        }
        for &next in connections[name].iter() {
            let mut temp = path.clone();
            if !temp.contains(&next) && connections[next].contains(&path[0]) {
                temp.push(next);
                queue.push_back((next, temp));
            }
        }
    }
    total.len()
}

fn solve_p2(input: &str) -> String {
    let (connections, _) = parse_input(input);
    let mut res = HashSet::new();
    dfs(
        Default::default(),
        connections.keys().copied().collect(),
        Default::default(),
        &connections,
        &mut res,
    );
    let mut res_vec: Vec<_> = res.into_iter().collect();
    res_vec.sort_unstable();
    res_vec.join(",")
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let start = std::time::Instant::now();
    let answer = solve_p1(&input);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:.1?}");

    let start = std::time::Instant::now();
    let answer = solve_p2(&input);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, elapsed: {elapsed:.1?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_with_test_input() {
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let answer = solve_p1(&input);
        assert_eq!(answer, 7);
        let answer = solve_p2(&input);
        assert_eq!(answer, "co,de,ka,ta");
    }
}
