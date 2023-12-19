use crate::common::direction::Direction;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Position(pub usize, pub usize);

impl Position {
    // Get the next position by taking a step in the given direction
    pub fn step(&self, direction: Direction) -> Option<Self> {
        self.step_by(direction, 1)
    }

    // Get the next position by taking a step in the given direction by the
    pub fn step_by(&self, direction: Direction, step_size: usize) -> Option<Self> {
        match direction {
            Direction::North => {
                if self.0 < step_size {
                    None
                } else {
                    Some(Self(self.0 - step_size, self.1))
                }
            }
            Direction::South => Some(Self(self.0 + step_size, self.1)),
            Direction::East => Some(Self(self.0, self.1 + step_size)),
            Direction::West => {
                if self.1 < step_size {
                    None
                } else {
                    Some(Self(self.0, self.1 - step_size))
                }
            }
        }
    }

    pub fn step_within_grid<T>(&self, direction: Direction, grid: &Vec<Vec<T>>) -> Option<Self> {
        self.step_by_within_grid(direction, 1, grid)
    }

    pub fn step_by_within_grid<T>(
        &self,
        direction: Direction,
        step_size: usize,
        grid: &Vec<Vec<T>>,
    ) -> Option<Self> {
        let next_position = self.step_by(direction, step_size)?;
        if next_position.is_in_bounds(grid) {
            Some(next_position)
        } else {
            None
        }
    }

    pub fn is_in_bounds<T>(&self, grid: &Vec<Vec<T>>) -> bool {
        if grid.len() == 0 {
            println!("Position::is_in_bounds: received empty grid!");
            return false;
        }
        self.0 < grid.len() && self.1 < grid[0].len()
    }
}
