use aoc2015::*;
use itertools::Itertools;

const DAY: u8 = 9;

/// A distance between two locations, e.g. `London to Dublin = 464`.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Edge<'a> {
    from: &'a str,
    to: &'a str,
    distance: usize,
}

impl<'a> Edge<'a> {
    fn parse(line: &'a str) -> Self {
        match line.split_whitespace().collect::<Vec<_>>().as_slice() {
            [from, "to", to, "=", distance] => Edge {
                from,
                to,
                distance: distance
                    .parse()
                    .unwrap_or_else(|_| panic!("invalid distance in: {line}")),
            },
            _ => panic!("invalid route: {line}"),
        }
    }
}

fn parse_input(input: &str) -> Vec<Edge<'_>> {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(Edge::parse)
        .collect()
}

/// Total distance of every route that visits each location exactly once.
///
/// Routes are open paths that may start anywhere, and every pair of locations
/// is assumed to be connected. Brute force: O(n!), fine for the ~8 locations
/// in the puzzle.
fn route_lengths(edges: &[Edge]) -> Vec<usize> {
    let mut names: Vec<&str> = edges.iter().flat_map(|e| [e.from, e.to]).collect();
    names.sort_unstable();
    names.dedup();
    let index = |name: &str| names.binary_search(&name).expect("known location");

    let n = names.len();
    let mut dist = vec![vec![None; n]; n];
    for e in edges {
        let (a, b) = (index(e.from), index(e.to));
        dist[a][b] = Some(e.distance);
        dist[b][a] = Some(e.distance);
    }

    (0..n)
        .permutations(n)
        .map(|route| {
            route
                .windows(2)
                .map(|w| {
                    dist[w[0]][w[1]].unwrap_or_else(|| {
                        panic!("no route between {} and {}", names[w[0]], names[w[1]])
                    })
                })
                .sum::<usize>()
        })
        .collect()
}

fn solve_p1(input: &str) -> usize {
    let edges = parse_input(input);
    route_lengths(&edges).into_iter().min().expect("no routes")
}

fn solve_p2(input: &str) -> usize {
    let edges = parse_input(input);
    route_lengths(&edges).into_iter().max().expect("no routes")
}

fn main() {
    run(DAY, solve_p1, solve_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_edge() {
        assert_eq!(
            Edge::parse("London to Dublin = 464"),
            Edge {
                from: "London",
                to: "Dublin",
                distance: 464
            }
        );
    }

    #[test]
    fn parse_test_input() {
        let input = read_test_input(DAY, 1);
        let edges = parse_input(&input);
        assert_eq!(edges.len(), 3);
        assert_eq!(edges[2].from, "Dublin");
        assert_eq!(edges[2].to, "Belfast");
        assert_eq!(edges[2].distance, 141);
    }

    #[test]
    fn parse_real_input() {
        // Every route line in the puzzle input must parse.
        let input = read_input(DAY);
        assert_eq!(parse_input(&input).len(), input.lines().count());
    }

    #[test]
    fn part1() {
        assert_eq!(solve_p1(&read_test_input(DAY, 1)), 605);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_p2(&read_test_input(DAY, 1)), 982);
    }
}
