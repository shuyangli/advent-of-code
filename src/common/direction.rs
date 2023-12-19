#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

const DIRECTIONS_BY_ORIENTATION: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

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

    pub fn rotate(&self, degrees: i32) -> Self {
        if degrees % 90 != 0 {
            panic!("We can only rotate 90 degrees at a time!");
        }
        let slice_idx = match &self {
            Self::North => 0,
            Self::East => 1,
            Self::South => 2,
            Self::West => 3,
        } + (degrees / 90);
        let slice_idx = (slice_idx % 4) as usize;
        DIRECTIONS_BY_ORIENTATION[slice_idx]
    }
}
