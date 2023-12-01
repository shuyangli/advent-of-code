use std::fmt::Display;

pub trait Day {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str>;
    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str>;
}
