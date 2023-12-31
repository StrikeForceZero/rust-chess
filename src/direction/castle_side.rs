use crate::board::board_file::BoardFile;
use crate::board::board_position::BoardPosition;
use crate::direction::direction::{Direction, SimpleDirection};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum CastleSideConversionError {
    #[error("Invalid direction for CastleSide, expected: East | West, received: {0:?}")]
    InvalidCastleSideDirection(Direction),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd)]
pub enum CastleSide {
    King,
    Queen,
}

impl CastleSide {
    pub const fn try_from_direction(
        direction: Direction,
    ) -> Result<Self, CastleSideConversionError> {
        Ok(match direction {
            Direction::East => Self::King,
            Direction::West => Self::Queen,
            _ => return Err(CastleSideConversionError::InvalidCastleSideDirection(direction)),
        })
    }
    pub const fn as_simple_direction(&self) -> SimpleDirection {
        match self {
            Self::King => SimpleDirection::East,
            Self::Queen => SimpleDirection::West,
        }
    }
    pub const fn from_pos(pos: BoardPosition) -> Self {
        let BoardPosition(file, _rank) = pos;
        match file {
            BoardFile::A => Self::Queen,
            BoardFile::B => Self::Queen,
            BoardFile::C => Self::Queen,
            BoardFile::D => Self::Queen,
            BoardFile::E => Self::King,
            BoardFile::F => Self::King,
            BoardFile::G => Self::King,
            BoardFile::H => Self::King,
        }
    }
}
