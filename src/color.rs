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
    pub const fn as_score(&self, maximizing_color: Color) -> i32 {
        match (self, maximizing_color) {
            (Self::White, Self::White) | (Self::Black, Self::Black) => 1,
            _ => -1,
        }
    }
}
