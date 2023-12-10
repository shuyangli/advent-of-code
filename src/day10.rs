use crate::day::Day;
use std::fmt::Display;

pub struct Day10 {}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

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
}

struct Maze {
    maze: Vec<Vec<char>>,
}

impl Maze {
    fn parse_from_input(input: &str) -> Self {
        Self {
            maze: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    fn get_starting_position(&self) -> Position {
        for (i, line) in self.maze.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c == 'S' {
                    return Position(i as i32, j as i32);
                }
            }
        }
        panic!("Maze does not have a starting position!");
    }

    fn distance_from_starting_position(&self, position: Position, from: Direction) -> Option<u32> {
        use Direction::*;

        if position.0 < 0
            || position.0 >= self.maze.len() as i32
            || position.1 < 0
            || position.1 >= self.maze[0].len() as i32
        {
            return None;
        }
        return match self.maze[position.0 as usize][position.1 as usize] {
            'S' => Some(0),
            '.' => None,
            '|' => match from {
                North => self.distance_from_starting_position(position.step(South), North),
                South => self.distance_from_starting_position(position.step(North), South),
                _ => None,
            },
            '-' => match from {
                West => self.distance_from_starting_position(position.step(East), West),
                East => self.distance_from_starting_position(position.step(West), East),
                _ => None,
            },
            'L' => match from {
                North => self.distance_from_starting_position(position.step(East), West),
                East => self.distance_from_starting_position(position.step(North), South),
                _ => None,
            },
            'J' => match from {
                North => self.distance_from_starting_position(position.step(West), East),
                West => self.distance_from_starting_position(position.step(North), South),
                _ => None,
            },
            '7' => match from {
                South => self.distance_from_starting_position(position.step(West), East),
                West => self.distance_from_starting_position(position.step(South), North),
                _ => None,
            },
            'F' => match from {
                South => self.distance_from_starting_position(position.step(East), West),
                East => self.distance_from_starting_position(position.step(South), North),
                _ => None,
            },
            _ => None,
        }
        .map(|v: u32| v + 1);
    }

    fn get_loop_size(&self) -> u32 {
        use Direction::*;

        let starting_position = self.get_starting_position();

        if let Some(distance) =
            self.distance_from_starting_position(starting_position.step(North), South)
        {
            return distance;
        }
        if let Some(distance) =
            self.distance_from_starting_position(starting_position.step(South), North)
        {
            return distance;
        }
        if let Some(distance) =
            self.distance_from_starting_position(starting_position.step(East), West)
        {
            return distance;
        }
        if let Some(distance) =
            self.distance_from_starting_position(starting_position.step(West), East)
        {
            return distance;
        }

        panic!("No loop detected!");
    }
}

impl Day for Day10 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let maze = Maze::parse_from_input(input);

        return Ok(Box::new(maze.get_loop_size() / 2));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;
}
