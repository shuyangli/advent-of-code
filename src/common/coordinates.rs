use crate::common::direction::Direction;
use num::Integer;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Coordinates<T: Integer>(pub T, pub T);

impl<T> Coordinates<T>
where
    T: Integer + Copy,
{
    pub fn step(&self, direction: Direction, step_size: T) -> Self {
        match direction {
            Direction::North => Self(self.0 - step_size, self.1),
            Direction::South => Self(self.0 + step_size, self.1),
            Direction::East => Self(self.0, self.1 + step_size),
            Direction::West => Self(self.0, self.1 - step_size),
        }
    }
}
