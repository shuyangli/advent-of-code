use crate::day::Day;
use std::fmt::Display;

pub struct Day6 {}

struct RaceRecord {
    time: u64,
    distance: u64,
}

impl RaceRecord {
    fn num_of_winning_approaches(&self) -> u64 {
        // We're solving for x * (time - x) > distance
        // -x^2 + x*time - distance > 0
        // x = (t +- sqrt(t^2 - 4d)) / 2
        let sqrt_term = self.time.pow(2) as f32 - 4_f32 * self.distance as f32;
        if sqrt_term < 0_f32 {
            return 0;
        }

        let sqrt_term = sqrt_term.sqrt();
        let min_solution = (self.time as f32 - sqrt_term) / 2_f32;
        let max_solution = (self.time as f32 + sqrt_term) / 2_f32;

        // Round up min_solution, round down max_solution
        let max_time: u64 = if max_solution - max_solution.floor() < f32::EPSILON {
            max_solution as u64 - 1
        } else {
            max_solution.floor() as u64
        };
        let min_time: u64 = if min_solution.ceil() - min_solution < f32::EPSILON {
            min_solution as u64 + 1
        } else {
            min_solution.ceil() as u64
        };

        if max_time >= min_time {
            1 + max_time - min_time
        } else {
            0
        }
    }
}

fn parse_race_records_part_1(input: &str) -> Vec<RaceRecord> {
    let mut lines = input.lines();
    let times = lines.next().unwrap().split_ascii_whitespace().skip(1);
    let distances = lines.next().unwrap().split_ascii_whitespace().skip(1);

    return times
        .zip(distances)
        .map(|(time, distance)| RaceRecord {
            time: time.parse().unwrap(),
            distance: distance.parse().unwrap(),
        })
        .collect();
}

fn parse_race_records_part_2(input: &str) -> RaceRecord {
    let mut lines = input.lines();
    let mut time = String::new();
    lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .for_each(|elem| time.push_str(elem));

    let mut distance = String::new();
    lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .for_each(|elem| distance.push_str(elem));

    return RaceRecord {
        time: time.parse().unwrap(),
        distance: distance.parse().unwrap(),
    };
}

impl Day for Day6 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let races = parse_race_records_part_1(input);
        let number_of_approaches: u64 = races
            .iter()
            .map(|race| race.num_of_winning_approaches())
            .product();

        return Ok(Box::new(number_of_approaches));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let race = parse_race_records_part_2(input);
        return Ok(Box::new(race.num_of_winning_approaches()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[googletest::test]
    fn computes_winning_solutions_correctly() {
        expect_that!(
            RaceRecord {
                time: 1,
                distance: 1
            }
            .num_of_winning_approaches(),
            eq(0)
        );
        expect_that!(
            RaceRecord {
                time: 2,
                distance: 1
            }
            .num_of_winning_approaches(),
            eq(0)
        );
        expect_that!(
            RaceRecord {
                time: 3,
                distance: 1
            }
            .num_of_winning_approaches(),
            eq(2)
        );
        expect_that!(
            RaceRecord {
                time: 4,
                distance: 1
            }
            .num_of_winning_approaches(),
            eq(3)
        );

        expect_that!(
            RaceRecord {
                time: 7,
                distance: 9
            }
            .num_of_winning_approaches(),
            eq(4)
        );

        expect_that!(
            RaceRecord {
                time: 15,
                distance: 40
            }
            .num_of_winning_approaches(),
            eq(8)
        );

        expect_that!(
            RaceRecord {
                time: 30,
                distance: 200
            }
            .num_of_winning_approaches(),
            eq(9)
        );
    }
}
