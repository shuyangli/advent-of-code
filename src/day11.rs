use crate::day::Day;
use std::fmt::Display;

pub struct Day11 {}

impl Day for Day11 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let galaxy: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let mut star_positions: Vec<(usize, usize)> = vec![];
        for (i, line) in galaxy.iter().enumerate() {
            for (j, &c) in line.iter().enumerate() {
                if c == '#' {
                    star_positions.push((i, j));
                }
            }
        }

        let double_width_rows: Vec<usize> = (0..galaxy.len())
            .filter(|i| galaxy[*i].iter().all(|c| *c == '.'))
            .collect();
        let double_width_columns: Vec<usize> = (0..galaxy[0].len())
            .filter(|j| galaxy.iter().all(|l| l[*j] == '.'))
            .collect();

        let mut sum_of_distances = 0;
        for i in 0..star_positions.len() {
            for j in (i + 1)..star_positions.len() {
                sum_of_distances +=
                    num::abs(star_positions[j].0 as i32 - star_positions[i].0 as i32)
                        + num::abs(star_positions[j].1 as i32 - star_positions[i].1 as i32);

                for &col in &double_width_rows {
                    if (star_positions[i].0 < col && col < star_positions[j].0)
                        || (star_positions[j].0 < col && col < star_positions[i].0)
                    {
                        sum_of_distances += 1;
                    }
                }
                for &col in &double_width_columns {
                    if (star_positions[i].1 < col && col < star_positions[j].1)
                        || (star_positions[j].1 < col && col < star_positions[i].1)
                    {
                        sum_of_distances += 1;
                    }
                }
            }
        }

        return Ok(Box::new(sum_of_distances));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let galaxy: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let mut star_positions: Vec<(usize, usize)> = vec![];
        for (i, line) in galaxy.iter().enumerate() {
            for (j, &c) in line.iter().enumerate() {
                if c == '#' {
                    star_positions.push((i, j));
                }
            }
        }

        let double_width_rows: Vec<usize> = (0..galaxy.len())
            .filter(|i| galaxy[*i].iter().all(|c| *c == '.'))
            .collect();
        let double_width_columns: Vec<usize> = (0..galaxy[0].len())
            .filter(|j| galaxy.iter().all(|l| l[*j] == '.'))
            .collect();

        let mut sum_of_distances = 0;
        for i in 0..star_positions.len() {
            for j in (i + 1)..star_positions.len() {
                sum_of_distances +=
                    num::abs(star_positions[j].0 as i64 - star_positions[i].0 as i64)
                        + num::abs(star_positions[j].1 as i64 - star_positions[i].1 as i64);

                for &col in &double_width_rows {
                    if (star_positions[i].0 < col && col < star_positions[j].0)
                        || (star_positions[j].0 < col && col < star_positions[i].0)
                    {
                        sum_of_distances += 999999;
                    }
                }
                for &col in &double_width_columns {
                    if (star_positions[i].1 < col && col < star_positions[j].1)
                        || (star_positions[j].1 < col && col < star_positions[i].1)
                    {
                        sum_of_distances += 999999;
                    }
                }
            }
        }

        return Ok(Box::new(sum_of_distances));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;
}
