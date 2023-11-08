use std::{ops, usize};
use std::cmp::Ordering;
use bitmaps::Bitmap;
use crate::board_file::BoardFile;
use crate::{board_position, position};
use crate::board_position::BoardPosition;
use crate::board_rank::BoardRank;

pub const SIZE: usize = 64;
const PLACES: usize = 8;
const ZERO_INDEX_PLACES: usize = PLACES - 1;

pub type BitBoardData = Bitmap<SIZE>;

#[repr(transparent)]
#[derive(Default, Clone)]
pub struct BitBoard(BitBoardData);

pub const EMPTY: u64 =  0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
pub const FULL: u64 =   0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111;
pub const FULL_RANK: u64 = 0b11111111;
pub const FULL_FILE: u64 = 0b_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
pub const FULL_DIAG_RIGHT: u64 = 0b_10000000_01000000_00100000_00010000_00001000_00000100_00000010_00000001;
pub const FULL_DIAG_LEFT: u64 = 0b_00000001_00000010_00000100_00001000_00010000_00100000_01000000_10000000;

pub const THREE_X_THREE: u64 = 0b_00000000_00000000_00000000_00000000_00000000_00000111_00000111_00000111;

pub const PAWN: u64 =   0b11111111;
pub const ROOK: u64 =   0b10000001;
pub const KNIGHT: u64 = 0b01000010;
pub const BISHOP: u64 = 0b00100100;
pub const QUEEN: u64 =  0b00001000;
pub const KING: u64 =   0b00010000;


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
    pub fn fill_plus_from_pos(&mut self, board_position: BoardPosition) -> &mut Self {
        self.fill_file(*board_position.file());
        self.fill_rank(*board_position.rank());
        self
    }
    pub fn fill_diag_from_pos(&mut self, board_position: BoardPosition) -> &mut Self {
        let file_num = board_position.file().as_zero_based_index();
        let rank_num = board_position.rank().as_zero_based_index();
        let sum = file_num + rank_num;
        let left_offset = sum.abs_diff(ZERO_INDEX_PLACES);
        let right_offset = file_num.abs_diff(rank_num);


        let mut left: u64 = FULL_DIAG_LEFT;
        let mut right: u64 = FULL_DIAG_RIGHT;

        // if left_offset != 0:
        let shift_amount = PLACES * left_offset;
        if sum < ZERO_INDEX_PLACES {
            // negative offset shifts right
            left >>= shift_amount;
        } else {
            // positive offset shifts left
            left <<= shift_amount;
        }

        // if right_offset != 0:
        let shift_amount = PLACES * right_offset;
        if rank_num < file_num {
            // negative offset shifts right
            right >>= shift_amount;
        } else {
            // positive offset shifts left
            right <<= shift_amount;
        }

        let cross: u64 = left | right;
        *self.bitmap_mut() |= BitBoardData::from_value(cross);
        self
    }
    pub fn fill_3x3_from_pos(&mut self, board_position: BoardPosition) -> &mut Self {
        let index = board_position.as_pos_index();

        // center
        self.bitmap_mut().set(index, true);
        // west, center col
        if board_position.file() > &BoardFile::A {
            self.bitmap_mut().set(index - 1, true);
        }
        // east, center col
        if board_position.file() < &BoardFile::H  {
            self.bitmap_mut().set(index + 1, true);
        };

        // north line
        if board_position.rank() < &BoardRank::Eight {
            // north, center col
            self.bitmap_mut().set(index + PLACES, true);
            // north, west col
            if board_position.file() > &BoardFile::A {
                self.bitmap_mut().set(index + PLACES - 1, true);
            }
            // north, east col
            if board_position.file() < &BoardFile::H {
                self.bitmap_mut().set(index + 8 + 1, true);
            }
        }

        // south line
        if board_position.rank() > &BoardRank::One {
            // south, center col
            self.bitmap_mut().set(index - PLACES, true);
            // south, west col
            if board_position.file() > &BoardFile::A {
                self.bitmap_mut().set(index - PLACES - 1, true);
            }
            // south, east col
            if board_position.file() < &BoardFile::H {
                self.bitmap_mut().set(index - PLACES + 1, true);
            }
        }

        self
    }
    pub fn as_multiline_str(&self) -> String {
        let mut res = String::from(" ┌ABCDEFGH");
        for ix in 0..SIZE {
            if ix % PLACES == 0 {
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

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::bit_board::BitBoard;
    use crate::board_file::BoardFile;
    use crate::board_position::BoardPosition;
    use crate::board_rank::BoardRank;
    use crate::position::*;

    #[rstest]
    #[case(A1, " ┌ABCDEFGH|0\n\
                1|#       |8\n\
                2| #      |16\n\
                3|  #     |24\n\
                4|   #    |32\n\
                5|    #   |40\n\
                6|     #  |48\n\
                7|      # |56\n\
                8|       #|64")]
    #[case(B2, " ┌ABCDEFGH|0\n\
                1|# #     |8\n\
                2| #      |16\n\
                3|# #     |24\n\
                4|   #    |32\n\
                5|    #   |40\n\
                6|     #  |48\n\
                7|      # |56\n\
                8|       #|64")]
    #[case(E4, " ┌ABCDEFGH|0\n\
                1| #     #|8\n\
                2|  #   # |16\n\
                3|   # #  |24\n\
                4|    #   |32\n\
                5|   # #  |40\n\
                6|  #   # |48\n\
                7| #     #|56\n\
                8|#       |64")]
    #[case(A8, " ┌ABCDEFGH|0\n\
                1|       #|8\n\
                2|      # |16\n\
                3|     #  |24\n\
                4|    #   |32\n\
                5|   #    |40\n\
                6|  #     |48\n\
                7| #      |56\n\
                8|#       |64")]
    #[case(H8, " ┌ABCDEFGH|0\n\
                1|#       |8\n\
                2| #      |16\n\
                3|  #     |24\n\
                4|   #    |32\n\
                5|    #   |40\n\
                6|     #  |48\n\
                7|      # |56\n\
                8|       #|64")]
    fn fill_diag_from_pos(
        #[case] pos: BoardPosition,
        #[case] expected: &'static str,
    ) {
        assert_eq!(expected, BitBoard::default().fill_diag_from_pos(pos).as_multiline_str())
    }

    #[rstest]
    #[case(BoardFile::A, " ┌ABCDEFGH|0\n\
                          1|#       |8\n\
                          2|#       |16\n\
                          3|#       |24\n\
                          4|#       |32\n\
                          5|#       |40\n\
                          6|#       |48\n\
                          7|#       |56\n\
                          8|#       |64")]
    #[rstest]
    #[case(BoardFile::E, " ┌ABCDEFGH|0\n\
                          1|    #   |8\n\
                          2|    #   |16\n\
                          3|    #   |24\n\
                          4|    #   |32\n\
                          5|    #   |40\n\
                          6|    #   |48\n\
                          7|    #   |56\n\
                          8|    #   |64")]
    fn fill_file(
        #[case] file: BoardFile,
        #[case] expected: &'static str,
    ) {
        assert_eq!(expected, BitBoard::default().fill_file(file).as_multiline_str())
    }

    #[rstest]
    #[case(BoardRank::One, " ┌ABCDEFGH|0\n\
                            1|########|8\n\
                            2|        |16\n\
                            3|        |24\n\
                            4|        |32\n\
                            5|        |40\n\
                            6|        |48\n\
                            7|        |56\n\
                            8|        |64")]
    #[rstest]
    #[case(BoardRank::Five, " ┌ABCDEFGH|0\n\
                            1|        |8\n\
                            2|        |16\n\
                            3|        |24\n\
                            4|        |32\n\
                            5|########|40\n\
                            6|        |48\n\
                            7|        |56\n\
                            8|        |64")]
    fn fill_rank(
        #[case] rank: BoardRank,
        #[case] expected: &'static str,
    ) {
        assert_eq!(expected, BitBoard::default().fill_rank(rank).as_multiline_str())
    }

    #[rstest]
    #[case(A1, " ┌ABCDEFGH|0\n\
                1|########|8\n\
                2|#       |16\n\
                3|#       |24\n\
                4|#       |32\n\
                5|#       |40\n\
                6|#       |48\n\
                7|#       |56\n\
                8|#       |64")]
    #[rstest]
    #[case(E5, " ┌ABCDEFGH|0\n\
                1|    #   |8\n\
                2|    #   |16\n\
                3|    #   |24\n\
                4|    #   |32\n\
                5|########|40\n\
                6|    #   |48\n\
                7|    #   |56\n\
                8|    #   |64")]
    fn fill_plus_from_pos(
        #[case] pos: BoardPosition,
        #[case] expected: &'static str,
    ) {
        assert_eq!(expected, BitBoard::default().fill_plus_from_pos(pos).as_multiline_str())
    }

    #[rstest]
    #[case(A1, " ┌ABCDEFGH|0\n\
                1|##      |8\n\
                2|##      |16\n\
                3|        |24\n\
                4|        |32\n\
                5|        |40\n\
                6|        |48\n\
                7|        |56\n\
                8|        |64")]
    #[rstest]
    #[case(B2, " ┌ABCDEFGH|0\n\
                1|###     |8\n\
                2|###     |16\n\
                3|###     |24\n\
                4|        |32\n\
                5|        |40\n\
                6|        |48\n\
                7|        |56\n\
                8|        |64")]
    #[case(G7, " ┌ABCDEFGH|0\n\
                1|        |8\n\
                2|        |16\n\
                3|        |24\n\
                4|        |32\n\
                5|        |40\n\
                6|     ###|48\n\
                7|     ###|56\n\
                8|     ###|64")]
    #[case(H8, " ┌ABCDEFGH|0\n\
                1|        |8\n\
                2|        |16\n\
                3|        |24\n\
                4|        |32\n\
                5|        |40\n\
                6|        |48\n\
                7|      ##|56\n\
                8|      ##|64")]
    #[case(H1, " ┌ABCDEFGH|0\n\
                1|      ##|8\n\
                2|      ##|16\n\
                3|        |24\n\
                4|        |32\n\
                5|        |40\n\
                6|        |48\n\
                7|        |56\n\
                8|        |64")]
    fn fill_3x3_from_pos(
        #[case] pos: BoardPosition,
        #[case] expected: &'static str,
    ) {
        println!("{pos}: \n{}", BitBoard::default().fill_3x3_from_pos(pos).as_multiline_str());
        assert_eq!(expected, BitBoard::default().fill_3x3_from_pos(pos).as_multiline_str())
    }
}
