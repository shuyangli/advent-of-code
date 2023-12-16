#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    pub fn reflect(&self, mirror: char) -> Self {
        match mirror {
            '\\' => match self {
                Self::North => Self::West,
                Self::West => Self::North,
                Self::South => Self::East,
                Self::East => Self::South,
            },
            '/' => match self {
                Self::North => Self::East,
                Self::East => Self::North,
                Self::South => Self::West,
                Self::West => Self::South,
            },
            c => panic!("Unexpected mirror character {c}"),
        }
    }
}
