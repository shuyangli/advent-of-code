use num::Integer;

use crate::day::Day;
use std::{collections::HashMap, fmt::Display};

pub struct Day8 {}

#[derive(PartialEq, Eq, Debug)]
struct MapEntry {
    left: String,
    right: String,
}

impl MapEntry {
    fn get_next_stop(&self, step: char) -> &str {
        match step {
            'L' => &self.left,
            'R' => &self.right,
            _ => panic!("Unexpected instruction"),
        }
    }
}

impl Day for Day8 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let mut lines = input.lines();

        let mut instructions = lines.next().unwrap().chars().into_iter().cycle();

        let mut map = HashMap::new();

        lines.next();
        while let Some(line) = lines.next() {
            let items: Vec<_> = line
                .split([' ', '=', '(', ',', ')'])
                .filter(|s| !s.is_empty())
                .collect();

            map.insert(
                items[0],
                MapEntry {
                    left: items[1].to_string(),
                    right: items[2].to_string(),
                },
            );
        }

        let mut current_stop = "AAA";
        let mut num_stops = 0;
        while current_stop != "ZZZ" {
            current_stop = map[current_stop].get_next_stop(instructions.next().unwrap());
            num_stops += 1;
        }

        return Ok(Box::new(num_stops));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let mut lines = input.lines();

        let instructions = lines.next().unwrap().chars().into_iter().cycle();

        let mut map = HashMap::new();
        let mut starting_points = vec![];

        lines.next();
        while let Some(line) = lines.next() {
            let items: Vec<_> = line
                .split([' ', '=', '(', ',', ')'])
                .filter(|s| !s.is_empty())
                .collect();

            map.insert(
                items[0],
                MapEntry {
                    left: items[1].to_string(),
                    right: items[2].to_string(),
                },
            );

            if items[0].ends_with('A') {
                starting_points.push(items[0]);
            }
        }

        let number_of_stops_per_point: Vec<_> = starting_points
            .iter()
            .map(|starting_point| {
                let mut instr = instructions.clone();
                let mut current_stop = *starting_point;
                let mut num_stops = 0_i64;
                while !current_stop.ends_with('Z') {
                    current_stop = map[current_stop].get_next_stop(instr.next().unwrap());
                    num_stops += 1;
                }
                num_stops
            })
            .collect();

        return Ok(Box::new(
            number_of_stops_per_point
                .iter()
                .fold(1, |acc, n| acc.lcm(n)),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;
}
