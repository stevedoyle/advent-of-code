use std::ops::{Add, Mul};

use regex::Regex;

fn parse_input(input: &str) -> Vec<Blueprint> {
    let mut blueprints = Vec::new();
    let bp_id_re = Regex::new(r"Blueprint (\d+):").unwrap();
    let ore_robot_re = Regex::new(r"Each ore robot costs (\d+) ore.").unwrap();
    let clay_robot_re = Regex::new(r"Each clay robot costs (\d+) ore.").unwrap();
    let obsidian_robot_re = Regex::new(r"Each obsidian robot costs (\d+) ore and (\d+) clay.").unwrap();
    let geode_robot_re = Regex::new(r"Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();

    for bp_text in input.lines() {
        // Handle differences in format between test input and real input.
        let bp = bp_text
            .split("\n")
            .collect::<Vec<&str>>()
            .join(" ")
            .to_string();

        let bp_id = bp_id_re
            .captures(&bp)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u8>()
            .unwrap();
        let mut blueprint = Blueprint::new(bp_id);

        let ore = ore_robot_re
            .captures(&bp)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u8>()
            .unwrap();
        blueprint.ore_robot_cost.ore = ore;

        let clay = clay_robot_re
            .captures(&bp)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u8>()
            .unwrap();
        blueprint.clay_robot_cost.ore = clay;

        let captures = obsidian_robot_re
            .captures(&bp)
            .unwrap();

        blueprint.obsidian_robot_cost.ore =
            captures.get(1).unwrap().as_str().parse::<u8>().unwrap();
        blueprint.obsidian_robot_cost.clay =
            captures.get(2).unwrap().as_str().parse::<u8>().unwrap();

        let captures = geode_robot_re
            .captures(&bp)
            .unwrap();

        blueprint.geode_robot_cost.ore = captures.get(1).unwrap().as_str().parse::<u8>().unwrap();
        blueprint.geode_robot_cost.obsidian =
            captures.get(2).unwrap().as_str().parse::<u8>().unwrap();
        blueprints.push(blueprint);
    }
    blueprints
}

fn branch_and_bound(blueprint: &Blueprint, state: State, best: &mut u8) {
    *best = state.geodes_secured.max(*best);
    for state in state.branch(blueprint) {
        if state.bound(blueprint) > *best {
            branch_and_bound(blueprint, state, best);
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    id: u8,
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
}

impl Blueprint {
    fn new(id: u8) -> Self {
        Self {
            id,
            ore_robot_cost: Default::default(),
            clay_robot_cost: Default::default(),
            obsidian_robot_cost: Default::default(),
            geode_robot_cost: Default::default(),
        }
    }
}

// fn blueprint(input: &str) -> IResult<Blueprint> {
//     let (input, id) = delimited(tag("Blueprint "), u8, tag(": "))(input)?;
//     let (input, ore_robot_cost) = delimited(tag("Each ore robot costs "), u8, tag(" ore. "))
//         .map(|ore| ONE_ORE * ore)
//         .parse(input)?;
//     let (input, clay_robot_cost) = delimited(tag("Each clay robot costs "), u8, tag(" ore. "))
//         .map(|ore| ONE_ORE * ore)
//         .parse(input)?;
//     let (input, obsidian_robot_cost) = delimited(
//         tag("Each obsidian robot costs "),
//         separated_pair(u8, tag(" ore and "), u8),
//         tag(" clay. "),
//     )
//     .map(|(ore, clay)| ONE_ORE * ore + ONE_CLAY * clay)
//     .parse(input)?;
//     let (input, geode_robot_cost) = delimited(
//         tag("Each geode robot costs "),
//         separated_pair(u8, tag(" ore and "), u8),
//         tag(" obsidian."),
//     )
//     .map(|(ore, obsidian)| ONE_ORE * ore + ONE_OBSIDIAN * obsidian)
//     .parse(input)?;
//     Ok((
//         input,
//         Blueprint {
//             id,
//             ore_robot_cost,
//             clay_robot_cost,
//             obsidian_robot_cost,
//             geode_robot_cost,
//         },
//     ))
// }

#[derive(Debug, Clone, Copy, Default)]
struct Resources {
    ore: u8,
    clay: u8,
    obsidian: u8,
}

const ONE_ORE: Resources = Resources {
    ore: 1,
    clay: 0,
    obsidian: 0,
};
const ONE_CLAY: Resources = Resources {
    ore: 0,
    clay: 1,
    obsidian: 0,
};
const ONE_OBSIDIAN: Resources = Resources {
    ore: 0,
    clay: 0,
    obsidian: 1,
};

impl Resources {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        Some(Self {
            ore: self.ore.checked_sub(rhs.ore)?,
            clay: self.clay.checked_sub(rhs.clay)?,
            obsidian: self.obsidian.checked_sub(rhs.obsidian)?,
        })
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
        }
    }
}

impl Mul<u8> for Resources {
    type Output = Self;

    fn mul(self, other: u8) -> Self {
        Self {
            ore: self.ore * other,
            clay: self.clay * other,
            obsidian: self.obsidian * other,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    minutes_remaining: u8,
    geodes_secured: u8,
    resources: Resources,
    resources_rate: Resources,
}

impl State {
    fn new(minutes_remaining: u8) -> Self {
        Self {
            minutes_remaining,
            geodes_secured: 0,
            resources: Default::default(),
            resources_rate: ONE_ORE,
        }
    }

    fn chose_robot(self, cost: Resources, robot: Resources) -> Option<Self> {
        (1..self.minutes_remaining).rev().zip(0..).find_map(
            |(minutes_remaining, minutes_passed)| {
                let resources = self.resources + self.resources_rate * minutes_passed;
                resources.checked_sub(cost).map(|resources| Self {
                    minutes_remaining,
                    resources: resources + self.resources_rate,
                    resources_rate: self.resources_rate + robot,
                    ..self
                })
            },
        )
    }

    fn branch(self, blueprint: &Blueprint) -> impl Iterator<Item = Self> + '_ {
        let max_ore_cost = blueprint
            .clay_robot_cost
            .ore
            .max(blueprint.obsidian_robot_cost.ore)
            .max(blueprint.geode_robot_cost.ore);
        let ore_robot_viable = self.resources_rate.ore < max_ore_cost;
        let clay_robot_viable = self.resources_rate.clay < blueprint.obsidian_robot_cost.clay;
        let obsidian_robot_viable = self.resources_rate.obsidian
            < blueprint.geode_robot_cost.obsidian
            && self.resources_rate.clay > 0;
        let geode_robot_viable = self.resources_rate.obsidian > 0;
        [
            ore_robot_viable.then(|| self.chose_robot(blueprint.ore_robot_cost, ONE_ORE)),
            clay_robot_viable.then(|| self.chose_robot(blueprint.clay_robot_cost, ONE_CLAY)),
            obsidian_robot_viable
                .then(|| self.chose_robot(blueprint.obsidian_robot_cost, ONE_OBSIDIAN)),
            geode_robot_viable.then(|| {
                self.chose_robot(blueprint.geode_robot_cost, Default::default())
                    .map(|state| Self {
                        geodes_secured: state.geodes_secured + state.minutes_remaining,
                        ..state
                    })
            }),
        ]
        .into_iter()
        .flatten()
        .flatten()
    }

    // we have unlimited ore and clay, and prefer building geode robots when possible
    fn bound(self, blueprint: &Blueprint) -> u8 {
        let geode_cost = blueprint.geode_robot_cost.obsidian;
        let (_, _, geodes) = (0..self.minutes_remaining).rev().fold(
            (
                self.resources.obsidian,
                self.resources_rate.obsidian,
                self.geodes_secured,
            ),
            |(obsidian, rate, geodes), minutes_remaining| {
                if obsidian >= geode_cost {
                    (
                        obsidian + rate - geode_cost,
                        rate,
                        geodes.saturating_add(minutes_remaining),
                    )
                } else {
                    (obsidian + rate, rate + 1, geodes)
                }
            },
        );
        geodes
    }
}

fn solve_p1(input: &str) -> u32 {
    let blueprints = parse_input(input);

    blueprints
        .iter()
        .map(|blueprint| {
            let mut best = 0;
            branch_and_bound(blueprint, State::new(24), &mut best);
            blueprint.id as u32 * best as u32
        })
        .sum()
}

fn solve_p2(input: &str) -> u32 {
    let blueprints = parse_input(input);
    blueprints.iter()
        .take(3)
        .map(|blueprint| {
            let mut best = 0;
            branch_and_bound(blueprint, State::new(32), &mut best);
            best as u32
        })
        .product()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let answer = solve_p1(&input);
    println!("Part 1: {answer}");
    let answer = solve_p2(&input);
    println!("Part 2: {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_with_test_input() {
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let answer = solve_p1(&input);
        assert_eq!(answer, 33);
        let answer = solve_p2(&input);
        assert_eq!(answer, 56 * 62);
    }
}
