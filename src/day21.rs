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
        println!("Starting position: {:?}", starting_position);

        let grid_height = grid.len();
        let grid_width = grid[0].len();

        // Since the grid is 131 x 131, we need to consider both colors in the grid.
        let reachable_squares_with_same_coloring =
            count_reachable_grids(&grid, starting_position, 0, i64::MAX) as i64;
        let reachable_squares_with_opposite_coloring =
            count_reachable_grids(&grid, starting_position, 1, i64::MAX) as i64;

        // Looking at the input, there's always a Manhattan way to get around the grid,
        // so we just need to calculate how far we can go on the edges.
        const MAX_STEPS: i64 = 50;

        // How many times can we walk directly in each diagonal?
        let num_lengths_per_diagonal =
            (MAX_STEPS - starting_position.0 as i64 - starting_position.1 as i64)
                / grid_height as i64
                - 1;

        // Count full cells in all 4 diagonals.
        let mut num_total_cells = 0_i64;
        let num_squares_with_same_coloring_per_diagonal = if num_lengths_per_diagonal % 2 == 0 {
            num_lengths_per_diagonal.pow(2) / 4
        } else {
            (num_lengths_per_diagonal + 1).pow(2) / 4
        };
        num_total_cells +=
            num_squares_with_same_coloring_per_diagonal * reachable_squares_with_same_coloring * 4;

        let num_squares_with_opposite_coloring_per_diagonal = if num_lengths_per_diagonal % 2 == 0 {
            (num_lengths_per_diagonal + 2) * num_lengths_per_diagonal / 4
        } else {
            (num_lengths_per_diagonal + 1) * (num_lengths_per_diagonal - 1) / 4
        };
        num_total_cells += num_squares_with_opposite_coloring_per_diagonal
            * reachable_squares_with_opposite_coloring
            * 4;

        // Count partial cells in 4 diagonals.
        let num_nearer_partial_squares_per_diagonal = num_lengths_per_diagonal + 1;
        let num_farther_partial_squares_per_diagonal = num_lengths_per_diagonal + 2;
        let num_remaining_steps_for_farther_partial_diagonal_cell =
            (MAX_STEPS - starting_position.0 as i64 - starting_position.1 as i64)
                % grid_width as i64;
        let num_remaining_steps_for_nearer_partial_diagonal_cell =
            (MAX_STEPS - starting_position.0 as i64 - starting_position.1 as i64)
                % grid_width as i64
                + grid_width as i64;

        if num_lengths_per_diagonal % 2 == 0 {
            // Same coloring is nearer, opposite coloring is farther. We need to count 4 corners.
            num_total_cells += num_nearer_partial_squares_per_diagonal
                * count_reachable_grids(
                    &grid,
                    Position(0, grid_height - 1),
                    0,
                    num_remaining_steps_for_nearer_partial_diagonal_cell,
                ) as i64;
            num_total_cells += num_farther_partial_squares_per_diagonal
                * count_reachable_grids(
                    &grid,
                    Position(0, grid_height - 1),
                    1,
                    num_remaining_steps_for_farther_partial_diagonal_cell,
                ) as i64;
            num_total_cells += num_nearer_partial_squares_per_diagonal
                * count_reachable_grids(
                    &grid,
                    Position(0, 0),
                    0,
                    num_remaining_steps_for_nearer_partial_diagonal_cell,
                ) as i64;
            num_total_cells += num_farther_partial_squares_per_diagonal
                * count_reachable_grids(
                    &grid,
                    Position(0, 0),
                    1,
                    num_remaining_steps_for_farther_partial_diagonal_cell,
                ) as i64;
            num_total_cells += num_nearer_partial_squares_per_diagonal
                * count_reachable_grids(
                    &grid,
                    Position(grid_width - 1, grid_height - 1),
                    0,
                    num_remaining_steps_for_nearer_partial_diagonal_cell,
                ) as i64;
            num_total_cells += num_farther_partial_squares_per_diagonal
                * count_reachable_grids(
                    &grid,
                    Position(grid_width - 1, grid_height - 1),
                    1,
                    num_remaining_steps_for_farther_partial_diagonal_cell,
                ) as i64;
            num_total_cells += num_nearer_partial_squares_per_diagonal
                * count_reachable_grids(
                    &grid,
                    Position(grid_width - 1, 0),
                    0,
                    num_remaining_steps_for_nearer_partial_diagonal_cell,
                ) as i64;
            num_total_cells += num_farther_partial_squares_per_diagonal
                * count_reachable_grids(
                    &grid,
                    Position(grid_width - 1, 0),
                    1,
                    num_remaining_steps_for_farther_partial_diagonal_cell,
                ) as i64;
        } else {
            // Same coloring is nearer, opposite coloring is farther. We need to count 4 corners.
            num_total_cells += num_nearer_partial_squares_per_diagonal
                * count_reachable_grids(
                    &grid,
                    Position(0, grid_height - 1),
                    1,
                    num_remaining_steps_for_nearer_partial_diagonal_cell,
                ) as i64;
            num_total_cells += num_farther_partial_squares_per_diagonal
                * count_reachable_grids(
                    &grid,
                    Position(0, grid_height - 1),
                    0,
                    num_remaining_steps_for_farther_partial_diagonal_cell,
                ) as i64;
            num_total_cells += num_nearer_partial_squares_per_diagonal
                * count_reachable_grids(
                    &grid,
                    Position(0, 0),
                    1,
                    num_remaining_steps_for_nearer_partial_diagonal_cell,
                ) as i64;
            num_total_cells += num_farther_partial_squares_per_diagonal
                * count_reachable_grids(
                    &grid,
                    Position(0, 0),
                    0,
                    num_remaining_steps_for_farther_partial_diagonal_cell,
                ) as i64;
            num_total_cells += num_nearer_partial_squares_per_diagonal
                * count_reachable_grids(
                    &grid,
                    Position(grid_width - 1, grid_height - 1),
                    1,
                    num_remaining_steps_for_nearer_partial_diagonal_cell,
                ) as i64;
            num_total_cells += num_farther_partial_squares_per_diagonal
                * count_reachable_grids(
                    &grid,
                    Position(grid_width - 1, grid_height - 1),
                    0,
                    num_remaining_steps_for_farther_partial_diagonal_cell,
                ) as i64;
            num_total_cells += num_nearer_partial_squares_per_diagonal
                * count_reachable_grids(
                    &grid,
                    Position(grid_width - 1, 0),
                    1,
                    num_remaining_steps_for_nearer_partial_diagonal_cell,
                ) as i64;
            num_total_cells += num_farther_partial_squares_per_diagonal
                * count_reachable_grids(
                    &grid,
                    Position(grid_width - 1, 0),
                    0,
                    num_remaining_steps_for_farther_partial_diagonal_cell,
                ) as i64;
        };

        // How many times can we walk directly in each straight line?
        let num_lengths_per_straight_line = num_lengths_per_diagonal + 1;
        num_total_cells += num_lengths_per_straight_line / 2 * reachable_squares_with_same_coloring;
        num_total_cells +=
            (num_lengths_per_straight_line + 1) / 2 * reachable_squares_with_opposite_coloring;

        let num_remaining_steps_for_farther_straight_cell =
            (MAX_STEPS - starting_position.0 as i64) % grid_width as i64;
        let num_remaining_steps_for_nearer_straight_cell =
            (MAX_STEPS - starting_position.0 as i64) % grid_width as i64 + grid_width as i64;

        if num_lengths_per_straight_line % 2 == 0 {
            // Opposite is nearer, Same is farther
            num_total_cells += count_reachable_grids(
                &grid,
                Position(starting_position.0, grid_height - 1),
                0,
                num_remaining_steps_for_nearer_straight_cell,
            ) as i64;
            num_total_cells += count_reachable_grids(
                &grid,
                Position(starting_position.0, grid_height - 1),
                1,
                num_remaining_steps_for_farther_straight_cell,
            ) as i64;
            num_total_cells += count_reachable_grids(
                &grid,
                Position(starting_position.0, 0),
                0,
                num_remaining_steps_for_nearer_straight_cell,
            ) as i64;
            num_total_cells += count_reachable_grids(
                &grid,
                Position(starting_position.0, 0),
                1,
                num_remaining_steps_for_farther_straight_cell,
            ) as i64;
            num_total_cells += count_reachable_grids(
                &grid,
                Position(grid_width - 1, starting_position.1),
                0,
                num_remaining_steps_for_nearer_straight_cell,
            ) as i64;
            num_total_cells += count_reachable_grids(
                &grid,
                Position(grid_width - 1, starting_position.1),
                1,
                num_remaining_steps_for_farther_straight_cell,
            ) as i64;
            num_total_cells += count_reachable_grids(
                &grid,
                Position(0, starting_position.1),
                0,
                num_remaining_steps_for_nearer_straight_cell,
            ) as i64;
            num_total_cells += count_reachable_grids(
                &grid,
                Position(0, starting_position.1),
                1,
                num_remaining_steps_for_farther_straight_cell,
            ) as i64;
        } else {
            // Opposite is farther, Same is nearer
            num_total_cells += count_reachable_grids(
                &grid,
                Position(starting_position.0, grid_height - 1),
                1,
                num_remaining_steps_for_nearer_straight_cell,
            ) as i64;
            num_total_cells += count_reachable_grids(
                &grid,
                Position(starting_position.0, grid_height - 1),
                0,
                num_remaining_steps_for_farther_straight_cell,
            ) as i64;
            num_total_cells += count_reachable_grids(
                &grid,
                Position(starting_position.0, 0),
                1,
                num_remaining_steps_for_nearer_straight_cell,
            ) as i64;
            num_total_cells += count_reachable_grids(
                &grid,
                Position(starting_position.0, 0),
                0,
                num_remaining_steps_for_farther_straight_cell,
            ) as i64;
            num_total_cells += count_reachable_grids(
                &grid,
                Position(grid_width - 1, starting_position.1),
                1,
                num_remaining_steps_for_nearer_straight_cell,
            ) as i64;
            num_total_cells += count_reachable_grids(
                &grid,
                Position(grid_width - 1, starting_position.1),
                0,
                num_remaining_steps_for_farther_straight_cell,
            ) as i64;
            num_total_cells += count_reachable_grids(
                &grid,
                Position(0, starting_position.1),
                1,
                num_remaining_steps_for_nearer_straight_cell,
            ) as i64;
            num_total_cells += count_reachable_grids(
                &grid,
                Position(0, starting_position.1),
                0,
                num_remaining_steps_for_farther_straight_cell,
            ) as i64;
        }

        // Finally, count the starting square itself.
        num_total_cells += count_reachable_grids(&grid, starting_position, 0, 64);

        return Ok(Box::new(num_total_cells));
    }
}
