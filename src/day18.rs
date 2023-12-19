use crate::{
    common::{coordinates::Coordinates, direction::Direction},
    day::Day,
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::Display;
pub struct Day18 {}

#[derive(Clone)]
struct DigStep {
    direction: Direction,
    steps: i64,
}

static INSTRUCTION_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?<dir>R|D|L|U) (?<steps>\d+) \(#(?<hex>.+)\)").unwrap());
fn parse_dig_steps_part1(input: &str) -> Vec<DigStep> {
    input
        .lines()
        .map(|l| {
            let caps = INSTRUCTION_REGEX.captures(l).unwrap();

            DigStep {
                direction: match &caps["dir"] {
                    "U" => Direction::North,
                    "D" => Direction::South,
                    "L" => Direction::West,
                    "R" => Direction::East,
                    d => panic!("Unexpected direction {d}"),
                },
                steps: caps["steps"].parse().unwrap(),
            }
        })
        .collect()
}

fn parse_dig_steps_part2(input: &str) -> Vec<DigStep> {
    input
        .lines()
        .map(|l| {
            let caps = INSTRUCTION_REGEX.captures(l).unwrap();

            let (hex_steps, direction) = caps["hex"].split_at(5);

            DigStep {
                direction: match direction {
                    "0" => Direction::West,
                    "1" => Direction::South,
                    "2" => Direction::East,
                    "3" => Direction::North,
                    d => panic!("Unexpected direction {d}"),
                },
                steps: <i64>::from_str_radix(hex_steps, 16).unwrap(),
            }
        })
        .collect()
}

fn compute_area_plus_border(steps: &Vec<DigStep>) -> i64 {
    let mut last_coordinates = Coordinates(0, 0);
    let mut interior_size = 0;

    // Just need the larger one of the two.
    let mut clockwise_exterior_size = 0_f64;
    let mut counterclockwise_exterior_size = 0_f64;
    let mut last_step = None;

    for step in steps.as_slice() {
        let coordinates = last_coordinates.step(step.direction, step.steps);

        if coordinates.0 != last_coordinates.0 {
            interior_size += (coordinates.0 - last_coordinates.0) * coordinates.1;
        }

        if last_step.is_none() {
            last_step = Some(step.clone());
            clockwise_exterior_size += step.steps as f64 * 0.5;
            counterclockwise_exterior_size += step.steps as f64 * 0.5;
            continue;
        }

        if last_step.unwrap().direction.rotate(90) == step.direction {
            clockwise_exterior_size += step.steps as f64 * 0.5 + 0.25;
            counterclockwise_exterior_size += step.steps as f64 * 0.5 - 0.25;
        } else {
            clockwise_exterior_size += step.steps as f64 * 0.5 - 0.25;
            counterclockwise_exterior_size += step.steps as f64 * 0.5 + 0.25;
        }

        last_step = Some(step.clone());
        last_coordinates = coordinates;
    }

    // Compute if we should add or substract 0.25 for completing the loop.
    if last_step.unwrap().direction.rotate(90) == steps.first().unwrap().direction {
        clockwise_exterior_size += 0.25;
        counterclockwise_exterior_size -= 0.25;
    } else {
        clockwise_exterior_size -= 0.25;
        counterclockwise_exterior_size += 0.25;
    }

    let is_clockwise = interior_size > 0;

    return interior_size.abs()
        + if is_clockwise {
            clockwise_exterior_size as i64
        } else {
            counterclockwise_exterior_size as i64
        };
}

impl Day for Day18 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let steps = parse_dig_steps_part1(input);
        return Ok(Box::new(compute_area_plus_border(&steps)));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let steps = parse_dig_steps_part2(input);
        return Ok(Box::new(compute_area_plus_border(&steps)));
    }
}
