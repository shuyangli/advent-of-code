use std::fmt::Display;

pub trait Day {
    fn part1(&self, _input: &str) -> Result<Box<dyn Display>, &str> {
        return Err("Unimplemented!");
    }
    fn part2(&self, _input: &str) -> Result<Box<dyn Display>, &str> {
        return Err("Unimplemented!");
    }
}
