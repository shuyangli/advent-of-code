use priority_queue::PriorityQueue;

use crate::common::direction::Direction;
use crate::common::grid::{parse_transform_grid, Grid};
use crate::common::position::Position;
use crate::day::Day;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::fmt::Display;

pub struct Day17 {}

#[derive(PartialEq, Eq, Hash, Debug)]
struct GridState {
    position: Position,
    direction: Direction,
    minimum_steps_before_turning: u32,
    remaining_steps_in_direction: u32,
    total_heat_loss: u32,
}

fn dijkstra_grid_for_minimum_heat_loss_part1(grid: &Grid<u32>) -> u32 {
    let mut visited_positions: HashMap<Position, u32> = HashMap::new();
    let mut next_positions: PriorityQueue<GridState, Reverse<u32>> = PriorityQueue::new();
    next_positions.push(
        GridState {
            position: Position(0, 0),
            direction: Direction::East,
            minimum_steps_before_turning: 0,
            remaining_steps_in_direction: 3,
            total_heat_loss: 0,
        },
        Reverse(0),
    );
    next_positions.push(
        GridState {
            position: Position(0, 0),
            direction: Direction::South,
            minimum_steps_before_turning: 0,
            remaining_steps_in_direction: 3,
            total_heat_loss: 0,
        },
        Reverse(0),
    );

    while let Some((grid_state, _)) = next_positions.pop() {
        println!("{:?}", grid_state);

        // How to more effectively shrink the search space?
        if visited_positions.contains_key(&grid_state.position)
            && grid_state.total_heat_loss - visited_positions[&grid_state.position] >= 10
        {
            continue;
        }

        if grid_state.position.0 == grid.len() - 1 && grid_state.position.1 == grid[0].len() - 1 {
            return grid_state.total_heat_loss;
        }

        for next_direction in [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            if next_direction == grid_state.direction.opposite() {
                continue;
            } else if grid_state.direction == next_direction
                && grid_state.remaining_steps_in_direction == 0
            {
                continue;
            } else if let Some(next_position) =
                grid_state.position.step_within_grid(next_direction, grid)
            {
                let next_heat_loss =
                    grid_state.total_heat_loss + grid[next_position.0][next_position.1];
                next_positions.push(
                    GridState {
                        position: next_position,
                        direction: next_direction,
                        minimum_steps_before_turning: 0,
                        remaining_steps_in_direction: if grid_state.direction == next_direction {
                            grid_state.remaining_steps_in_direction - 1
                        } else {
                            2
                        },
                        total_heat_loss: next_heat_loss,
                    },
                    Reverse(next_heat_loss),
                );
            }
        }

        visited_positions
            .entry(grid_state.position)
            .or_insert(grid_state.total_heat_loss);
    }

    panic!("Didn't reach the end somehow");
}

fn dijkstra_grid_for_minimum_heat_loss_part2(grid: &Grid<u32>) -> u32 {
    let mut visited_positions: HashMap<Position, u32> = HashMap::new();
    let mut next_positions: PriorityQueue<GridState, Reverse<u32>> = PriorityQueue::new();
    next_positions.push(
        GridState {
            position: Position(0, 0),
            direction: Direction::East,
            minimum_steps_before_turning: 4,
            remaining_steps_in_direction: 10,
            total_heat_loss: 0,
        },
        Reverse(0),
    );
    next_positions.push(
        GridState {
            position: Position(0, 0),
            direction: Direction::South,
            minimum_steps_before_turning: 4,
            remaining_steps_in_direction: 10,
            total_heat_loss: 0,
        },
        Reverse(0),
    );

    while let Some((grid_state, _)) = next_positions.pop() {
        println!("{:?}", grid_state);

        if grid_state.position.0 == grid.len() - 1 && grid_state.position.1 == grid[0].len() - 1 {
            return grid_state.total_heat_loss;
        }

        // How to more effectively shrink the search space?
        if visited_positions.contains_key(&grid_state.position)
            && grid_state.total_heat_loss - visited_positions[&grid_state.position] >= 40
        {
            continue;
        }

        if grid_state.minimum_steps_before_turning > 0 {
            if let Some(next_position) = grid_state
                .position
                .step_within_grid(grid_state.direction, grid)
            {
                let next_heat_loss =
                    grid_state.total_heat_loss + grid[next_position.0][next_position.1];
                next_positions.push(
                    GridState {
                        position: next_position,
                        direction: grid_state.direction,
                        minimum_steps_before_turning: grid_state.minimum_steps_before_turning - 1,
                        remaining_steps_in_direction: grid_state.remaining_steps_in_direction - 1,
                        total_heat_loss: next_heat_loss,
                    },
                    Reverse(next_heat_loss),
                );
            }
            continue;
        }

        for next_direction in [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            if next_direction == grid_state.direction.opposite() {
                continue;
            } else if grid_state.direction == next_direction
                && grid_state.remaining_steps_in_direction == 0
            {
                continue;
            } else if let Some(next_position) =
                grid_state.position.step_within_grid(next_direction, grid)
            {
                let next_heat_loss =
                    grid_state.total_heat_loss + grid[next_position.0][next_position.1];
                next_positions.push(
                    GridState {
                        position: next_position,
                        direction: next_direction,
                        minimum_steps_before_turning: if grid_state.direction == next_direction {
                            0
                        } else {
                            3
                        },
                        remaining_steps_in_direction: if grid_state.direction == next_direction {
                            grid_state.remaining_steps_in_direction - 1
                        } else {
                            9
                        },
                        total_heat_loss: next_heat_loss,
                    },
                    Reverse(next_heat_loss),
                );
            }
        }

        visited_positions
            .entry(grid_state.position)
            .or_insert(grid_state.total_heat_loss);
    }

    panic!("Didn't reach the end somehow");
}

impl Day for Day17 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let grid = parse_transform_grid(input, |c: char| c.to_digit(10).unwrap());

        return Ok(Box::new(dijkstra_grid_for_minimum_heat_loss_part1(&grid)));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let grid = parse_transform_grid(input, |c: char| c.to_digit(10).unwrap());

        return Ok(Box::new(dijkstra_grid_for_minimum_heat_loss_part2(&grid)));
    }
}
