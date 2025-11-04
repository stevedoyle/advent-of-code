use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split('-');
        let from = parts.next().unwrap().to_string();
        let to = parts.next().unwrap().to_string();

        // Add edge in both directions
        graph
            .entry(from.clone())
            .or_insert_with(Vec::new)
            .push(to.clone());
        graph.entry(to).or_insert_with(Vec::new).push(from);
    }

    graph
}

fn dfs_with_path(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    end: &str,
    visited: &mut HashSet<String>,
    path: &mut Vec<String>,
    all_paths: &mut Vec<Vec<String>>,
) -> i32 {
    path.push(current.to_string());

    if current == end {
        // Record the complete path from start to end
        all_paths.push(path.clone());
        path.pop();
        return 1;
    }

    let mut count = 0;

    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            let is_small_cave = neighbor.chars().all(char::is_lowercase);

            // Skip if it's a small cave we've already visited
            if is_small_cave && visited.contains(neighbor) {
                continue;
            }

            // Don't go back to start
            if neighbor == "start" {
                continue;
            }

            if is_small_cave {
                visited.insert(neighbor.clone());
            }

            count += dfs_with_path(graph, neighbor, end, visited, path, all_paths);

            if is_small_cave {
                visited.remove(neighbor);
            }
        }
    }

    path.pop();
    count
}

fn solve_p1(input: &str) -> i32 {
    let graph = parse_input(input);
    let mut visited = HashSet::new();
    let mut path = Vec::new();
    let mut all_paths = Vec::new();

    visited.insert("start".to_string());
    dfs_with_path(
        &graph,
        "start",
        "end",
        &mut visited,
        &mut path,
        &mut all_paths,
    )
}

fn dfs_with_path_p2(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    end: &str,
    visited: &mut HashMap<String, i32>,
    has_visited_small_twice: bool,
    path: &mut Vec<String>,
) -> i32 {
    path.push(current.to_string());

    if current == end {
        path.pop();
        return 1;
    }

    let mut count = 0;

    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            // Don't go back to start
            if neighbor == "start" {
                continue;
            }

            let is_small_cave = neighbor.chars().all(char::is_lowercase);
            let visit_count = visited.get(neighbor).unwrap_or(&0);

            let can_visit = if is_small_cave {
                *visit_count == 0 || (*visit_count == 1 && !has_visited_small_twice)
            } else {
                true // Large caves can always be visited
            };

            if can_visit {
                let new_has_visited_twice = if is_small_cave && *visit_count == 1 {
                    true
                } else {
                    has_visited_small_twice
                };

                if is_small_cave {
                    *visited.entry(neighbor.clone()).or_insert(0) += 1;
                }

                count +=
                    dfs_with_path_p2(graph, neighbor, end, visited, new_has_visited_twice, path);

                if is_small_cave {
                    let entry = visited.get_mut(neighbor).unwrap();
                    *entry -= 1;
                    if *entry == 0 {
                        visited.remove(neighbor);
                    }
                }
            }
        }
    }

    path.pop();
    count
}

fn solve_p2(input: &str) -> i32 {
    let graph = parse_input(input);
    let mut visited = HashMap::new();
    let mut path = Vec::new();

    dfs_with_path_p2(&graph, "start", "end", &mut visited, false, &mut path)
}

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let input_path = std::path::Path::new(manifest_dir).join("input.txt");
    let input = std::fs::read_to_string(input_path).unwrap();

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
        let manifest_dir = env!("CARGO_MANIFEST_DIR");

        let test_input_path = std::path::Path::new(manifest_dir).join("test_input.txt");
        let input = std::fs::read_to_string(test_input_path).unwrap();
        let answer = solve_p1(&input);
        assert_eq!(answer, 10);

        let answer = solve_p2(&input);
        assert_eq!(answer, 36);

        let test_input1_path = std::path::Path::new(manifest_dir).join("test_input1.txt");
        let input = std::fs::read_to_string(test_input1_path).unwrap();
        let answer = solve_p1(&input);
        assert_eq!(answer, 19);

        let answer = solve_p2(&input);
        assert_eq!(answer, 103);

        let test_input2_path = std::path::Path::new(manifest_dir).join("test_input2.txt");
        let input = std::fs::read_to_string(test_input2_path).unwrap();
        let answer = solve_p1(&input);
        assert_eq!(answer, 226);

        let answer = solve_p2(&input);
        assert_eq!(answer, 3509);
    }
}
