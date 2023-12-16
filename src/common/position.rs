use crate::common::direction::Direction;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Position(pub usize, pub usize);

impl Position {
    // Get the next position by taking a step in the given direction
    pub fn step(&self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::North => {
                if self.0 == 0 {
                    None
                } else {
                    Some(Self(self.0 - 1, self.1))
                }
            }
            Direction::South => Some(Self(self.0 + 1, self.1)),
            Direction::East => Some(Self(self.0, self.1 + 1)),
            Direction::West => {
                if self.1 == 0 {
                    None
                } else {
                    Some(Self(self.0, self.1 - 1))
                }
            }
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
