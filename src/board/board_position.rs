use crate::board::board_file::BoardFile;
use crate::board::board_rank::BoardRank;
use crate::board::position;
use crate::direction::direction::Direction;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd)]
pub struct BoardPosition(pub BoardFile, pub BoardRank);

impl BoardPosition {}

impl BoardPosition {
    pub const fn file(&self) -> &BoardFile {
        &self.0
    }
    pub const fn rank(&self) -> &BoardRank {
        &self.1
    }
    pub const fn from(board_file: BoardFile, board_rank: BoardRank) -> Self {
        Self(board_file, board_rank)
    }
    pub const fn as_pos_index(&self) -> usize {
        let rank_index = self.rank().as_zero_based_index();
        let file_index = self.file().as_zero_based_index();
        rank_index * 8 + file_index
    }

    pub const fn next_pos(self, direction: Direction) -> Option<Self> {
        direction.get_next_pos(self)
    }

    pub fn from_str(s: &str) -> Result<Self, BoardPositionStrParseError> {
        let chars = s.chars().collect::<Vec<_>>();
        let (file_char, rank_char) = match chars.as_slice() {
            [file, rank] => (*file, *rank),
            _ => return Err(BoardPositionStrParseError::InvalidNumberOfChars(s.to_string()))
        };
        let Ok(file) = BoardFile::from_char(file_char) else {
            return Err(BoardPositionStrParseError::InvalidFileOrRank(s.to_string()));
        };
        let Ok(rank) = BoardRank::from_char(rank_char) else {
            return Err(BoardPositionStrParseError::InvalidFileOrRank(s.to_string()));
        };
        Ok(BoardPosition(file, rank))
    }

    pub const fn as_score(self) -> i32 {
        match self {
            position::D4
            | position::D5
            | position::E4
            | position::E5 => 3,
            position::C3
            | position::C4
            | position::C5
            | position::C6
            | position::D6
            | position::E6
            | position::F6
            | position::F5
            | position::F4
            | position::F3
            | position::E3
            | position::D3 => 1,
            _ => 0,
        }
    }
}

impl Display for BoardPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let BoardPosition(file, rank) = self;
        write!(f, "{file}{rank}")
    }
}

impl Debug for BoardPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let BoardPosition(file, rank) = self;
        write!(f, "{file}{rank}")
    }
}

#[derive(Error, Debug, Clone)]
pub enum BoardPositionStrParseError {
    #[error("Invalid number of chars for BoardPosition: {0}")]
    InvalidNumberOfChars(String),

    #[error("Invalid file or rank for BoardPosition: {0}")]
    InvalidFileOrRank(String),
}

impl FromStr for BoardPosition {
    type Err = BoardPositionStrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BoardPosition::from_str(s)
    }
}
