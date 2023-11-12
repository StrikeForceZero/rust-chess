use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
pub enum BoardFileError {
    #[error("Invalid character for BoardFile: {0}")]
    InvalidChar(char),
    #[error("Invalid usize for BoardFile: {0}")]
    InvalidUsize(usize),
    #[error("Invalid u8 for BoardFile: {0}")]
    InvalidU8(u8),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum BoardFile {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl BoardFile {
    pub const fn as_char(&self) -> char {
        match self {
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D',
            Self::E => 'E',
            Self::F => 'F',
            Self::G => 'G',
            Self::H => 'H',
        }
    }
    pub const fn as_usize(&self) -> usize {
        match self {
            Self::A => 1,
            Self::B => 2,
            Self::C => 3,
            Self::D => 4,
            Self::E => 5,
            Self::F => 6,
            Self::G => 7,
            Self::H => 8,
        }
    }
    pub const fn as_zero_based_index(&self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
            Self::D => 3,
            Self::E => 4,
            Self::F => 5,
            Self::G => 6,
            Self::H => 7,
        }
    }
    pub const fn as_shift_offset(&self) -> usize {
        self.as_zero_based_index()
    }

    pub const fn from_u8(num: u8) -> Result<Self, BoardFileError> {
        Ok(match num {
            1 => Self::A,
            2 => Self::B,
            3 => Self::C,
            4 => Self::D,
            5 => Self::E,
            6 => Self::F,
            7 => Self::G,
            8 => Self::H,
            _ => return Err(BoardFileError::InvalidU8(num)),
        })
    }
    pub const fn from_usize(n: usize) -> Result<Self, BoardFileError> {
        Ok(match n {
            1 => Self::A,
            2 => Self::B,
            3 => Self::C,
            4 => Self::D,
            5 => Self::E,
            6 => Self::F,
            7 => Self::G,
            8 => Self::H,
            _ => return Err(BoardFileError::InvalidUsize(n)),
        })
    }
    pub const fn from_char(char: char) -> Result<Self, BoardFileError> {
        Ok(match char {
            'A' | 'a' => Self::A,
            'B' | 'b' => Self::B,
            'C' | 'c' => Self::C,
            'D' | 'd' => Self::D,
            'E' | 'e' => Self::E,
            'F' | 'f' => Self::F,
            'G' | 'g' => Self::G,
            'H' | 'h' => Self::H,
            _ => return Err(BoardFileError::InvalidChar(char)),
        })
    }
    pub const fn from_zero_based_index(n: usize) -> Result<Self, BoardFileError> {
        Ok(match n {
            0 => Self::A,
            1 => Self::B,
            2 => Self::C,
            3 => Self::D,
            4 => Self::E,
            5 => Self::F,
            6 => Self::G,
            7 => Self::H,
            _ => return Err(BoardFileError::InvalidUsize(n)),
        })
    }
    pub const fn next(&self) -> Option<Self> {
        Some(match self {
            Self::A => Self::B,
            Self::B => Self::C,
            Self::C => Self::D,
            Self::D => Self::E,
            Self::E => Self::F,
            Self::F => Self::G,
            Self::G => Self::H,
            Self::H => return None,
        })
    }

    pub const fn prev(&self) -> Option<Self> {
        Some(match self {
            Self::A => return None,
            Self::B => Self::A,
            Self::C => Self::B,
            Self::D => Self::C,
            Self::E => Self::D,
            Self::F => Self::E,
            Self::G => Self::F,
            Self::H => Self::G,
        })
    }
}

impl Display for BoardFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_char())
    }
}
