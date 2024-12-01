use regex::Regex;
use std::{
    fmt::{self, Display, Formatter},
    ops::{Add, Mul},
};

#[derive(Debug, Default, Clone, Copy)]
struct Resources {
    ore: u8,
    clay: u8,
    obsidian: u8,
}

impl Resources {
    fn new(ore: u8, clay: u8, obsidian: u8) -> Self {
        Resources {
            ore,
            clay,
            obsidian,
        }
    }

    fn checked_sub(self, other: Self) -> Option<Self> {
        Some(Self {
            ore: self.ore.checked_sub(other.ore)?,
            clay: self.clay.checked_sub(other.clay)?,
            obsidian: self.obsidian.checked_sub(other.obsidian)?,
        })
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
        }
    }
}

impl Mul<u8> for Resources {
    type Output = Self;

    fn mul(self, other: u8) -> Self {
        Self {
            ore: self.ore * other,
            clay: self.clay * other,
            obsidian: self.obsidian,
        }
    }
}

#[derive(Debug, Default)]
struct Blueprint {
    id: usize,
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
}

impl Blueprint {
    fn new(id: usize) -> Self {
        Blueprint {
            id,
            ..Default::default()
        }
    }
}

impl Display for Blueprint {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Blueprint {}:\n  Each ore robot costs {} ore.\n  Each clay robot costs {} ore.\n  Each obsidian robot costs {} ore and {} clay.\n  Each geode robot costs {} ore and {} obsidian.",
            self.id,
            self.ore_robot_cost.ore,
            self.clay_robot_cost.ore,
            self.obsidian_robot_cost.ore,
            self.obsidian_robot_cost.clay,
            self.geode_robot_cost.ore,
            self.geode_robot_cost.obsidian
        )
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
        State {
            minutes_remaining,
            geodes_secured: 0,
            resources: Default::default(),
            resources_rate: Resources::new(1, 0, 0),
        }
    }

    fn select_robot(self, cost: Resources, robot: Resources) -> Option<Self> {
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
            ore_robot_viable
                .then(|| self.select_robot(blueprint.ore_robot_cost, Resources::new(1, 0, 0))),
            clay_robot_viable
                .then(|| self.select_robot(blueprint.clay_robot_cost, Resources::new(0, 1, 0))),
            obsidian_robot_viable
                .then(|| self.select_robot(blueprint.obsidian_robot_cost, Resources::new(0, 0, 1))),
            geode_robot_viable.then(|| {
                self.select_robot(blueprint.geode_robot_cost, Resources::new(0, 0, 0))
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

    // As we have unlimited ore and clay, we prefer building geode robots when possible
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

fn parse_input(input: &str) -> Vec<Blueprint> {
    let mut blueprints = Vec::new();
    for bp_text in input.split("\n\n") {
        // Handle differences in format between test input and real input.
        let bp = bp_text
            .split("\n")
            .collect::<Vec<&str>>()
            .join(" ")
            .to_string();

        let bp_id = Regex::new(r"Blueprint (\d+):")
            .unwrap()
            .captures(&bp)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let mut blueprint = Blueprint::new(bp_id);

        let ore = Regex::new(r"Each ore robot costs (\d+) ore.")
            .unwrap()
            .captures(&bp)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u8>()
            .unwrap();
        blueprint.ore_robot_cost.ore = ore;

        let ore = Regex::new(r"Each clay robot costs (\d+) ore.")
            .unwrap()
            .captures(&bp)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u8>()
            .unwrap();
        blueprint.clay_robot_cost.ore = ore;

        let captures = Regex::new(r"Each obsidian robot costs (\d+) ore and (\d+) clay.")
            .unwrap()
            .captures(&bp)
            .unwrap();

        blueprint.obsidian_robot_cost.ore =
            captures.get(1).unwrap().as_str().parse::<u8>().unwrap();
        blueprint.obsidian_robot_cost.clay =
            captures.get(2).unwrap().as_str().parse::<u8>().unwrap();

        let captures = Regex::new(r"Each geode robot costs (\d+) ore and (\d+) obsidian.")
            .unwrap()
            .captures(&bp)
            .unwrap();

        blueprint.geode_robot_cost.ore = captures.get(1).unwrap().as_str().parse::<u8>().unwrap();
        blueprint.geode_robot_cost.obsidian =
            captures.get(2).unwrap().as_str().parse::<u8>().unwrap();
        println!("{:#}", blueprint);
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

fn solve_p1(input: &str) -> u32 {
    let blueprints = parse_input(input);

    blueprints
        .iter()
        .map(|blueprint| {
            let mut best = 0;
            branch_and_bound(blueprint, State::new(24), &mut best);
            println!("Best: {}", best);
            blueprint.id as u32 * best as u32
        })
        .sum()
}

fn solve_p2(_input: &str) -> i32 {
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

    const INPUT: &str = "Blueprint 1:
  Each ore robot costs 4 ore.
  Each clay robot costs 2 ore.
  Each obsidian robot costs 3 ore and 14 clay.
  Each geode robot costs 2 ore and 7 obsidian.

Blueprint 2:
  Each ore robot costs 2 ore.
  Each clay robot costs 3 ore.
  Each obsidian robot costs 3 ore and 8 clay.
  Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 33);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 0);
    }
}
