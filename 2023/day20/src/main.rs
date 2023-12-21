use core::fmt;
use num::Integer;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum FlipFlopState {
    Off,
    On,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Signal {
    pulse: Pulse,
    src: String,
    dst: String,
}

impl ToString for Signal {
    fn to_string(&self) -> String {
        format!("{} -{:?}-> {}", self.src, self.pulse, self.dst)
    }
}

impl fmt::Debug for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -{:?}-> {}", self.src, self.pulse, self.dst)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum ModuleType {
    Broadcast,
    FlipFlop,
    Conjunction,
    Output,
}

#[derive(Debug, PartialEq, Eq)]
struct Module {
    kind: ModuleType,
    name: String,
    // id: ModuleId,
    outputs: Vec<String>,
    conj_state: HashMap<String, Pulse>,
    flip_flop_state: FlipFlopState,
    count: Vec<usize>,
}

impl FromStr for Module {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, outputs) = s.split_once(" -> ").unwrap();
        let name = name.trim();
        let connected_to: Vec<String> = outputs.split(", ").map(|s| s.to_string()).collect();

        let module = match name {
            "broadcaster" => Module {
                kind: ModuleType::Broadcast,
                name: "broadcaster".to_string(),
                outputs: connected_to,
                conj_state: HashMap::new(),
                flip_flop_state: FlipFlopState::Off,
                count: vec![0, 0],
            },

            _ => match &name[0..1] {
                "%" => Module {
                    kind: ModuleType::FlipFlop,
                    name: name[1..].to_string(),
                    outputs: connected_to,
                    conj_state: HashMap::new(),
                    flip_flop_state: FlipFlopState::Off,
                    count: vec![0, 0],
                },
                "&" => Module {
                    kind: ModuleType::Conjunction,
                    name: name[1..].to_string(),
                    outputs: connected_to,
                    conj_state: HashMap::new(),
                    flip_flop_state: FlipFlopState::Off,
                    count: vec![0, 0],
                },
                _ => unreachable!(),
            },
        };
        Ok(module)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ParseError;

fn parse_input(input: &str) -> HashMap<String, Module> {
    let mut modules: HashMap<String, Module> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Module::from_str(line).unwrap())
        .map(|module| (module.name.clone(), module))
        .collect();
    let output_modules: Vec<String> = modules
        .iter()
        .map(|(_, v)| {
            v.outputs
                .iter()
                .filter(|&m| !modules.contains_key(m))
                .cloned()
                .collect()
        })
        .collect();
    output_modules
        .iter()
        .filter(|x| !x.is_empty())
        .for_each(|m| {
            modules.insert(
                m.clone(),
                Module {
                    kind: ModuleType::Output,
                    name: m.clone(),
                    outputs: vec![],
                    conj_state: HashMap::new(),
                    flip_flop_state: FlipFlopState::Off,
                    count: vec![0, 0],
                },
            );
        });
    modules
}

fn propagate(modules: &mut HashMap<String, Module>, signals: &[Signal]) -> Vec<Signal> {
    signals
        .iter()
        .flat_map(|signal| propagate_signal(modules, signal))
        .collect()
}

fn propagate_signal(modules: &mut HashMap<String, Module>, signal: &Signal) -> Vec<Signal> {
    // dbg!(signal);
    let dest = modules.get_mut(&signal.dst).unwrap();
    match dest.kind {
        ModuleType::FlipFlop => flip_flop_propagate(dest),
        ModuleType::Conjunction => conjunction_propagate(dest, &signal.src),
        ModuleType::Broadcast => broadcast_propagate(dest),
        ModuleType::Output => vec![],
    }
}

fn flip_flop_propagate(module: &mut Module) -> Vec<Signal> {
    return module
        .outputs
        .iter()
        .map(|m| Signal {
            pulse: Pulse::Low,
            src: module.name.clone(),
            dst: m.clone(),
        })
        .collect();
}

fn conjunction_propagate(module: &mut Module, source: &str) -> Vec<Signal> {
    module
        .conj_state
        .entry(source.to_owned())
        .or_insert(Pulse::Low);

    module
        .outputs
        .iter()
        .map(|m| Signal {
            pulse: Pulse::Low,
            src: module.name.clone(),
            dst: m.clone(),
        })
        .collect()
}

fn broadcast_propagate(module: &mut Module) -> Vec<Signal> {
    return module
        .outputs
        .iter()
        .map(|m| Signal {
            pulse: Pulse::Low,
            src: module.name.clone(),
            dst: m.clone(),
        })
        .collect();
}

///////

fn process(modules: &mut HashMap<String, Module>, signals: &[Signal]) -> Vec<Signal> {
    // dbg!(signals);
    signals
        .iter()
        .flat_map(|signal| process_signal(modules, signal))
        .collect()
}

fn process_signal(modules: &mut HashMap<String, Module>, signal: &Signal) -> Vec<Signal> {
    // dbg!(signal);
    let dest = modules.get_mut(&signal.dst).unwrap();
    match dest.kind {
        ModuleType::FlipFlop => flip_flop_process(dest, signal.pulse),
        ModuleType::Conjunction => conjunction_process(dest, signal.pulse, &signal.src),
        ModuleType::Broadcast => broadcast_process(dest, signal.pulse),
        ModuleType::Output => broadcast_process(dest, signal.pulse),
    }
}

fn flip_flop_process(module: &mut Module, pulse: Pulse) -> Vec<Signal> {
    if pulse == Pulse::High {
        return vec![];
    }
    let to_send = match module.flip_flop_state {
        FlipFlopState::Off => {
            module.flip_flop_state = FlipFlopState::On;
            Pulse::High
        }
        FlipFlopState::On => {
            module.flip_flop_state = FlipFlopState::Off;
            Pulse::Low
        }
    };
    module.count[to_send as usize] += module.outputs.len();
    return module
        .outputs
        .iter()
        .map(|m| Signal {
            pulse: to_send,
            src: module.name.clone(),
            dst: m.clone(),
        })
        .collect();
}

fn conjunction_process(module: &mut Module, pulse: Pulse, source: &str) -> Vec<Signal> {
    let state = module
        .conj_state
        .entry(source.to_owned())
        .or_insert(Pulse::Low);
    *state = pulse;
    let to_send = match module.conj_state.iter().all(|(_, v)| *v == Pulse::High) {
        true => Pulse::Low,
        false => Pulse::High,
    };
    // dbg!(&module, to_send);
    module.count[to_send as usize] += module.outputs.len();
    module
        .outputs
        .iter()
        .map(|m| Signal {
            pulse: to_send,
            src: module.name.clone(),
            dst: m.clone(),
        })
        .collect()
}

fn broadcast_process(module: &mut Module, pulse: Pulse) -> Vec<Signal> {
    module.count[pulse as usize] += module.outputs.len();
    return module
        .outputs
        .iter()
        .map(|m| Signal {
            pulse,
            src: module.name.clone(),
            dst: m.clone(),
        })
        .collect();
}

fn button_press(modules: &mut HashMap<String, Module>) {
    let mut next = process(
        modules,
        &vec![Signal {
            pulse: Pulse::Low,
            dst: "broadcaster".to_string(),
            src: "button".to_string(),
        }],
    );

    while !next.is_empty() {
        // dbg!(&next);
        next = process(modules, &next);
    }
    // dbg!(modules);
}

fn setup(modules: &mut HashMap<String, Module>) {
    let next: Vec<Signal> = modules
        .iter()
        .map(|(name, module)| {
            module
                .outputs
                .iter()
                .map(|m| Signal {
                    pulse: Pulse::Low,
                    src: name.clone(),
                    dst: m.clone(),
                })
                .collect::<Vec<Signal>>()
        })
        .flatten()
        .collect();

    // dbg!(&next);
    propagate(modules, &next);
    // dbg!(modules);
}

fn solve_p1(input: &str) -> usize {
    let iterations = 1000;
    let mut modules = parse_input(input);

    setup(&mut modules);

    (0..iterations).for_each(|_| button_press(&mut modules));

    let low: usize = modules
        .iter()
        .map(|(_, v)| v.count[Pulse::Low as usize])
        .sum::<usize>()
        + iterations;
    let high: usize = modules
        .iter()
        .map(|(_, v)| v.count[Pulse::High as usize])
        .sum();
    low * high
}

fn solve_p2(input: &str) -> usize {
    let mut iterations = 0;
    let mut modules = parse_input(input);

    setup(&mut modules);

    let mut zhwatch: HashMap<String, usize> = HashMap::new();
    let zh = modules.get("zh").unwrap();
    zh.conj_state.iter().for_each(|(k, _)| {
        zhwatch.insert(k.clone(), 0);
    });

    loop {
        button_press(&mut modules);
        iterations += 1;
        if modules.get("rx").unwrap().count[Pulse::Low as usize] > 0 {
            break;
        }
        let zh = modules.get("zh").unwrap();
        zh.conj_state.iter().for_each(|(k, v)| {
            if *v == Pulse::High {
                let watcher = zhwatch.get_mut(k).unwrap();
                if *watcher == 0 {
                    *watcher = iterations;
                    dbg!(&zhwatch);
                }
            }
        });
        if zhwatch.iter().all(|(_, v)| *v != 0) {
            break;
        }
    }

    if modules.get("rx").unwrap().count[Pulse::Low as usize] == 0 {
        return zhwatch
            .values()
            .cloned()
            .reduce(|acc, d| acc.lcm(&d))
            .unwrap();
    }
    iterations
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
        broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a";

    const INPUT2: &str = "
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output";

    #[test]
    fn test_parse_module() {
        let module = Module::from_str("%a -> b").unwrap();
        assert_eq!(module.kind, ModuleType::FlipFlop);
        assert_eq!(module.outputs.len(), 1);
    }

    #[test]
    fn test_parse_input() {
        let modules = parse_input(INPUT);
        // dbg!(&modules);
        assert_eq!(modules.len(), 5);
    }

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 32000000);
        let answer = solve_p1(INPUT2);
        assert_eq!(answer, 11687500);
    }

    #[test]
    fn test_solve() {
        let input = include_str!("../input.txt");
        let answer = solve_p1(input);
        assert_eq!(answer, 886701120);
    }
}
