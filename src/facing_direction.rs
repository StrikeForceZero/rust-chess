use crate::direction::{DiagonalDirection, SimpleDirection};

pub enum FacingDirection {
    North,
    South,
}

impl FacingDirection {
    pub const fn as_simple_direction(&self) -> SimpleDirection {
        match self {
            Self::North => SimpleDirection::North,
            Self::South => SimpleDirection::South,
        }
    }
    pub const fn split(&self) -> (DiagonalDirection, DiagonalDirection) {
        match self {
            FacingDirection::North => (DiagonalDirection::NorthWest, DiagonalDirection::NorthEast),
            FacingDirection::South => (DiagonalDirection::SouthWest, DiagonalDirection::SouthEast),
        }
    }
}
