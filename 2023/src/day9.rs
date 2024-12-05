use crate::day::Day;
use std::fmt::Display;

pub struct Day9 {}

struct Series {
    nums: Vec<i32>,
}

fn compute_differences(nums: &Vec<i32>) -> Vec<i32> {
    nums.iter()
        .zip(nums.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect()
}

fn get_next(nums: &Vec<i32>) -> i32 {
    if nums.iter().all(|f| *f == 0) {
        return 0;
    }

    nums.last().unwrap() + get_next(&compute_differences(nums))
}

fn get_previous(nums: &Vec<i32>) -> i32 {
    if nums.iter().all(|f| *f == 0) {
        return 0;
    }
    nums.first().unwrap() - get_previous(&compute_differences(nums))
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
}

impl Day for Day9 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let answer: i32 = input
            .lines()
            .map(|l| Series::parse_from_line(l))
            .map(|series| get_next(&series.nums))
            .sum();
        return Ok(Box::new(answer));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let answer: i32 = input
            .lines()
            .map(|l| Series::parse_from_line(l))
            .map(|series| get_previous(&series.nums))
            .sum();
        return Ok(Box::new(answer));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;
}
