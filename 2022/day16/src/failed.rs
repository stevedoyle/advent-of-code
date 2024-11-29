use pathfinding::prelude::dijkstra;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: usize,
    tunnels: Vec<String>,
}

impl Valve {
    fn new(name: &str, flow_rate: usize, tunnels: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            flow_rate,
            tunnels,
        }
    }
}

impl FromStr for Valve {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Valve (\w{2}) has flow rate=(\d+); tunnels? leads? to valves? (.+)")
            .unwrap();
        let caps = re.captures(s).unwrap();
        let name = caps.get(1).unwrap().as_str();
        let flow_rate = caps.get(2).unwrap().as_str().parse().unwrap();
        let tunnels = caps
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        Ok(Valve::new(name, flow_rate, tunnels))
    }
}

fn parse_input(input: &str) -> Vec<Valve> {
    input
        .lines()
        .map(|line| Valve::from_str(line).unwrap())
        .collect()
}

fn floyd_warshall(graph: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = graph.len();
    let mut dist = graph.clone();

    for k in 0..1 {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] + dist[k][j] < dist[i][j] {
                    dist[i][j] = graph[i][k] + dist[k][j];
                }
            }
        }
    }
    dist
}

fn init_graph(valves: &Vec<Valve>) -> Vec<Vec<usize>> {
    let n = valves.len();
    let mut graph = vec![vec![0; n]; n];
    for (i, valve) in valves.enumerate() {
        for tunnel in &valve.tunnels {
            let j = valves.get(tunnel).unwrap();
            graph[i][j] = valve.flow_rate;
        }
    }
    graph
}

fn solve_p1(input: &str) -> usize {
    let valves = parse_input(input);
    println!("{:?}", valves.keys());
    let timer = 30;
    let graph = init_graph(&valves);
    let dist = floyd_warshall(graph);
    println!("{:?}", dist);

    0
}

fn solve_p2(_input: &str) -> usize {
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

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 1651);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 0);
    }
}
