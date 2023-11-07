use crate::board_file::BoardFile;
use crate::board_position::BoardPosition;
use crate::direction::SimpleDirection;

pub enum BoardSide {
    King,
    Queen,
}

impl BoardSide {
    pub fn to_direction(&self) -> SimpleDirection {
        match self {
            BoardSide::King => SimpleDirection::East,
            BoardSide::Queen => SimpleDirection::West,
        }
    }
    pub fn from_pos(pos: BoardPosition) -> BoardSide {
        let BoardPosition(file, _rank) = pos;
        match file {
            BoardFile::A => BoardSide::Queen,
            BoardFile::B => BoardSide::Queen,
            BoardFile::C => BoardSide::Queen,
            BoardFile::D => BoardSide::Queen,
            BoardFile::E => BoardSide::King,
            BoardFile::F => BoardSide::King,
            BoardFile::G => BoardSide::King,
            BoardFile::H => BoardSide::King,
        }
    }
}