use crate::common::direction::Direction;
use crate::common::grid;
use crate::common::position::Position;
use crate::day::Day;
use std::fmt::Display;

pub struct Day14 {}

fn inplace_slide_rock_in_direction(
    grid: &mut Vec<Vec<char>>,
    position: Position,
    direction: Direction,
) {
    if !position.is_in_bounds(grid) {
        return;
    }
    if grid[position.0][position.1] != 'O' {
        return;
    }

    let mut previous_position = position.clone();
    while let Some(next_position) = previous_position.step(direction) {
        if !next_position.is_in_bounds(grid) {
            return;
        }
        match grid[next_position.0][next_position.1] {
            '#' | 'O' => {
                // Cannot slide anymore
                return;
            }
            '.' => {
                grid[next_position.0][next_position.1] = 'O';
                grid[previous_position.0][previous_position.1] = '.';
                previous_position = next_position;
            }
            c => {
                panic!("Unexpected grid input {c}");
            }
        }
    }
}

fn inplace_slide_grid_in_direction(grid: &mut Vec<Vec<char>>, direction: Direction) {
    match direction {
        Direction::North | Direction::West => {
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    inplace_slide_rock_in_direction(grid, Position(i, j), direction);
                }
            }
        }
        Direction::South => {
            for i in (0..grid.len()).rev() {
                for j in 0..grid[0].len() {
                    inplace_slide_rock_in_direction(grid, Position(i, j), direction);
                }
            }
        }
        Direction::East => {
            for i in 0..grid.len() {
                for j in (0..grid[0].len()).rev() {
                    inplace_slide_rock_in_direction(grid, Position(i, j), direction);
                }
            }
        }
    }
}

fn compute_load(grid: &Vec<Vec<char>>) -> i32 {
    let num_rows = grid.len();

    grid.iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|&&c| c == 'O').count() as i32 * (num_rows - i) as i32)
        .sum()
}

impl Day for Day14 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let mut grid = grid::parse_grid(input);

        inplace_slide_grid_in_direction(&mut grid, Direction::North);

        return Ok(Box::new(compute_load(&grid)));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let mut grid = grid::parse_grid(input);
        let mut past_grids = vec![grid.clone()];

        let mut cycle_begin: usize = 0;
        let mut cycle_end: usize = 0;

        // Original grid is at index 0, so we start at 1 for convenience
        // Grid after X iterations = with index X, find the corresponding index in past_grids
        for j in 1_usize..=1000000000_usize {
            for direction in [
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East,
            ] {
                inplace_slide_grid_in_direction(&mut grid, direction);
            }

            if let Some((i, _)) = past_grids
                .iter()
                .enumerate()
                .rev()
                .find(|&past_grid| past_grid.1 == &grid)
            {
                cycle_begin = i;
                cycle_end = j;
                break;
            }

            past_grids.push(grid.clone())
        }

        let grid_idx_at_cycle =
            cycle_begin + (1000000000_usize - cycle_begin) % (cycle_end - cycle_begin);

        return Ok(Box::new(compute_load(&past_grids[grid_idx_at_cycle])));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;
}
