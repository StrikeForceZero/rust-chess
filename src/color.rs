use std::fmt::{Display, Formatter};
use crate::direction::facing_direction::FacingDirection;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum Color {
    #[default]
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

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Color::White => "white",
            Color::Black => "black",
        };
        write!(f, "{str}")
    }
}
