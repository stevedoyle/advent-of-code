use std::{collections::HashMap, ops::Range, str::FromStr};

use regex::Regex;

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
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

        let mut wf = Workflow {
            name: name.to_owned(),
            // rules: Vec::with_capacity(10),
            rules: parts[0..parts.len() - 1]
                .iter()
                .map(|&r| Rule::from_str(r).unwrap())
                .collect(),
        };
        wf.rules.push(Rule {
            category: Category::None,
            op: "".to_string(),
            threshold: 0,
            action: parts.last().unwrap().to_string(),
        });
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
    None,
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
            _ => 0,
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
        for rule in &wf.rules {
            let match_found = match rule.op.as_str() {
                "<" => part.cat_val(rule.category) < rule.threshold,
                ">" => part.cat_val(rule.category) > rule.threshold,
                "" => true,
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
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct PartRange {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

impl PartRange {
    fn get_cat(&self, cat: Category) -> Range<usize> {
        match cat {
            Category::X => self.x.clone(),
            Category::M => self.m.clone(),
            Category::A => self.a.clone(),
            Category::S => self.s.clone(),
            _ => 0..0,
        }
    }

    fn combinations(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

/// Returns true if the part is accepted.
fn get_accepted_ranges(workflows: &HashMap<String, Workflow>) -> Vec<PartRange> {
    let start = PartRange {
        x: 1..4001,
        m: 1..4001,
        a: 1..4001,
        s: 1..4001,
    };

    process_workflow(workflows, "in", &start)
}

fn process_workflow(
    workflows: &HashMap<String, Workflow>,
    name: &str,
    in_range: &PartRange,
) -> Vec<PartRange> {
    match name {
        "A" => {
            return vec![in_range.clone()];
        }
        "R" => {
            return vec![];
        }
        _ => (),
    }

    let mut accepted = Vec::with_capacity(100);
    let mut working_range = in_range.clone();
    let wf = workflows.get(name).unwrap();

    for rule in &wf.rules {
        match rule.op.as_str() {
            "" => {
                accepted.extend(process_workflow(workflows, &rule.action, &working_range));
                break;
            }
            _ => {
                let (a, b) = split_range(&working_range, rule);
                if let Some(r) = a {
                    accepted.extend(process_workflow(workflows, &rule.action, &r));
                }
                if let Some(r) = b {
                    working_range = r;
                }
            }
        };
    }
    accepted
}

// lhs: range that matches the rule condition
// rhs: range that doesn't match the rule condition
fn split_range(in_range: &PartRange, rule: &Rule) -> (Option<PartRange>, Option<PartRange>) {
    match rule.op.as_str() {
        "<" => split_range_lt(in_range, rule),
        ">" => split_range_gt(in_range, rule),
        _ => (None, None),
    }
}

fn split_range_lt(in_range: &PartRange, rule: &Rule) -> (Option<PartRange>, Option<PartRange>) {
    let target = in_range.get_cat(rule.category);
    if target.start > rule.threshold {
        // entire range is outside the threshold
        return (None, Some(in_range.clone()));
    } else if target.end <= rule.threshold {
        // entire range is within the threshold
        return (Some(in_range.clone()), None);
    }

    // threshold lies within the range. Split it.
    match rule.category {
        Category::X => {
            let lhs = PartRange {
                x: in_range.x.start..rule.threshold,
                ..in_range.clone()
            };
            let rhs = PartRange {
                x: rule.threshold..in_range.x.end,
                ..in_range.clone()
            };
            (Some(lhs), Some(rhs))
        }
        Category::M => {
            let lhs = PartRange {
                m: in_range.m.start..rule.threshold,
                ..in_range.clone()
            };
            let rhs = PartRange {
                m: rule.threshold..in_range.m.end,
                ..in_range.clone()
            };
            (Some(lhs), Some(rhs))
        }
        Category::A => {
            let lhs = PartRange {
                a: in_range.a.start..rule.threshold,
                ..in_range.clone()
            };
            let rhs = PartRange {
                a: rule.threshold..in_range.a.end,
                ..in_range.clone()
            };
            (Some(lhs), Some(rhs))
        }
        Category::S => {
            let lhs = PartRange {
                s: in_range.s.start..rule.threshold,
                ..in_range.clone()
            };
            let rhs = PartRange {
                s: rule.threshold..in_range.s.end,
                ..in_range.clone()
            };
            (Some(lhs), Some(rhs))
        }
        _ => (None, None),
    }
}

fn split_range_gt(in_range: &PartRange, rule: &Rule) -> (Option<PartRange>, Option<PartRange>) {
    let target = in_range.get_cat(rule.category);
    if target.start > rule.threshold {
        // entire range is inside the threshold
        return (Some(in_range.clone()), None);
    } else if target.end <= rule.threshold {
        // entire range is outside the threshold
        return (None, Some(in_range.clone()));
    }

    // threshold lies within the range. Split it.

    match rule.category {
        Category::X => {
            let rhs = PartRange {
                x: in_range.x.start..rule.threshold + 1,
                ..in_range.clone()
            };
            let lhs = PartRange {
                x: (rule.threshold + 1)..in_range.x.end,
                ..in_range.clone()
            };
            (Some(lhs), Some(rhs))
        }
        Category::M => {
            let rhs = PartRange {
                m: in_range.m.start..rule.threshold + 1,
                ..in_range.clone()
            };
            let lhs = PartRange {
                m: (rule.threshold + 1)..in_range.m.end,
                ..in_range.clone()
            };
            (Some(lhs), Some(rhs))
        }
        Category::A => {
            let rhs = PartRange {
                a: in_range.a.start..rule.threshold + 1,
                ..in_range.clone()
            };
            let lhs = PartRange {
                a: (rule.threshold + 1)..in_range.a.end,
                ..in_range.clone()
            };
            (Some(lhs), Some(rhs))
        }
        Category::S => {
            let rhs = PartRange {
                s: in_range.s.start..rule.threshold + 1,
                ..in_range.clone()
            };
            let lhs = PartRange {
                s: (rule.threshold + 1)..in_range.s.end,
                ..in_range.clone()
            };
            (Some(lhs), Some(rhs))
        }
        _ => (None, None),
    }
}

fn get_combinations(accepted: &[PartRange]) -> usize {
    accepted.iter().map(|r| r.combinations()).sum()
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
    let accepted = get_accepted_ranges(&workflows);
    get_combinations(&accepted)
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
    use crate::split_range;

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
    fn test_split_range() {
        let start = PartRange {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        };
        let r1 = Rule {
            category: Category::X,
            op: "<".to_string(),
            threshold: 1000,
            action: "".to_string(),
        };
        let (lhs, rhs) = split_range(&start, &r1);
        assert_eq!(
            lhs,
            Some(PartRange {
                x: 1..1000,
                ..start.clone()
            })
        );
        assert_eq!(
            rhs,
            Some(PartRange {
                x: 1000..4001,
                ..start.clone()
            })
        );

        let r1 = Rule {
            category: Category::M,
            op: ">".to_string(),
            threshold: 1000,
            action: "".to_string(),
        };
        let (lhs, rhs) = split_range(&start, &r1);
        assert_eq!(
            lhs,
            Some(PartRange {
                m: 1001..4001,
                ..start.clone()
            })
        );
        assert_eq!(
            rhs,
            Some(PartRange {
                m: 1..1001,
                ..start.clone()
            })
        );

        let r1 = Rule {
            category: Category::S,
            op: "<".to_string(),
            threshold: 1351,
            action: "".to_string(),
        };
        let r2 = Rule {
            category: Category::S,
            op: "<".to_string(),
            threshold: 537,
            action: "".to_string(),
        };
        let (lhs, rhs) = split_range(&start, &r1);
        assert_eq!(
            lhs,
            Some(PartRange {
                s: 1..1351,
                ..start.clone()
            })
        );
        assert_eq!(
            rhs,
            Some(PartRange {
                s: 1351..4001,
                ..start.clone()
            })
        );
        let (lhs, rhs) = split_range(&lhs.unwrap(), &r2);
        assert_eq!(
            lhs,
            Some(PartRange {
                s: 1..537,
                ..start.clone()
            })
        );
        assert_eq!(
            rhs,
            Some(PartRange {
                s: 537..1351,
                ..start.clone()
            })
        );
    }

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
        assert_eq!(wf.rules.len(), 3);
        assert_eq!(wf.rules[1].category, Category::M);
        assert_eq!(wf.rules[1].op, ">");
        assert_eq!(wf.rules[1].threshold, 2090);
        assert_eq!(wf.rules[1].action, "A");
        assert_eq!(wf.rules[2].op, "");
        assert_eq!(wf.rules[2].action, "rfg");
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
