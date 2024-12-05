use crate::{
    common::{
        direction::Direction,
        grid::{self, Grid},
        position::Position,
    },
    day::Day,
};
use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

pub struct Day21 {}

fn get_starting_position(grid: &Grid<char>) -> Position {
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'S' {
                return Position(i, j);
            }
        }
    }
    panic!("Didn't find a starting position in the grid!");
}

fn count_reachable_grids(
    grid: &Grid<char>,
    starting_position: Position,
    polarity: i64,
    max_num_steps: i64,
) -> i64 {
    // Color the grid in a checkerboard pattern, we can floodfill from start
    // and count the number of grids we can reach that match the starting grid's color.
    let mut num_matching_grids = 0;

    let mut visited = HashSet::new();
    let mut next_positions = VecDeque::new();
    next_positions.push_back((starting_position, 0));

    while let Some((next_position, num_steps)) = next_positions.pop_front() {
        if visited.contains(&next_position) {
            continue;
        } else if grid[next_position.0][next_position.1] == '#' {
            continue;
        } else if num_steps > max_num_steps {
            continue;
        }
        visited.insert(next_position);

        if num_steps % 2 == polarity {
            num_matching_grids += 1;
        }

        for direction in [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            if let Some(neighbor) = next_position.step_within_grid(direction, &grid) {
                next_positions.push_back((neighbor, num_steps + 1));
            }
        }
    }
    return num_matching_grids;
}

fn count_reachable_grids_repeating_infinitely(
    grid: &Grid<char>,
    starting_position: Position,
    max_num_steps: i64,
) -> i64 {
    dbg!(max_num_steps);

    // Looking at the input, the grid is a square, and there's always a Manhattan way to get around the grid,
    // so we just need to calculate how many grids we can fully traverse and how far we can go along the edges.
    let grid_length = grid.len() as i64;
    let half_grid_length = starting_position.1 as i64;

    let mut num_total_cells = 0_i64;

    // Since the grid is 131 x 131, we need to consider both colors in the grid.
    let reachable_squares_with_same_coloring =
        count_reachable_grids(&grid, starting_position, 0, i64::MAX) as i64;
    dbg!(reachable_squares_with_same_coloring);
    let reachable_squares_with_opposite_coloring =
        count_reachable_grids(&grid, starting_position, 1, i64::MAX) as i64;
    dbg!(reachable_squares_with_opposite_coloring);

    // Along each horizontal and vertical axis, how many grids can we fully traverse?
    // 1-indexed
    println!("On axes");
    let n_grid_we_can_just_reach_on_axes = (max_num_steps - half_grid_length - 1) / grid_length + 1;
    dbg!(n_grid_we_can_just_reach_on_axes);
    let num_steps =
        (n_grid_we_can_just_reach_on_axes - 1) * grid_length + half_grid_length as i64 + 1;
    let num_steps_remaining = max_num_steps - num_steps;
    dbg!(num_steps_remaining);

    for position in [
        Position(starting_position.0, (grid_length - 1) as usize),
        Position(starting_position.0, 0),
        Position((grid_length - 1) as usize, starting_position.1),
        Position(0, starting_position.1),
    ] {
        // We can't fully traverse the grid at that position.
        let num_cells_in_end_grid = count_reachable_grids(
            grid,
            position,
            n_grid_we_can_just_reach_on_axes % 2,
            num_steps_remaining,
        );
        dbg!(num_cells_in_end_grid);

        // We also may not be able to fully traverse the grid just before it.
        let num_steps_remaining = num_steps_remaining + grid_length;
        let num_cells_in_grid_before_end = count_reachable_grids(
            grid,
            position,
            (n_grid_we_can_just_reach_on_axes + 1) % 2,
            num_steps_remaining,
        );
        dbg!(num_cells_in_grid_before_end);

        num_total_cells += num_cells_in_end_grid + num_cells_in_grid_before_end;

        // For the fully-traversed grids on that side, add up their cells.
        // Opposite coloring
        let num_fully_traversed_squares_with_opposite_coloring =
            (n_grid_we_can_just_reach_on_axes - 1) / 2;
        dbg!(num_fully_traversed_squares_with_opposite_coloring);
        num_total_cells += num_fully_traversed_squares_with_opposite_coloring
            * reachable_squares_with_opposite_coloring;
        // Same coloring
        let num_fully_traversed_squares_with_same_coloring =
            (n_grid_we_can_just_reach_on_axes - 2) / 2;
        dbg!(num_fully_traversed_squares_with_same_coloring);
        num_total_cells +=
            num_fully_traversed_squares_with_same_coloring * reachable_squares_with_same_coloring;
    }

    // On each diagonal, how many grids can we fully traverse?
    println!("On diagonals");
    let n_grids_we_can_just_reach_diagonally =
        (max_num_steps - 2 * half_grid_length - 2) / grid_length + 1;
    dbg!(n_grids_we_can_just_reach_diagonally);
    let num_steps =
        (n_grids_we_can_just_reach_diagonally - 1) * grid_length + half_grid_length as i64 * 2 + 2;
    let num_steps_remaining = max_num_steps - num_steps;
    dbg!(num_steps_remaining);

    for position in [
        Position(0, (grid_length - 1) as usize),
        Position(0, 0),
        Position((grid_length - 1) as usize, 0),
        Position((grid_length - 1) as usize, (grid_length - 1) as usize),
    ] {
        // We can't fully traverse the grid at that position.
        let num_cells_in_end_grid = count_reachable_grids(
            grid,
            position,
            (n_grids_we_can_just_reach_diagonally + 1) % 2,
            num_steps_remaining,
        );
        dbg!(num_cells_in_end_grid);

        // We also may not be able to fully traverse the grid just before it.
        let num_steps_remaining = num_steps_remaining + grid_length;
        let num_cells_in_grid_before_end = count_reachable_grids(
            grid,
            position,
            n_grid_we_can_just_reach_on_axes % 2,
            num_steps_remaining,
        );
        dbg!(num_cells_in_grid_before_end);

        num_total_cells += num_cells_in_end_grid * n_grids_we_can_just_reach_diagonally
            + num_cells_in_grid_before_end * (n_grids_we_can_just_reach_diagonally - 1);

        // For the fully-traversed grids on that diagonal, add up their cells.
        // Same coloring
        let num_fully_traversed_squares_with_same_coloring =
            (n_grid_we_can_just_reach_on_axes - 1) * (n_grid_we_can_just_reach_on_axes - 1) / 4;
        dbg!(num_fully_traversed_squares_with_same_coloring);
        num_total_cells +=
            num_fully_traversed_squares_with_same_coloring * reachable_squares_with_same_coloring;

        // Opposite coloring
        let num_fully_traversed_squares_with_opposite_coloring =
            (n_grid_we_can_just_reach_on_axes - 3) * (n_grid_we_can_just_reach_on_axes - 1) / 4;
        dbg!(num_fully_traversed_squares_with_opposite_coloring);
        num_total_cells += num_fully_traversed_squares_with_opposite_coloring
            * reachable_squares_with_opposite_coloring;
    }

    // We can fully traverse the center cell.
    num_total_cells += reachable_squares_with_same_coloring;

    return num_total_cells;
}

impl Day for Day21 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let grid = grid::parse_grid(input);
        let starting_position = get_starting_position(&grid);

        return Ok(Box::new(count_reachable_grids(
            &grid,
            starting_position,
            0,
            64,
        )));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let grid = grid::parse_grid(input);
        let starting_position = get_starting_position(&grid);

        dbg!(starting_position);

        return Ok(Box::new(count_reachable_grids_repeating_infinitely(
            &grid,
            starting_position,
            26501365,
        )));
    }
}
