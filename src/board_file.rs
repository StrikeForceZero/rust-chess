use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
pub enum BoardFileError {
    #[error("Invalid character for BoardFile: {0}")]
    InvalidChar(char),
    #[error("Invalid usize for BoardFile: {0}")]
    InvalidUsize(usize),
}

#[derive(Copy, Clone)]
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
    pub fn as_char(&self) -> char {
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
    pub fn as_zero_based_index(&self) -> usize {
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
    pub fn as_shift_offset(&self) -> usize {
        self.as_zero_based_index()
    }
    pub fn from_char(char: char) -> Result<Self, BoardFileError> {
        Ok(match char {
            'A' | 'a' => Self::A,
            'B' | 'b' => Self::B,
            'C' | 'c' => Self::C,
            'D' | 'd' => Self::D,
            'E' | 'e' => Self::E,
            'F' | 'f' => Self::F,
            'G' | 'g' => Self::G,
            'H' | 'h' => Self::H,
            _ => return Err(BoardFileError::InvalidChar(char))
        })
    }
    pub fn from_zero_based_index(n: usize) -> Result<Self, BoardFileError> {
        Ok(match n {
            0 => Self::A,
            1 => Self::B,
            2 => Self::C,
            3 => Self::D,
            4 => Self::E,
            5 => Self::F,
            6 => Self::G,
            7 => Self::H,
            _ => return Err(BoardFileError::InvalidUsize(n))
        })
    }
}

impl Display for BoardFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_char())
    }
}
