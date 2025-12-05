use aoc2025::*;

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let (range_part, id_part) = input.split_once("\n\n").unwrap();
    let ranges = range_part
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();
    let ids = id_part.lines().map(|line| line.parse().unwrap()).collect();
    (ranges, ids)
}

fn merge_ranges(ranges: &mut [(usize, usize)]) -> Vec<(usize, usize)> {
    ranges.sort_by_key(|&(start, _)| start);
    let mut merged: Vec<(usize, usize)> = Vec::new();
    for &(start, end) in ranges.iter() {
        if let Some((_last_start, last_end)) = merged.last_mut() {
            if start <= *last_end + 1 {
                // The +1 above allows merging adjacent ranges
                *last_end = (*last_end).max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }
    merged
}

fn solve_p1(input: &str) -> usize {
    let (ranges, ids) = parse_input(input);
    ids.iter()
        .filter(|id| ranges.iter().any(|(start, end)| (start..=end).contains(id)))
        .count()
}

fn solve_p2(input: &str) -> usize {
    let (mut ranges, _ids) = parse_input(input);
    let merged = merge_ranges(&mut ranges);
    // Count the numbers covered by the merged ranges
    merged.iter().map(|(start, end)| end - start + 1).sum()
}

fn main() {
    let input = read_input(5);

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
        let input = read_test_input(5);
        let answer = solve_p1(&input);
        assert_eq!(answer, 3);
        let answer = solve_p2(&input);
        assert_eq!(answer, 14);
    }
}
