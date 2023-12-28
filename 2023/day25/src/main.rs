use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graphmap::UnGraphMap;
use std::collections::HashSet;
use std::time::Instant;

fn solve_p1(input: &str) -> usize {
    let mut edges = HashSet::<(&str, &str)>::new();

    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|l| l.trim().split_once(':'))
        .for_each(|(node, s)| {
            s.split_whitespace().for_each(|neighbor| {
                edges.insert((node, neighbor));
            })
        });

    let graph = UnGraphMap::<&str, ()>::from_edges(edges);
    let Ok(Some((_, group))) = stoer_wagner_min_cut(&graph, |_| Ok::<_, ()>(1)) else {
        panic!()
    };
    let g1 = group.len();
    let g2 = graph.node_count() - g1;

    g1 * g2
}

fn main() {
    let input = include_str!("../input.txt");
    let start = Instant::now();
    let answer = solve_p1(input);
    let elapsed = Instant::now() - start;
    println!("Part 1: {answer}");
    println!("{elapsed:.2?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    jqt: rhn xhk nvd
    rsh: frs pzl lsr
    xhk: hfx
    cmg: qnr nvd lhk bvb
    rhn: xhk bvb hfx
    bvb: xhk hfx
    pzl: lsr hfx nvd
    qnr: nvd
    ntq: jqt hfx bvb xhk
    nvd: lhk
    lsr: lhk
    rzs: qnr cmg lsr rsh
    frs: qnr lhk lsr";

    #[test]
    fn test_solve_p1_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 54);
    }

    #[test]
    fn test_solve() {
        let input = include_str!("../input.txt");
        let answer = solve_p1(input);
        assert_eq!(answer, 551196);
    }
}
