use crate::direction::{DiagonalDirection, Direction, SimpleDirection};

#[derive(Copy, Clone, Debug, PartialEq)]
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

impl From<FacingDirection> for Direction {
    fn from(value: FacingDirection) -> Self {
        value.as_simple_direction().as_direction()
    }
}

impl From<FacingDirection> for SimpleDirection {
    fn from(value: FacingDirection) -> Self {
        value.as_simple_direction()
    }
}
