use std::fmt::Display;

pub enum DayResult {
    Ok(Box<dyn Display>),
    Err(String),
}

pub trait Day {
    fn part1(&self, input: &str) -> DayResult;
    fn part2(&self, input: &str) -> DayResult;
}
