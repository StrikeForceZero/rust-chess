use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
pub enum BoardRankError {
    #[error("Invalid number for BoardRank: {0}")]
    InvalidNumber(usize),
    #[error("Invalid char for BoardRank: {0}")]
    InvalidChar(char),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum BoardRank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl BoardRank {
    pub const fn as_usize(&self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
        }
    }
    pub const fn from_char(char: char) -> Result<Self, BoardRankError> {
        Ok(match char {
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            _ => return Err(BoardRankError::InvalidChar(char))
        })
    }
    pub const fn as_char(&self) -> char {
        match self {
            Self::One => '1',
            Self::Two => '2',
            Self::Three => '3',
            Self::Four => '4',
            Self::Five => '5',
            Self::Six => '6',
            Self::Seven => '7',
            Self::Eight => '8',
        }
    }
    pub const fn as_zero_based_index(&self) -> usize {
        match self {
            Self::One => 0,
            Self::Two => 1,
            Self::Three => 2,
            Self::Four => 3,
            Self::Five => 4,
            Self::Six => 5,
            Self::Seven => 6,
            Self::Eight => 7,
        }
    }
    pub const fn as_shift_offset(&self) -> usize {
        self.as_zero_based_index() * 8
    }
    const fn from_usize(n: usize) -> Result<Self, BoardRankError> {
        Ok(match n {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            _ => return Err(BoardRankError::InvalidNumber(n))
        })
    }
    pub const fn from_zero_based_index(n: usize) -> Result<Self, BoardRankError> {
        Ok(match n {
            0 => Self::One,
            1 => Self::Two,
            2 => Self::Three,
            3 => Self::Four,
            4 => Self::Five,
            5 => Self::Six,
            6 => Self::Seven,
            7 => Self::Eight,
            _ => return Err(BoardRankError::InvalidNumber(n))
        })
    }
    pub const fn next(&self) -> Option<Self> {
        Some(match self {
            Self::One => Self::Two,
            Self::Two => Self::Three,
            Self::Three => Self::Four,
            Self::Four => Self::Five,
            Self::Five => Self::Six,
            Self::Six => Self::Seven,
            Self::Seven => Self::Eight,
            Self::Eight => return None,
        })
    }

    pub const fn prev(&self) -> Option<Self> {
        Some(match self {
            Self::One => return None,
            Self::Two => Self::One,
            Self::Three => Self::Two,
            Self::Four => Self::Three,
            Self::Five => Self::Four,
            Self::Six => Self::Five,
            Self::Seven => Self::Six,
            Self::Eight => Self::Seven,
        })
    }
}

impl Display for BoardRank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_usize())
    }
}

pub const WHITE_BACK_RANK: BoardRank = BoardRank::One;
pub const WHITE_PAWNN_RANK: BoardRank = BoardRank::Two;
pub const BLACK_PAWNN_RANK: BoardRank = BoardRank::Seven;
pub const BLACK_BACK_RANK: BoardRank = BoardRank::Eight;
