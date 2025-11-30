use aoc2020::*;

#[derive(Debug)]
struct BagRule {
    outer: String,
    inners: Vec<(u32, String)>,
}

fn parse_input(input: &str) -> Vec<BagRule> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" bags contain ").collect();
            let outer = parts[0].to_string();
            let inners = if parts[1] == "no other bags." {
                vec![]
            } else {
                parts[1]
                    .trim_end_matches('.')
                    .split(", ")
                    .map(|s| {
                        let mut iter = s.split_whitespace();
                        let count = iter.next().unwrap().parse::<u32>().unwrap();
                        let color = iter
                            .take_while(|&w| w != "bag" && w != "bags")
                            .collect::<Vec<&str>>()
                            .join(" ");
                        (count, color)
                    })
                    .collect()
            };
            BagRule { outer, inners }
        })
        .collect()
}

fn solve_p1(input: &str) -> usize {
    let bag_rules = parse_input(input);
    let my_bag = "shiny gold";

    let mut can_contain_my_bag = std::collections::HashSet::new();
    let mut changed = true;
    while changed {
        changed = false;
        for rule in &bag_rules {
            if rule
                .inners
                .iter()
                .any(|(_, color)| can_contain_my_bag.contains(color) || color == my_bag)
                && can_contain_my_bag.insert(rule.outer.clone())
            {
                changed = true;
            }
        }
    }
    can_contain_my_bag.len()
}

fn count_inner_bags(
    bag_rules: &Vec<BagRule>,
    bag_color: &str,
    memo: &mut std::collections::HashMap<String, usize>,
) -> usize {
    if let Some(&count) = memo.get(bag_color) {
        return count;
    }
    let rule = bag_rules.iter().find(|r| r.outer == bag_color).unwrap();
    let mut total = 0;
    for (count, inner_color) in &rule.inners {
        total +=
            *count as usize + (*count as usize) * count_inner_bags(bag_rules, inner_color, memo);
    }
    memo.insert(bag_color.to_string(), total);
    total
}

fn solve_p2(input: &str) -> usize {
    let bag_rules = parse_input(input);
    let my_bag = "shiny gold";

    let mut memo = std::collections::HashMap::new();
    count_inner_bags(&bag_rules, my_bag, &mut memo)
}

fn main() {
    let input = read_input(7);

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
        let input = read_test_input(7);
        let answer = solve_p1(&input);
        assert_eq!(answer, 4);
        let answer = solve_p2(&input);
        assert_eq!(answer, 32);
    }
}
