use itertools::Itertools;

use crate::common::direction::Direction;
use crate::common::grid::{self, Grid};
use crate::common::position::Position;
use crate::day::Day;
use std::collections::HashSet;
use std::fmt::Display;

pub struct Day16 {}

fn step(
    next_positions: &mut Vec<(Position, Direction)>,
    position: &Position,
    heading: Direction,
    grid: &Grid<char>,
) {
    if let Some(next_position) = position.step_within_grid(heading, &grid) {
        next_positions.push((next_position, heading))
    }
}

fn count_energized_tiles(grid: &Grid<char>, starting: (Position, Direction)) -> usize {
    let mut visited: HashSet<_> = HashSet::<(Position, Direction)>::new();
    let mut next_positions = vec![starting];

    while let Some((position, heading)) = next_positions.pop() {
        if visited.contains(&(position, heading)) {
            continue;
        }

        visited.insert((position.clone(), heading));

        match grid[position.0][position.1] {
            '.' => step(&mut next_positions, &position, heading, &grid),
            '/' | '\\' => step(
                &mut next_positions,
                &position,
                heading.reflect(grid[position.0][position.1]),
                &grid,
            ),
            '|' => match heading {
                Direction::North | Direction::South => {
                    step(&mut next_positions, &position, heading, &grid)
                }
                Direction::East | Direction::West => {
                    step(&mut next_positions, &position, Direction::North, &grid);
                    step(&mut next_positions, &position, Direction::South, &grid);
                }
            },
            '-' => match heading {
                Direction::East | Direction::West => {
                    step(&mut next_positions, &position, heading, &grid)
                }
                Direction::North | Direction::South => {
                    step(&mut next_positions, &position, Direction::East, &grid);
                    step(&mut next_positions, &position, Direction::West, &grid);
                }
            },
            c => panic!("Unexpected character {c} in grid"),
        }
    }

    return visited.iter().map(|pair| pair.0).unique().count();
}

impl Day for Day16 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let grid = grid::parse_grid(input);
        return Ok(Box::new(count_energized_tiles(
            &grid,
            (Position(0, 0), Direction::East),
        )));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let grid = grid::parse_grid(input);

        let num_rows = grid.len();
        let num_columns = grid[0].len();

        let max_energized_tiles = (0..num_rows)
            .map(|i| (Position(i, 0), Direction::East))
            .chain((0..num_rows).map(|i| (Position(i, num_columns - 1), Direction::West)))
            .chain((0..num_columns).map(|j| (Position(0, j), Direction::South)))
            .chain((0..num_columns).map(|j| (Position(num_rows - 1, j), Direction::North)))
            .map(|starting| count_energized_tiles(&grid, starting))
            .max()
            .unwrap();

        return Ok(Box::new(max_energized_tiles));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;
}
