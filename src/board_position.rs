use std::fmt::{Display, Formatter};
use crate::board_file::BoardFile;
use crate::board_rank::BoardRank;
use crate::direction::Direction;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct BoardPosition(pub BoardFile, pub BoardRank);

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
}

impl Display for BoardPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let BoardPosition(file, rank) = self;
        write!(f, "{file}{rank}")
    }
}
