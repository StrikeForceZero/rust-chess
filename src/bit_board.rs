use std::{ops, usize};
use bitmaps::Bitmap;
use crate::board_file::BoardFile;
use crate::board_position;
use crate::board_position::BoardPosition;
use crate::board_rank::BoardRank;

pub type BitBoardData = Bitmap<64>;

#[repr(transparent)]
#[derive(Default, Clone)]
pub struct BitBoard(BitBoardData);

pub const EMPTY: u64 =  0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
pub const FULL: u64 =   0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111;
pub const FULL_RANK: u64 = 0b11111111;
pub const FULL_FILE: u64 = 0b_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
pub const FULL_DIAG_RIGHT: u64 = 0b_10000000_01000000_00100000_00010000_00001000_00000100_00000010_00000001;
pub const FULL_DIAG_LEFT: u64 = 0b_00000001_00000010_00000100_00001000_00010000_00100000_01000000_10000000;

pub const PAWN: u64 =   0b11111111;
pub const ROOK: u64 =   0b10000001;
pub const KNIGHT: u64 = 0b01000010;
pub const BISHOP: u64 = 0b00100100;
pub const QUEEN: u64 =  0b00001000;
pub const KING: u64 =   0b00010000;

const PLACES: usize = 7;
const PLACES_I8: i8 = PLACES as i8;

impl BitBoard {
    pub fn bitmap(&self) -> &BitBoardData {
        &self.0
    }
    pub fn bitmap_mut(&mut self) -> &mut BitBoardData {
        &mut self.0
    }
    pub fn from(bit_board_data: BitBoardData) -> Self {
        Self(bit_board_data)
    }
    pub fn from_value(value: u64) -> Self {
        Self(BitBoardData::from_value(value))
    }
    pub fn set_pos(&mut self, board_position: BoardPosition, value: bool) -> bool {
        let index = board_position.as_pos_index();
        self.bitmap_mut().set(index, value)
    }
    pub fn get_pos(&self, board_position: BoardPosition) -> bool {
        let index = board_position.as_pos_index();
        self.bitmap().get(index)
    }
    pub fn fill_rank(&mut self, board_rank: BoardRank) -> &mut Self {
        *self.bitmap_mut() |= BitBoardData::from_value(FULL_RANK << board_rank.as_shift_offset());
        self
    }
    pub fn fill_file(&mut self, board_file: BoardFile) -> &mut Self {
        *self.bitmap_mut() |= BitBoardData::from_value(FULL_FILE << board_file.as_shift_offset());
        self
    }
    pub fn fill_diag_from_pos(&mut self, board_position: BoardPosition) -> &mut Self {
        let file_num = board_position.file().as_zero_based_index();
        let rank_num = board_position.rank().as_zero_based_index();
        let sum = file_num + rank_num;
        let difference = file_num as i8 - rank_num as i8;
        let left_offset = sum.abs_diff(PLACES);
        let right_offset = PLACES_I8 - difference - PLACES_I8;


        let mut left: u64 = FULL_DIAG_LEFT;
        let mut right: u64 = FULL_DIAG_RIGHT;

        if left_offset != 0 {
            let shift_amount = 8 * left_offset;
            if sum < PLACES {
                // negative offset shifts right
                left >>= shift_amount;
            } else {
                // positive offset shifts left
                left <<= shift_amount;
            }
        }

        if right_offset != 0 {
            let shift_amount = 8 * right_offset.abs();
            if right_offset < 0 {
                // negative offset shifts right
                right >>= shift_amount;
            } else {
                // positive offset shifts left
                right <<= shift_amount;
            }
        }

        let cross: u64 = left | right;
        *self.bitmap_mut() |= BitBoardData::from_value(cross);
        self
    }
    pub fn as_multiline_str(&self) -> String {
        let mut res = String::from(" ┌ABCDEFGH");
        for ix in 0..64 {
            if ix % 8 == 0 {
                res.push('|');
                res.push_str(ix.to_string().as_str());
                res.push('\n');
                res.push(BoardRank::from_zero_based_index(ix / 8).unwrap().as_char());
                res.push('|');
            }
            if self.bitmap().get(ix) {
                res.push('#');
            } else {
                res.push(' ');
            }
        }
        res.push('|');
        res.push_str("64");
        res
    }
}
