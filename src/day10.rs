use crate::common::direction::Direction;
use crate::day::Day;
use core::panic;
use std::fmt::Display;

pub struct Day10 {}

#[derive(Debug)]
struct Position(i32, i32);

impl Position {
    // Get the next position by taking a step in the given direction
    fn step(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self(self.0 - 1, self.1),
            Direction::South => Self(self.0 + 1, self.1),
            Direction::East => Self(self.0, self.1 + 1),
            Direction::West => Self(self.0, self.1 - 1),
        }
    }

    fn is_valid_for<T>(&self, maze: &Vec<Vec<T>>) -> bool {
        self.0 < 0 || self.0 >= maze.len() as i32 || self.1 < 0 || self.1 >= maze[0].len() as i32
    }
}

struct Maze {
    maze: Vec<Vec<char>>,
    starting_position: Position,
    loop_markings: Vec<Vec<bool>>,
}

fn get_starting_position(maze: &Vec<Vec<char>>) -> Position {
    for (i, line) in maze.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'S' {
                return Position(i as i32, j as i32);
            }
        }
    }
    panic!("Maze does not have a starting position!");
}

impl Maze {
    fn parse_from_input(input: &str) -> Self {
        let maze: Vec<_> = input.lines().map(|line| line.chars().collect()).collect();
        let loop_markings: Vec<_> = maze
            .iter()
            .map(|l: &Vec<char>| l.iter().map(|_c| false).collect())
            .collect();
        let starting_position = get_starting_position(&maze);
        Self {
            maze,
            starting_position,
            loop_markings,
        }
    }

    fn mark_loop(&mut self, position: Position, from: Direction) -> Option<u32> {
        use Direction::*;

        if position.is_valid_for(&self.maze) {
            return None;
        }
        return match self.maze[position.0 as usize][position.1 as usize] {
            'S' => Some(0),
            '.' => None,
            '|' => match from {
                North => self.mark_loop(position.step(South), North),
                South => self.mark_loop(position.step(North), South),
                _ => None,
            },
            '-' => match from {
                West => self.mark_loop(position.step(East), West),
                East => self.mark_loop(position.step(West), East),
                _ => None,
            },
            'L' => match from {
                North => self.mark_loop(position.step(East), West),
                East => self.mark_loop(position.step(North), South),
                _ => None,
            },
            'J' => match from {
                North => self.mark_loop(position.step(West), East),
                West => self.mark_loop(position.step(North), South),
                _ => None,
            },
            '7' => match from {
                South => self.mark_loop(position.step(West), East),
                West => self.mark_loop(position.step(South), North),
                _ => None,
            },
            'F' => match from {
                South => self.mark_loop(position.step(East), West),
                East => self.mark_loop(position.step(South), North),
                _ => None,
            },
            _ => None,
        }
        .map(|v: u32| {
            self.loop_markings[position.0 as usize][position.1 as usize] = true;
            v + 1
        });
    }

    fn get_loop_size_and_mark_loop(&mut self) -> u32 {
        use Direction::*;

        if let Some(distance) = self.mark_loop(self.starting_position.step(North), South) {
            return distance;
        }
        if let Some(distance) = self.mark_loop(self.starting_position.step(South), North) {
            return distance;
        }
        if let Some(distance) = self.mark_loop(self.starting_position.step(East), West) {
            return distance;
        }
        if let Some(distance) = self.mark_loop(self.starting_position.step(West), East) {
            return distance;
        }

        panic!("No loop detected!");
    }

    fn count_empty_spaces_within_line<I>(&self, i: usize, indices: I) -> u32
    where
        I: Iterator<Item = usize>,
    {
        let mut is_in_loop = false;
        let mut polarity = 0;
        let mut num_empty_spaces = 0;
        for j in indices {
            if !self.loop_markings[i][j] {
                // If it's not part of the loop, consider accumulating.
                if is_in_loop {
                    num_empty_spaces += 1;
                }
                continue;
            }

            match self.maze[i][j] {
                'S' => panic!("Starting line has special processing"),
                '|' => is_in_loop = !is_in_loop,
                '-' => {
                    // Noop if we're going horizontal.
                }
                'L' => {
                    polarity -= 1;
                }
                'J' => {
                    polarity += 1;
                }
                'F' => {
                    polarity += 1;
                }
                '7' => {
                    polarity -= 1;
                }
                c => panic!("Unexpected loop character {}", c),
            }
            if polarity == 2 || polarity == -2 {
                polarity = 0;
                is_in_loop = !is_in_loop;
            }
        }

        num_empty_spaces
    }
}

impl Day for Day10 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let mut maze = Maze::parse_from_input(input);
        return Ok(Box::new(maze.get_loop_size_and_mark_loop() / 2));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let mut maze = Maze::parse_from_input(input);
        maze.get_loop_size_and_mark_loop();

        let mut num_spaces = 0;
        for (i, line) in maze.maze.iter().enumerate() {
            if i as i32 == maze.starting_position.0 {
                // Starting line, we will partition from 'S' and count from both ends
                num_spaces +=
                    maze.count_empty_spaces_within_line(i, 0..maze.starting_position.1 as usize);
                num_spaces += maze.count_empty_spaces_within_line(
                    i,
                    (maze.starting_position.1 as usize + 1..line.len()).rev(),
                )
            } else {
                num_spaces += maze.count_empty_spaces_within_line(i, 0..line.len())
            }
        }

        return Ok(Box::new(num_spaces));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;
}
