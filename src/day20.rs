use crate::day::Day;
use itertools::Itertools;
use num::Integer;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

pub struct Day20 {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Module {
    Broadcast {
        name: String,
        descendants: Vec<String>,
    },
    FlipFlop {
        name: String,
        is_on: bool,
        descendants: Vec<String>,
    },
    Conjunction {
        name: String,
        inputs: HashMap<String, Pulse>,
        descendants: Vec<String>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct PulseAction {
    source: String,
    pulse: Pulse,
    destination: String,
}

impl Module {
    fn get_descendants(&self) -> &Vec<String> {
        match self {
            Self::Broadcast {
                name: _,
                descendants,
            } => descendants,
            Self::FlipFlop {
                name: _,
                is_on: _,
                descendants,
            } => descendants,
            Self::Conjunction {
                name: _,
                inputs: _,
                descendants,
            } => descendants,
        }
    }

    fn handle_pulse(&mut self, source: String, pulse: Pulse) -> Vec<PulseAction> {
        match self {
            Self::Broadcast { name, descendants } => descendants
                .iter()
                .map(|destination| PulseAction {
                    source: name.clone(),
                    pulse,
                    destination: destination.clone(),
                })
                .collect(),
            Self::FlipFlop {
                name,
                is_on,
                descendants,
            } => {
                if pulse == Pulse::High {
                    vec![]
                } else {
                    let results = descendants
                        .iter()
                        .map(|destination| PulseAction {
                            source: name.clone(),
                            pulse: if *is_on { Pulse::Low } else { Pulse::High },
                            destination: destination.clone(),
                        })
                        .collect();
                    *is_on = !*is_on;
                    results
                }
            }
            Self::Conjunction {
                name,
                inputs,
                descendants,
            } => {
                inputs.insert(source.to_string(), pulse);
                let pulse_to_send = if inputs.values().all(|&pulse| pulse == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                descendants
                    .iter()
                    .map(|destination| PulseAction {
                        source: name.clone(),
                        pulse: pulse_to_send,
                        destination: destination.clone(),
                    })
                    .collect()
            }
        }
    }
}

static MODULE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?<type>[%&])?(?<name>.+) -> (?<dests>.+)$").unwrap());
fn parse_modules(input: &str) -> HashMap<String, Module> {
    let mut modules = HashMap::new();

    // Parse each module
    for line in input.lines() {
        let captures = MODULE_REGEX.captures(line).unwrap();

        let name = captures["name"].to_string();
        let descendants = captures["dests"]
            .split(", ")
            .map(|d| d.to_string())
            .collect_vec();

        let new_module = match captures.name("type").map(|t| t.as_str()) {
            Some("%") => Module::FlipFlop {
                name: name.clone(),
                is_on: false,
                descendants,
            },
            Some("&") => Module::Conjunction {
                name: name.clone(),
                inputs: HashMap::new(),
                descendants,
            },
            None => {
                if name == "broadcaster" {
                    Module::Broadcast {
                        name: name.clone(),
                        descendants,
                    }
                } else {
                    panic!("Unexpected module name {} without type", name);
                }
            }
            _ => panic!("Unexpected capture value for module type!"),
        };
        modules.insert(name, new_module);
    }

    // Initialize each broadcast module to include its predecessor
    // How do I get around the borrow checker here?
    for (name, module) in modules.clone().iter() {
        for descendant in module.get_descendants() {
            match modules.get_mut(descendant) {
                Some(Module::Conjunction {
                    inputs,
                    name: _,
                    descendants: _,
                }) => {
                    inputs.insert(name.clone(), Pulse::Low);
                }
                _ => {}
            }
        }
    }

    return modules;
}

impl Day for Day20 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let mut modules = parse_modules(input);

        let mut next_pulses = VecDeque::new();
        let mut num_low_pulses = 0_i64;
        let mut num_high_pulses = 0_i64;

        for _ in 0..1000 {
            next_pulses.push_back(PulseAction {
                source: "button".to_string(),
                destination: "broadcaster".to_string(),
                pulse: Pulse::Low,
            });

            while let Some(next_pulse) = next_pulses.pop_front() {
                match next_pulse.pulse {
                    Pulse::Low => {
                        num_low_pulses += 1;
                    }
                    Pulse::High => {
                        num_high_pulses += 1;
                    }
                }

                // Some destinations may not have any additional downstreams.
                if !modules.contains_key(&next_pulse.destination) {
                    continue;
                }

                for n in modules
                    .get_mut(&next_pulse.destination)
                    .unwrap()
                    .handle_pulse(next_pulse.source, next_pulse.pulse)
                {
                    next_pulses.push_back(n);
                }
            }
        }

        return Ok(Box::new(num_low_pulses * num_high_pulses));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let mut modules = parse_modules(input);

        let mut num_button_presses: i64 = 0;
        let mut next_pulses = VecDeque::new();

        // Wiring:
        // xc, th, pd, bp each needs to receive LOW
        // xc, th, pd, bp each sends HIGH to zh
        // zh sends LOW to rx
        let mut xc_low_size = None;
        let mut th_low_size = None;
        let mut pd_low_size = None;
        let mut bp_low_size = None;

        'main_loop: loop {
            num_button_presses += 1;
            next_pulses.push_back(PulseAction {
                source: "button".to_string(),
                destination: "broadcaster".to_string(),
                pulse: Pulse::Low,
            });

            while let Some(next_pulse) = next_pulses.pop_front() {
                if next_pulse.pulse == Pulse::Low && next_pulse.destination == "rx" {
                    return Ok(Box::new(num_button_presses));
                }

                if next_pulse.pulse == Pulse::Low {
                    match next_pulse.destination.as_str() {
                        "xc" => {
                            xc_low_size = Some(num_button_presses);
                        }
                        "th" => {
                            th_low_size = Some(num_button_presses);
                        }
                        "pd" => {
                            pd_low_size = Some(num_button_presses);
                        }
                        "bp" => {
                            bp_low_size = Some(num_button_presses);
                        }
                        _ => {}
                    }
                    if xc_low_size.is_some()
                        && th_low_size.is_some()
                        && pd_low_size.is_some()
                        && bp_low_size.is_some()
                    {
                        break 'main_loop;
                    }
                }

                // Some destinations may not have any additional downstreams.
                if !modules.contains_key(&next_pulse.destination) {
                    continue;
                }

                for n in modules
                    .get_mut(&next_pulse.destination)
                    .unwrap()
                    .handle_pulse(next_pulse.source, next_pulse.pulse)
                {
                    next_pulses.push_back(n);
                }
            }
        }

        return Ok(Box::new(
            xc_low_size
                .unwrap()
                .lcm(&th_low_size.unwrap())
                .lcm(&pd_low_size.unwrap())
                .lcm(&bp_low_size.unwrap()),
        ));
    }
}
