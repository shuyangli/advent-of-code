use once_cell::sync::Lazy;
use regex::Regex;

use crate::day::Day;
use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    fmt::Display,
    rc::Rc,
};

pub struct Day19 {}

#[derive(Clone, Copy, Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Debug)]
struct Condition {
    part: char,
    op: char,
    value: u64,
}

impl Condition {
    fn matches(&self, part: &Part) -> bool {
        let part_to_compare = match self.part {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => panic!("Invalid part instruction!"),
        };
        match self.op {
            '>' => part_to_compare > self.value,
            '<' => part_to_compare < self.value,
            _ => panic!("Invalid comparison operator!"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum NextStep {
    Accept,
    Reject,
    Goto(String),
}

#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    next_step: NextStep,
}

impl Rule {
    fn execute(&self, part: &Part) -> Option<&NextStep> {
        match &self.condition {
            Some(c) => {
                if c.matches(part) {
                    Some(&self.next_step)
                } else {
                    None
                }
            }
            None => Some(&self.next_step),
        }
    }
}

fn parse_condition(input: &str) -> Condition {
    let (part, rest) = input.split_at(1);
    let (op, value) = rest.split_at(1);
    Condition {
        part: part.chars().nth(0).unwrap(),
        op: op.chars().nth(0).unwrap(),
        value: value.parse().unwrap(),
    }
}

fn parse_next_step(input: &str) -> NextStep {
    match input {
        "A" => NextStep::Accept,
        "R" => NextStep::Reject,
        n => NextStep::Goto(n.to_string()),
    }
}

fn parse_rules(input: &str) -> HashMap<String, Vec<Rule>> {
    let mut rules = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let (label, rest) = line.split_once('{').unwrap();
        let rest = rest.trim_end_matches('}');

        for rule_text in rest.split(",") {
            let rule = if let Some((condition, next_step)) = rule_text.split_once(":") {
                Rule {
                    condition: Some(parse_condition(condition)),
                    next_step: parse_next_step(next_step),
                }
            } else {
                Rule {
                    condition: None,
                    next_step: parse_next_step(rule_text),
                }
            };

            rules.entry(label.to_string()).or_insert(vec![]).push(rule);
        }
    }

    return rules;
}

static PARTS_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)\}").unwrap());
fn parse_parts(input: &str) -> Vec<Part> {
    let mut parts = vec![];
    let mut lines = input.lines();

    // Skip over the instructions.
    while let Some(l) = lines.next() {
        if l.is_empty() {
            break;
        }
    }

    while let Some(l) = lines.next() {
        let captures = PARTS_REGEX.captures(l).unwrap();
        parts.push(Part {
            x: captures["x"].parse().unwrap(),
            m: captures["m"].parse().unwrap(),
            a: captures["a"].parse().unwrap(),
            s: captures["s"].parse().unwrap(),
        })
    }

    return parts;
}

// These are pairs of [low, high), keyed by 'x', 'm', 'a', or 's'.
#[derive(Debug, Clone)]
struct ConstraintRange {
    low: u64,
    high: u64,
}

// These are pairs of [low, high), keyed by 'x', 'm', 'a', or 's'.
#[derive(Debug, Clone)]
struct Constraints(HashMap<char, ConstraintRange>);

impl Constraints {
    fn world() -> Self {
        let mut ranges = HashMap::new();
        ranges.insert('x', ConstraintRange { low: 1, high: 4001 });
        ranges.insert('m', ConstraintRange { low: 1, high: 4001 });
        ranges.insert('a', ConstraintRange { low: 1, high: 4001 });
        ranges.insert('s', ConstraintRange { low: 1, high: 4001 });

        Self(ranges)
    }

    fn possible_values(&self) -> u64 {
        self.0
            .iter()
            .map(|(_, r)| if r.high <= r.low { 0 } else { r.high - r.low })
            .product()
    }
}

// A partition tree. Root is the entire world, leaves are nodes with acceptance conditions.
#[derive(Debug)]
struct Partition {
    partitions: Vec<Rc<RefCell<Partition>>>,
    constraints: Option<Constraints>,
    acceptance: Option<NextStep>,
}

impl Partition {
    fn new() -> Self {
        Self {
            constraints: None,
            partitions: vec![],
            acceptance: None,
        }
    }
}

// DFS through the ruleset, creating a tree of partitions ending in leaves.
fn build_partition(ruleset: &HashMap<String, Vec<Rule>>) -> Partition {
    let mut root = Partition::new();

    fn build_partition_helper(
        ruleset: &HashMap<String, Vec<Rule>>,
        parent_constraint: &Constraints,
        root: &mut Partition,
        rule_label: &str,
    ) {
        let rule = &ruleset[rule_label];
        let mut current_constraint = parent_constraint.clone();

        for r in rule {
            let mut next_constraint = current_constraint.clone();

            if let Some(condition) = &r.condition {
                match condition.op {
                    '<' => {
                        // Low is inclusive
                        current_constraint.0.get_mut(&condition.part).unwrap().high =
                            condition.value;
                        next_constraint.0.get_mut(&condition.part).unwrap().low = condition.value;
                    }
                    '>' => {
                        // High is exclusive
                        current_constraint.0.get_mut(&condition.part).unwrap().low =
                            condition.value + 1;
                        next_constraint.0.get_mut(&condition.part).unwrap().high =
                            condition.value + 1;
                    }
                    _ => panic!("Unexpected operator."),
                }
            }

            let mut new_partition = Partition::new();
            match &r.next_step {
                NextStep::Accept | NextStep::Reject => {
                    new_partition.acceptance = Some(r.next_step.clone());
                    new_partition.constraints = Some(current_constraint);
                    root.partitions.push(Rc::new(RefCell::new(new_partition)));
                }
                NextStep::Goto(label) => {
                    build_partition_helper(
                        ruleset,
                        &current_constraint,
                        &mut new_partition,
                        &label,
                    );
                    root.partitions.push(Rc::new(RefCell::new(new_partition)));
                }
            }

            current_constraint = next_constraint;
        }
    }

    build_partition_helper(ruleset, &Constraints::world(), &mut root, "in");

    return root;
}

fn get_possible_values_in_partition(partition: &Partition) -> u64 {
    if let Some(acceptance) = &partition.acceptance {
        if acceptance == &NextStep::Accept {
            return partition.constraints.as_ref().unwrap().possible_values();
        } else {
            return 0;
        }
    }

    return partition
        .partitions
        .iter()
        .map(|child| get_possible_values_in_partition(&child.borrow()))
        .sum::<u64>();
}

impl Day for Day19 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let all_rules = parse_rules(input);
        let all_parts = parse_parts(input);

        let answer: u64 = all_parts
            .iter()
            .map(|part| {
                let mut rule_label: String = "in".to_string();

                loop {
                    let rule = &all_rules[&rule_label];
                    'rule_loop: for r in rule {
                        if let Some(next_step) = r.execute(part) {
                            match next_step {
                                NextStep::Accept | NextStep::Reject => return (part, next_step),
                                NextStep::Goto(label) => {
                                    rule_label = label.clone();
                                    break 'rule_loop;
                                }
                            }
                        }
                    }
                }
            })
            .filter(|(_, n)| n == &&NextStep::Accept)
            .map(|(p, _)| p.x + p.m + p.a + p.s)
            .sum();

        return Ok(Box::new(answer));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let all_rules = parse_rules(input);

        let root = build_partition(&all_rules);
        return Ok(Box::new(get_possible_values_in_partition(&root)));
    }
}
