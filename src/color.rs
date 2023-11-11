use crate::facing_direction::FacingDirection;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub const fn as_inverse(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
    pub const fn as_facing_direction(&self) -> FacingDirection {
        match self {
            Self::White => FacingDirection::North,
            Self::Black => FacingDirection::South,
        }
    }

    pub const fn as_score(&self) -> i32 {
        match self {
            Color::White => 1,
            Color::Black => -1,
        }
    }
}
