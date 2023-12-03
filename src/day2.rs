use crate::day::Day;
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::Display;

pub struct Day2 {}

impl Day for Day2 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let red_limit = 12;
        let green_limit = 13;
        let blue_limit = 14;

        let mut sum_of_possible_game_ids = 0;

        for line in input.lines() {
            let game = parse_game_or_panic(line);
            if game.is_valid(red_limit, green_limit, blue_limit) {
                sum_of_possible_game_ids += game.id;
            }
        }
        return Ok(Box::new(sum_of_possible_game_ids));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let mut sum_of_powers = 0;
        for line in input.lines() {
            let game = parse_game_or_panic(line);
            let (red_limit, green_limit, blue_limit) = game.get_fewest_cubes();
            sum_of_powers += red_limit * green_limit * blue_limit;
        }
        return Ok(Box::new(sum_of_powers));
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Operation {
    red_count: i32,
    green_count: i32,
    blue_count: i32,
}
impl Operation {
    fn new() -> Operation {
        Operation {
            red_count: 0,
            green_count: 0,
            blue_count: 0,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Game {
    id: i32,
    operations: Vec<Operation>,
}

impl Game {
    fn is_valid(&self, red_limit: i32, green_limit: i32, blue_limit: i32) -> bool {
        for op in &self.operations {
            if op.red_count > red_limit
                || op.green_count > green_limit
                || op.blue_count > blue_limit
            {
                return false;
            }
        }
        return true;
    }

    fn get_fewest_cubes(&self) -> (i32, i32, i32) {
        let mut red_limit = 0;
        let mut green_limit = 0;
        let mut blue_limit = 0;
        for op in &self.operations {
            if op.red_count > red_limit {
                red_limit = op.red_count;
            }
            if op.green_count > green_limit {
                green_limit = op.green_count;
            }
            if op.blue_count > blue_limit {
                blue_limit = op.blue_count;
            }
        }
        return (red_limit, green_limit, blue_limit);
    }
}

static GAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"Game (?<game_id>\d+): (?<sequence>.*)$").unwrap());
static ROUND_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?<count>\d+) (?<color>red|green|blue)").unwrap());

fn parse_game_or_panic(line: &str) -> Game {
    let Some(caps) = GAME_REGEX.captures(line) else {
        panic!("Error parsing game!")
    };
    let game_id = caps["game_id"]
        .parse::<i32>()
        .expect("Error parsing game ID!");

    let mut game = Game {
        id: game_id,
        operations: vec![],
    };

    let game_sequence = &caps["sequence"];
    for round in game_sequence.split(';') {
        let mut operation = Operation::new();
        for (_, [count, color]) in ROUND_REGEX.captures_iter(round).map(|c| c.extract()) {
            let count = count.parse::<i32>().expect("Error parsing cube count");
            match color {
                "red" => operation.red_count += count,
                "green" => operation.green_count += count,
                "blue" => operation.blue_count += count,
                _ => panic!("Unknown color"),
            };
        }
        game.operations.push(operation);
    }

    return game;
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[googletest::test]
    fn parses_game_correctly() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected_game = Game {
            id: 1,
            operations: vec![
                Operation {
                    red_count: 4,
                    blue_count: 3,
                    green_count: 0,
                },
                Operation {
                    red_count: 1,
                    blue_count: 6,
                    green_count: 2,
                },
                Operation {
                    red_count: 0,
                    blue_count: 0,
                    green_count: 2,
                },
            ],
        };

        expect_that!(parse_game_or_panic(game), eq(expected_game));
    }
}
