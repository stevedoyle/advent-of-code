use std::{collections::HashMap, str::FromStr};

use regex::Regex;

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default_action: String,
}

impl FromStr for Workflow {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let re = Regex::new(r"(\w+)\{(([\w\d<>:]+),)*?(\w+)\}").map_err(|_| ParseError)?;
        let re = Regex::new(r"(\w+)\{(.+)\}").map_err(|_| ParseError)?;
        let captures = re.captures(s).unwrap();
        let name = captures.get(1).unwrap().as_str();
        let rule_str = captures.get(2).unwrap().as_str();
        let parts: Vec<&str> = rule_str.split(',').collect();

        let wf = Workflow {
            name: name.to_owned(),
            // rules: Vec::with_capacity(10),
            rules: parts[0..parts.len() - 1]
                .iter()
                .map(|&r| Rule::from_str(r).unwrap())
                .collect(),
            default_action: parts.last().unwrap().to_string(),
        };
        Ok(wf)
    }
}

#[derive(Debug, Clone)]
struct Rule {
    category: Category,
    op: String,
    threshold: usize,
    action: String,
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // a<2006:qkq,m>2090:A
        let re = Regex::new(r"([xmas])([<>])(\d+):(\w+)").map_err(|_| ParseError)?;
        let captures = re.captures(s).unwrap();
        let rule = Rule {
            category: Category::from(captures.get(1).unwrap().as_str()),
            op: captures.get(2).unwrap().as_str().to_owned(),
            threshold: captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            action: captures.get(4).unwrap().as_str().to_owned(),
        };
        Ok(rule)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

impl From<&str> for Category {
    fn from(s: &str) -> Self {
        match s {
            "x" => Category::X,
            "m" => Category::M,
            "a" => Category::A,
            "s" => Category::S,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl FromStr for Part {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"x=(\d+),m=(\d+),a=(\d+),s=(\d+)").map_err(|_| ParseError)?;
        let captures = re.captures(s).unwrap();
        let plan = Part {
            x: captures.get(1).unwrap().as_str().parse().unwrap(),
            m: captures.get(2).unwrap().as_str().parse().unwrap(),
            a: captures.get(3).unwrap().as_str().parse().unwrap(),
            s: captures.get(4).unwrap().as_str().parse().unwrap(),
        };
        Ok(plan)
    }
}

impl Part {
    fn rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    fn cat_val(&self, cat: Category) -> usize {
        match cat {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ParseError;

fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let (workflow_lines, part_lines) = input.split_once("\n\n").unwrap();

    let workflows: HashMap<String, Workflow> = workflow_lines
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Workflow::from_str(line.trim()).unwrap())
        .map(|w| (w.name.clone(), w))
        .collect();

    let parts = part_lines
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Part::from_str(line.trim()).unwrap())
        .collect();

    (workflows, parts)
}

/// Returns true if the part is accepted.
fn process(workflows: &HashMap<String, Workflow>, part: &Part) -> bool {
    let mut wf = workflows.get("in").unwrap();
    loop {
        let mut match_found = false;
        for rule in &wf.rules {
            let value = part.cat_val(rule.category);
            match_found = match rule.op.as_str() {
                "<" => value < rule.threshold,
                ">" => value > rule.threshold,
                _ => unreachable!(),
            };
            if match_found {
                match rule.action.as_str() {
                    "A" => return true,
                    "R" => return false,
                    _ => {
                        wf = workflows.get(&rule.action).unwrap();
                        break;
                    }
                }
            }
        }
        // All rules processed without a match. Use the default action.
        if !match_found {
            match wf.default_action.as_str() {
                "A" => return true,
                "R" => return false,
                _ => {
                    wf = workflows.get(&wf.default_action).unwrap();
                }
            }
        }
    }
}

fn solve_p1(input: &str) -> usize {
    let (workflows, parts) = parse_input(input);

    let accepted: Vec<Part> = parts
        .iter()
        .filter(|p| process(&workflows, p))
        .cloned()
        .collect();
    accepted.iter().map(|p| p.rating()).sum()
}

fn solve_p2(input: &str) -> usize {
    let (workflows, _) = parse_input(input);

    0
}

fn main() {
    let input = include_str!("../input.txt");
    let answer = solve_p1(input);
    println!("Part 1: {answer}");
    // let answer = solve_p2(input);
    // println!("Part 2: {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_parse_part() {
        let s = "{x=787,m=2655,a=1222,s=2876}";
        let part = Part::from_str(s).unwrap();
        assert_eq!(part.x, 787);
        assert_eq!(part.m, 2655);
        assert_eq!(part.a, 1222);
        assert_eq!(part.s, 2876);
    }

    #[test]
    fn test_parse_workflow() {
        let s = "px{a<2006:qkq,m>2090:A,rfg}";
        let wf = Workflow::from_str(s).unwrap();
        assert_eq!(wf.name, "px");
        assert_eq!(wf.default_action, "rfg");
        assert_eq!(wf.rules.len(), 2);
        assert_eq!(wf.rules[1].category, Category::M);
        assert_eq!(wf.rules[1].op, ">");
        assert_eq!(wf.rules[1].threshold, 2090);
        assert_eq!(wf.rules[1].action, "A");
    }

    #[test]
    fn test_parse_rule() {
        let s = "a<2006:qkq";
        let rule = Rule::from_str(s).unwrap();
        assert_eq!(rule.category, Category::A);
        assert_eq!(rule.op, "<");
        assert_eq!(rule.threshold, 2006);
        assert_eq!(rule.action, "qkq");
    }

    #[test]
    fn test_parse_input() {
        let (workflows, parts) = parse_input(INPUT);
        assert_eq!(workflows.len(), 11);
        assert_eq!(parts.len(), 5);
    }

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 19114);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 167409079868000);
    }

    #[test]
    fn test_solve() {
        let input = include_str!("../input.txt");
        let answer = solve_p1(input);
        assert_eq!(answer, 402185);
    }
}
