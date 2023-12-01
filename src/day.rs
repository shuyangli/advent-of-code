use std::fmt::Display;

pub trait Day<T: Display> {
    fn part1(&self, input: &str) -> Result<T, &str>;
    fn part2(&self, input: &str) -> Result<T, &str>;
}
