use crate::day::Day;
use std::fmt::Display;

pub struct Day13 {}

#[derive(Debug)]
struct Pattern {
    grid: Vec<Vec<char>>,
}

fn transpose_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..grid[0].len())
        .map(|col_idx| grid.iter().map(|row| row[col_idx]).collect())
        .collect()
}

fn get_reflected_row(grid: &Vec<Vec<char>>) -> Option<usize> {
    let num_rows = grid.len();
    'row_loop: for row_idx in 0..(num_rows - 1) {
        for (i, j) in (0..=row_idx).rev().zip((row_idx + 1)..num_rows) {
            if grid[i] != grid[j] {
                continue 'row_loop;
            }
        }
        return Some(row_idx);
    }
    return None;
}

fn are_rows_equal_allowing_smudge(a: &Vec<char>, b: &Vec<char>, allow_smudge: &mut bool) -> bool {
    if a == b {
        return true;
    }

    if *allow_smudge
        && a.iter()
            .zip(b.iter())
            .map(|(a, b)| if a == b { 0 } else { 1 })
            .sum::<i32>()
            == 1
    {
        *allow_smudge = false;
        return true;
    }

    return false;
}

fn get_reflected_row_with_smudge(grid: &Vec<Vec<char>>) -> Option<usize> {
    let num_rows = grid.len();
    'row_loop: for row_idx in 0..(num_rows - 1) {
        let mut allow_smudge = true;
        for (i, j) in (0..=row_idx).rev().zip((row_idx + 1)..num_rows) {
            if !are_rows_equal_allowing_smudge(&grid[i], &grid[j], &mut allow_smudge) {
                continue 'row_loop;
            }
        }
        if allow_smudge {
            // We need to use up the smudge allowance, since there's exactly 1 smudge.
            continue 'row_loop;
        }
        return Some(row_idx);
    }
    return None;
}

impl Pattern {
    fn new() -> Self {
        Self { grid: vec![] }
    }

    // Returns the row index just above of the reflecting line.
    fn get_reflected_row(&self) -> Option<usize> {
        get_reflected_row(&self.grid)
    }

    // Returns the column index just left of the reflecting line.
    fn get_reflected_column(&self) -> Option<usize> {
        let transposed = transpose_grid(&self.grid);
        get_reflected_row(&transposed)
    }

    // Returns the row index just above of the reflecting line, reflecting the smudge.
    fn get_reflected_row_part2(&self) -> Option<usize> {
        get_reflected_row_with_smudge(&self.grid)
    }

    // Returns the column index just left of the reflecting line, reflecting the smudge.
    fn get_reflected_column_part2(&self) -> Option<usize> {
        let transposed = transpose_grid(&self.grid);
        get_reflected_row_with_smudge(&transposed)
    }
}

fn parse_patterns(input: &str) -> Vec<Pattern> {
    let mut patterns = vec![];
    let mut lines = input.lines();

    let mut pattern = Pattern::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            patterns.push(pattern);
            pattern = Pattern::new();
        } else {
            pattern.grid.push(line.chars().collect());
        }
    }
    patterns.push(pattern);

    return patterns;
}

impl Day for Day13 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let patterns = parse_patterns(input);

        let results: usize = patterns
            .iter()
            .map(|p| {
                if let Some(row) = p.get_reflected_row() {
                    100 * (row + 1)
                } else if let Some(column) = p.get_reflected_column() {
                    column + 1
                } else {
                    panic!("Did not find any reflection in the pattern!");
                }
            })
            .sum();
        return Ok(Box::new(results));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let patterns = parse_patterns(input);

        let results: usize = patterns
            .iter()
            .map(|p| {
                if let Some(row) = p.get_reflected_row_part2() {
                    100 * (row + 1)
                } else if let Some(column) = p.get_reflected_column_part2() {
                    column + 1
                } else {
                    panic!("Did not find any reflection in the pattern!");
                }
            })
            .sum();
        return Ok(Box::new(results));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[googletest::test]
    fn failing_case() {
        let input = r"##.##.###
##.##.###
..####..#
###.#.###
###.#..#.
#.###....
###.#...#
###.#...#
#.###....
###.#..#.
###.#.###
..##.#..#
##.##.###";
        let patterns = parse_patterns(input);

        expect_that!(patterns[0].get_reflected_row_part2(), eq(Some(6)));
    }
}
