use crate::day::Day;
use std::fmt::Display;

pub struct Day9 {}

struct Series {
    nums: Vec<i32>,
}

impl Series {
    fn parse_from_line(line: &str) -> Self {
        Series {
            nums: line
                .split_ascii_whitespace()
                .map(|f| f.parse().unwrap())
                .collect(),
        }
    }

    fn construct_stack(&self) -> Vec<Vec<i32>> {
        let mut stack = vec![];

        stack.push(self.nums.clone());
        while stack.last().unwrap().iter().any(|v| *v != 0) {
            let next_vec = stack.last().unwrap();
            stack.push(
                next_vec
                    .iter()
                    .zip(next_vec.iter().skip(1))
                    .map(|(a, b)| b - a)
                    .collect(),
            );
        }

        // Pop the all-0 one
        stack.pop();

        return stack;
    }

    fn get_next_value(&self) -> i32 {
        let mut stack = self.construct_stack();
        let mut last_value = 0;

        while !stack.is_empty() {
            last_value = stack.pop().unwrap().last().unwrap() + last_value;
        }

        return last_value;
    }

    fn get_previous_value(&self) -> i32 {
        let mut stack = self.construct_stack();
        let mut previsou_value = 0;

        while !stack.is_empty() {
            previsou_value = stack.pop().unwrap().first().unwrap() - previsou_value;
        }

        return previsou_value;
    }
}

impl Day for Day9 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let answer: i32 = input
            .lines()
            .map(|l| Series::parse_from_line(l))
            .map(|series| series.get_next_value())
            .sum();
        return Ok(Box::new(answer));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let answer: i32 = input
            .lines()
            .map(|l| Series::parse_from_line(l))
            .map(|series| series.get_previous_value())
            .sum();
        return Ok(Box::new(answer));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;
}
