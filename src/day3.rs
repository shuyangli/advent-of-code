use crate::day::Day;
use std::{collections::HashMap, fmt::Display};

#[derive(PartialEq, Eq, Debug, Hash)]
struct Gear {
    character: char,
    position: (usize, usize),
}

pub struct Day3 {}

fn get_adjacent_gear(
    starting_index: i32,
    ending_index: i32,
    line_number: i32,
    grid: &Vec<Vec<char>>,
) -> Option<Gear> {
    let left = std::cmp::max(0, starting_index - 1) as usize;
    let right = std::cmp::min(ending_index + 1, grid[0].len() as i32 - 1) as usize;
    let first_line = std::cmp::max(0, line_number - 1) as usize;
    let last_line = std::cmp::min(line_number + 1, grid.len() as i32 - 1) as usize;

    for i in first_line..=last_line {
        for j in left..=right {
            let c = grid[i][j];
            if !c.is_digit(10) && c != '.' {
                return Some(Gear {
                    character: c,
                    position: (i, j),
                });
            }
        }
    }
    return None;
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    return input.lines().map(|l| l.chars().collect()).collect();
}

impl Day for Day3 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let characters = parse_grid(input);
        let mut sum_part_numbers = 0;

        for (i, line) in characters.iter().enumerate() {
            let mut number_buffer = String::new();
            let mut starting_index: Option<i32> = None;
            for (j, char) in line.iter().enumerate() {
                if char.is_numeric() {
                    if starting_index.is_none() {
                        starting_index = Some(j as i32);
                    }
                    number_buffer.push(*char);
                    continue;
                }

                if number_buffer.is_empty() || starting_index.is_none() {
                    continue;
                }

                // This is the end of a number; check if it's next to a gear.
                if let Some(_) =
                    get_adjacent_gear(starting_index.unwrap(), j as i32 - 1, i as i32, &characters)
                {
                    sum_part_numbers += number_buffer
                        .parse::<i32>()
                        .expect("Failed to parse number");
                }

                starting_index = None;
                number_buffer.clear();
            }

            // This is the end of a line, validate if the number is next to a gear
            if !number_buffer.is_empty() && starting_index.is_some() {
                if let Some(_) = get_adjacent_gear(
                    starting_index.unwrap(),
                    line.len() as i32 - 1,
                    i as i32,
                    &characters,
                ) {
                    sum_part_numbers += number_buffer
                        .parse::<i32>()
                        .expect("Failed to parse number");
                }
            }
        }

        return Ok(Box::new(sum_part_numbers));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let characters = parse_grid(input);
        let mut gears: HashMap<Gear, Vec<i32>> = HashMap::new();

        for (i, line) in characters.iter().enumerate() {
            let mut number_buffer = String::new();
            let mut starting_index: Option<i32> = None;
            for (j, char) in line.iter().enumerate() {
                if char.is_numeric() {
                    if starting_index.is_none() {
                        starting_index = Some(j as i32);
                    }
                    number_buffer.push(*char);
                    continue;
                }

                if number_buffer.is_empty() || starting_index.is_none() {
                    continue;
                }
                let number = number_buffer.parse().expect("Failed to parse number");

                // This is the end of a number; process the gears.
                if let Some(gear) =
                    get_adjacent_gear(starting_index.unwrap(), j as i32 - 1, i as i32, &characters)
                {
                    match gears.get_mut(&gear) {
                        Some(numbers) => {
                            numbers.push(number);
                        }
                        None => {
                            gears.insert(gear, vec![number]);
                        }
                    }
                }
                starting_index = None;
                number_buffer.clear();
            }
            // This is the end of a line; process the gears.
            if !number_buffer.is_empty() && starting_index.is_some() {
                let number = number_buffer.parse().expect("Failed to parse number");
                if let Some(gear) = get_adjacent_gear(
                    starting_index.unwrap(),
                    line.len() as i32 - 1,
                    i as i32,
                    &characters,
                ) {
                    match gears.get_mut(&gear) {
                        Some(numbers) => {
                            numbers.push(number);
                        }
                        None => {
                            gears.insert(gear, vec![number]);
                        }
                    }
                }
            }
        }

        let mut sum_gear_ratios = 0;
        for (gear, numbers) in gears {
            if gear.character == '*' && numbers.len() == 2 {
                sum_gear_ratios += numbers.iter().fold(1, |acc, elem| acc * elem);
            }
        }
        return Ok(Box::new(sum_gear_ratios));
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use googletest::prelude::*;
}
