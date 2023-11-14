use crate::board::board_file::BoardFile;
use crate::board::board_position::BoardPosition;
use crate::board::board_rank::BoardRank;
use crate::board::position;
use crate::direction::direction::Direction;
use crate::utils::custom_struct_iterator::CustomStructIterator;
use bitmaps::Bitmap;

pub const SIZE: usize = 64;
const PLACES: usize = 8;
const PLACES_ISIZE: isize = PLACES as isize;
const ZERO_INDEX_PLACES: usize = PLACES - 1;

pub type BitBoardData = Bitmap<SIZE>;

#[repr(transparent)]
#[derive(Default, Clone)]
pub struct BitBoard(BitBoardData);

pub const EMPTY: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
pub const FULL: u64 = 0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111;
pub const FULL_RANK: u64 = 0b11111111;
pub const FULL_FILE: u64 =
    0b_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
pub const FULL_DIAG_RIGHT: u64 =
    0b_10000000_01000000_00100000_00010000_00001000_00000100_00000010_00000001;
pub const FULL_DIAG_LEFT: u64 =
    0b_00000001_00000010_00000100_00001000_00010000_00100000_01000000_10000000;

pub const THREE_X_THREE: u64 =
    0b_00000000_00000000_00000000_00000000_00000000_00000111_00000111_00000111;

pub const PAWN_STARTING_POS: u64 = 0b11111111;
pub const ROOK_STARTING_POS: u64 = 0b10000001;
pub const KNIGHT_STARTING_POS: u64 = 0b01000010;
pub const BISHOP_STARTING_POS: u64 = 0b00100100;
pub const QUEEN_STARTING_POS: u64 = 0b00001000;
pub const KING_STARTING_POS: u64 = 0b00010000;

const KNIGHT_MOVES: [(isize, isize); 8] = [
    (1, 2),
    (2, 1),
    (-1, 2),
    (2, -1),
    (-1, -2),
    (-2, -1),
    (1, -2),
    (-2, 1),
];

impl BitBoard {
    pub const fn bitmap(&self) -> &BitBoardData {
        &self.0
    }
    pub fn bitmap_mut(&mut self) -> &mut BitBoardData {
        &mut self.0
    }
    pub fn new() -> Self {
        Self::from_value(0)
    }
    pub const fn from(bit_board_data: BitBoardData) -> Self {
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
    pub const fn lookup_knight(board_position: BoardPosition) -> u64 {
        use position::*;
        match board_position {
            A1 => 0b0000000000000000000000000000000000000000000000100000010000000000,
            A2 => 0b0000000000000000000000000000000000000010000001000000000000000100,
            A3 => 0b0000000000000000000000000000001000000100000000000000010000000010,
            A4 => 0b0000000000000000000000100000010000000000000001000000001000000000,
            A5 => 0b0000000000000010000001000000000000000100000000100000000000000000,
            A6 => 0b0000001000000100000000000000010000000010000000000000000000000000,
            A7 => 0b0000010000000000000001000000001000000000000000000000000000000000,
            A8 => 0b0000000000000100000000100000000000000000000000000000000000000000,
            B1 => 0b0000000000000000000000000000000000000000000001010000100000000000,
            B2 => 0b0000000000000000000000000000000000000101000010000000000000001000,
            B3 => 0b0000000000000000000000000000010100001000000000000000100000000101,
            B4 => 0b0000000000000000000001010000100000000000000010000000010100000000,
            B5 => 0b0000000000000101000010000000000000001000000001010000000000000000,
            B6 => 0b0000010100001000000000000000100000000101000000000000000000000000,
            B7 => 0b0000100000000000000010000000010100000000000000000000000000000000,
            B8 => 0b0000000000001000000001010000000000000000000000000000000000000000,
            C1 => 0b0000000000000000000000000000000000000000000010100001000100000000,
            C2 => 0b0000000000000000000000000000000000001010000100010000000000010001,
            C3 => 0b0000000000000000000000000000101000010001000000000001000100001010,
            C4 => 0b0000000000000000000010100001000100000000000100010000101000000000,
            C5 => 0b0000000000001010000100010000000000010001000010100000000000000000,
            C6 => 0b0000101000010001000000000001000100001010000000000000000000000000,
            C7 => 0b0001000100000000000100010000101000000000000000000000000000000000,
            C8 => 0b0000000000010001000010100000000000000000000000000000000000000000,
            D1 => 0b0000000000000000000000000000000000000000000101000010001000000000,
            D2 => 0b0000000000000000000000000000000000010100001000100000000000100010,
            D3 => 0b0000000000000000000000000001010000100010000000000010001000010100,
            D4 => 0b0000000000000000000101000010001000000000001000100001010000000000,
            D5 => 0b0000000000010100001000100000000000100010000101000000000000000000,
            D6 => 0b0001010000100010000000000010001000010100000000000000000000000000,
            D7 => 0b0010001000000000001000100001010000000000000000000000000000000000,
            D8 => 0b0000000000100010000101000000000000000000000000000000000000000000,
            E1 => 0b0000000000000000000000000000000000000000001010000100010000000000,
            E2 => 0b0000000000000000000000000000000000101000010001000000000001000100,
            E3 => 0b0000000000000000000000000010100001000100000000000100010000101000,
            E4 => 0b0000000000000000001010000100010000000000010001000010100000000000,
            E5 => 0b0000000000101000010001000000000001000100001010000000000000000000,
            E6 => 0b0010100001000100000000000100010000101000000000000000000000000000,
            E7 => 0b0100010000000000010001000010100000000000000000000000000000000000,
            E8 => 0b0000000001000100001010000000000000000000000000000000000000000000,
            F1 => 0b0000000000000000000000000000000000000000010100001000100000000000,
            F2 => 0b0000000000000000000000000000000001010000100010000000000010001000,
            F3 => 0b0000000000000000000000000101000010001000000000001000100001010000,
            F4 => 0b0000000000000000010100001000100000000000100010000101000000000000,
            F5 => 0b0000000001010000100010000000000010001000010100000000000000000000,
            F6 => 0b0101000010001000000000001000100001010000000000000000000000000000,
            F7 => 0b1000100000000000100010000101000000000000000000000000000000000000,
            F8 => 0b0000000010001000010100000000000000000000000000000000000000000000,
            G1 => 0b0000000000000000000000000000000000000000101000000001000000000000,
            G2 => 0b0000000000000000000000000000000010100000000100000000000000010000,
            G3 => 0b0000000000000000000000001010000000010000000000000001000010100000,
            G4 => 0b0000000000000000101000000001000000000000000100001010000000000000,
            G5 => 0b0000000010100000000100000000000000010000101000000000000000000000,
            G6 => 0b1010000000010000000000000001000010100000000000000000000000000000,
            G7 => 0b0001000000000000000100001010000000000000000000000000000000000000,
            G8 => 0b0000000000010000101000000000000000000000000000000000000000000000,
            H1 => 0b0000000000000000000000000000000000000000010000000010000000000000,
            H2 => 0b0000000000000000000000000000000001000000001000000000000000100000,
            H3 => 0b0000000000000000000000000100000000100000000000000010000001000000,
            H4 => 0b0000000000000000010000000010000000000000001000000100000000000000,
            H5 => 0b0000000001000000001000000000000000100000010000000000000000000000,
            H6 => 0b0100000000100000000000000010000001000000000000000000000000000000,
            H7 => 0b0010000000000000001000000100000000000000000000000000000000000000,
            H8 => 0b0000000000100000010000000000000000000000000000000000000000000000,
        }
    }
    pub const fn lookup_bishop(board_position: BoardPosition) -> u64 {
        use position::*;
        match board_position {
            A1 => 0b1000000001000000001000000001000000001000000001000000001000000000,
            A2 => 0b0100000000100000000100000000100000000100000000100000000000000010,
            A3 => 0b0010000000010000000010000000010000000010000000000000001000000100,
            A4 => 0b0001000000001000000001000000001000000000000000100000010000001000,
            A5 => 0b0000100000000100000000100000000000000010000001000000100000010000,
            A6 => 0b0000010000000010000000000000001000000100000010000001000000100000,
            A7 => 0b0000001000000000000000100000010000001000000100000010000001000000,
            A8 => 0b0000000000000010000001000000100000010000001000000100000010000000,
            B1 => 0b0000000010000000010000000010000000010000000010000000010100000000,
            B2 => 0b1000000001000000001000000001000000001000000001010000000000000101,
            B3 => 0b0100000000100000000100000000100000000101000000000000010100001000,
            B4 => 0b0010000000010000000010000000010100000000000001010000100000010000,
            B5 => 0b0001000000001000000001010000000000000101000010000001000000100000,
            B6 => 0b0000100000000101000000000000010100001000000100000010000001000000,
            B7 => 0b0000010100000000000001010000100000010000001000000100000010000000,
            B8 => 0b0000000000000101000010000001000000100000010000001000000000000000,
            C1 => 0b0000000000000000100000000100000000100000000100010000101000000000,
            C2 => 0b0000000010000000010000000010000000010001000010100000000000001010,
            C3 => 0b1000000001000000001000000001000100001010000000000000101000010001,
            C4 => 0b0100000000100000000100010000101000000000000010100001000100100000,
            C5 => 0b0010000000010001000010100000000000001010000100010010000001000000,
            C6 => 0b0001000100001010000000000000101000010001001000000100000010000000,
            C7 => 0b0000101000000000000010100001000100100000010000001000000000000000,
            C8 => 0b0000000000001010000100010010000001000000100000000000000000000000,
            D1 => 0b0000000000000000000000001000000001000001001000100001010000000000,
            D2 => 0b0000000000000000100000000100000100100010000101000000000000010100,
            D3 => 0b0000000010000000010000010010001000010100000000000001010000100010,
            D4 => 0b1000000001000001001000100001010000000000000101000010001001000001,
            D5 => 0b0100000100100010000101000000000000010100001000100100000110000000,
            D6 => 0b0010001000010100000000000001010000100010010000011000000000000000,
            D7 => 0b0001010000000000000101000010001001000001100000000000000000000000,
            D8 => 0b0000000000010100001000100100000110000000000000000000000000000000,
            E1 => 0b0000000000000000000000000000000110000010010001000010100000000000,
            E2 => 0b0000000000000000000000011000001001000100001010000000000000101000,
            E3 => 0b0000000000000001100000100100010000101000000000000010100001000100,
            E4 => 0b0000000110000010010001000010100000000000001010000100010010000010,
            E5 => 0b1000001001000100001010000000000000101000010001001000001000000001,
            E6 => 0b0100010000101000000000000010100001000100100000100000000100000000,
            E7 => 0b0010100000000000001010000100010010000010000000010000000000000000,
            E8 => 0b0000000000101000010001001000001000000001000000000000000000000000,
            F1 => 0b0000000000000000000000010000001000000100100010000101000000000000,
            F2 => 0b0000000000000001000000100000010010001000010100000000000001010000,
            F3 => 0b0000000100000010000001001000100001010000000000000101000010001000,
            F4 => 0b0000001000000100100010000101000000000000010100001000100000000100,
            F5 => 0b0000010010001000010100000000000001010000100010000000010000000010,
            F6 => 0b1000100001010000000000000101000010001000000001000000001000000001,
            F7 => 0b0101000000000000010100001000100000000100000000100000000100000000,
            F8 => 0b0000000001010000100010000000010000000010000000010000000000000000,
            G1 => 0b0000000000000001000000100000010000001000000100001010000000000000,
            G2 => 0b0000000100000010000001000000100000010000101000000000000010100000,
            G3 => 0b0000001000000100000010000001000010100000000000001010000000010000,
            G4 => 0b0000010000001000000100001010000000000000101000000001000000001000,
            G5 => 0b0000100000010000101000000000000010100000000100000000100000000100,
            G6 => 0b0001000010100000000000001010000000010000000010000000010000000010,
            G7 => 0b1010000000000000101000000001000000001000000001000000001000000001,
            G8 => 0b0000000010100000000100000000100000000100000000100000000100000000,
            H1 => 0b0000000100000010000001000000100000010000001000000100000000000000,
            H2 => 0b0000001000000100000010000001000000100000010000000000000001000000,
            H3 => 0b0000010000001000000100000010000001000000000000000100000000100000,
            H4 => 0b0000100000010000001000000100000000000000010000000010000000010000,
            H5 => 0b0001000000100000010000000000000001000000001000000001000000001000,
            H6 => 0b0010000001000000000000000100000000100000000100000000100000000100,
            H7 => 0b0100000000000000010000000010000000010000000010000000010000000010,
            H8 => 0b0000000001000000001000000001000000001000000001000000001000000001,
        }
    }
    pub const fn lookup_rook(board_position: BoardPosition) -> u64 {
        use position::*;
        match board_position {
            A1 => 0b0000000100000001000000010000000100000001000000010000000111111110,
            A2 => 0b0000000100000001000000010000000100000001000000011111111000000001,
            A3 => 0b0000000100000001000000010000000100000001111111100000000100000001,
            A4 => 0b0000000100000001000000010000000111111110000000010000000100000001,
            A5 => 0b0000000100000001000000011111111000000001000000010000000100000001,
            A6 => 0b0000000100000001111111100000000100000001000000010000000100000001,
            A7 => 0b0000000111111110000000010000000100000001000000010000000100000001,
            A8 => 0b1111111000000001000000010000000100000001000000010000000100000001,
            B1 => 0b0000001000000010000000100000001000000010000000100000001011111101,
            B2 => 0b0000001000000010000000100000001000000010000000101111110100000010,
            B3 => 0b0000001000000010000000100000001000000010111111010000001000000010,
            B4 => 0b0000001000000010000000100000001011111101000000100000001000000010,
            B5 => 0b0000001000000010000000101111110100000010000000100000001000000010,
            B6 => 0b0000001000000010111111010000001000000010000000100000001000000010,
            B7 => 0b0000001011111101000000100000001000000010000000100000001000000010,
            B8 => 0b1111110100000010000000100000001000000010000000100000001000000010,
            C1 => 0b0000010000000100000001000000010000000100000001000000010011111011,
            C2 => 0b0000010000000100000001000000010000000100000001001111101100000100,
            C3 => 0b0000010000000100000001000000010000000100111110110000010000000100,
            C4 => 0b0000010000000100000001000000010011111011000001000000010000000100,
            C5 => 0b0000010000000100000001001111101100000100000001000000010000000100,
            C6 => 0b0000010000000100111110110000010000000100000001000000010000000100,
            C7 => 0b0000010011111011000001000000010000000100000001000000010000000100,
            C8 => 0b1111101100000100000001000000010000000100000001000000010000000100,
            D1 => 0b0000100000001000000010000000100000001000000010000000100011110111,
            D2 => 0b0000100000001000000010000000100000001000000010001111011100001000,
            D3 => 0b0000100000001000000010000000100000001000111101110000100000001000,
            D4 => 0b0000100000001000000010000000100011110111000010000000100000001000,
            D5 => 0b0000100000001000000010001111011100001000000010000000100000001000,
            D6 => 0b0000100000001000111101110000100000001000000010000000100000001000,
            D7 => 0b0000100011110111000010000000100000001000000010000000100000001000,
            D8 => 0b1111011100001000000010000000100000001000000010000000100000001000,
            E1 => 0b0001000000010000000100000001000000010000000100000001000011101111,
            E2 => 0b0001000000010000000100000001000000010000000100001110111100010000,
            E3 => 0b0001000000010000000100000001000000010000111011110001000000010000,
            E4 => 0b0001000000010000000100000001000011101111000100000001000000010000,
            E5 => 0b0001000000010000000100001110111100010000000100000001000000010000,
            E6 => 0b0001000000010000111011110001000000010000000100000001000000010000,
            E7 => 0b0001000011101111000100000001000000010000000100000001000000010000,
            E8 => 0b1110111100010000000100000001000000010000000100000001000000010000,
            F1 => 0b0010000000100000001000000010000000100000001000000010000011011111,
            F2 => 0b0010000000100000001000000010000000100000001000001101111100100000,
            F3 => 0b0010000000100000001000000010000000100000110111110010000000100000,
            F4 => 0b0010000000100000001000000010000011011111001000000010000000100000,
            F5 => 0b0010000000100000001000001101111100100000001000000010000000100000,
            F6 => 0b0010000000100000110111110010000000100000001000000010000000100000,
            F7 => 0b0010000011011111001000000010000000100000001000000010000000100000,
            F8 => 0b1101111100100000001000000010000000100000001000000010000000100000,
            G1 => 0b0100000001000000010000000100000001000000010000000100000010111111,
            G2 => 0b0100000001000000010000000100000001000000010000001011111101000000,
            G3 => 0b0100000001000000010000000100000001000000101111110100000001000000,
            G4 => 0b0100000001000000010000000100000010111111010000000100000001000000,
            G5 => 0b0100000001000000010000001011111101000000010000000100000001000000,
            G6 => 0b0100000001000000101111110100000001000000010000000100000001000000,
            G7 => 0b0100000010111111010000000100000001000000010000000100000001000000,
            G8 => 0b1011111101000000010000000100000001000000010000000100000001000000,
            H1 => 0b1000000010000000100000001000000010000000100000001000000001111111,
            H2 => 0b1000000010000000100000001000000010000000100000000111111110000000,
            H3 => 0b1000000010000000100000001000000010000000011111111000000010000000,
            H4 => 0b1000000010000000100000001000000001111111100000001000000010000000,
            H5 => 0b1000000010000000100000000111111110000000100000001000000010000000,
            H6 => 0b1000000010000000011111111000000010000000100000001000000010000000,
            H7 => 0b1000000001111111100000001000000010000000100000001000000010000000,
            H8 => 0b0111111110000000100000001000000010000000100000001000000010000000,
        }
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
        self.bitmap_mut().set(board_position.as_pos_index(), false);
        self
    }
    pub fn fill_diag_from_pos(&mut self, board_position: BoardPosition) -> &mut Self {
        let BoardPosition(file, rank) = board_position;
        let file_num = file.as_zero_based_index();
        let rank_num = rank.as_zero_based_index();
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

        self.bitmap_mut().set(board_position.as_pos_index(), false);

        self
    }
    pub fn fill_3x3_from_pos(&mut self, board_position: BoardPosition) -> &mut Self {
        let index = board_position.as_pos_index();
        let BoardPosition(file, rank) = board_position;

        // center
        // self.bitmap_mut().set(index, true);
        // west, center col
        if file > BoardFile::A {
            self.bitmap_mut().set(index - 1, true);
        }
        // east, center col
        if file < BoardFile::H {
            self.bitmap_mut().set(index + 1, true);
        };

        // north line
        if rank < BoardRank::Eight {
            // north, center col
            self.bitmap_mut().set(index + PLACES, true);
            // north, west col
            if file > BoardFile::A {
                self.bitmap_mut().set(index + PLACES - 1, true);
            }
            // north, east col
            if file < BoardFile::H {
                self.bitmap_mut().set(index + PLACES + 1, true);
            }
        }

        // south line
        if rank > BoardRank::One {
            // south, center col
            self.bitmap_mut().set(index - PLACES, true);
            // south, west col
            if file > BoardFile::A {
                self.bitmap_mut().set(index - PLACES - 1, true);
            }
            // south, east col
            if file < BoardFile::H {
                self.bitmap_mut().set(index - PLACES + 1, true);
            }
        }

        self
    }
    pub fn fill_l_jump_from_pos(&mut self, board_position: BoardPosition) -> &mut Self {
        let BoardPosition(file, rank) = board_position;
        let file_index = file.as_zero_based_index() as isize;
        let rank_index = rank.as_zero_based_index() as isize;

        for (file_offset, rank_offset) in KNIGHT_MOVES {
            let new_file = file_index + file_offset;
            let new_rank = rank_index + rank_offset;

            // Check if the new position is on the board
            if (0..PLACES_ISIZE).contains(&new_file) && (0..PLACES_ISIZE).contains(&new_rank) {
                let new_index = (new_rank * PLACES_ISIZE + new_file) as usize;
                self.bitmap_mut().set(new_index, true);
            }
        }

        self
    }
    pub fn fill_single_from_pos(
        &mut self,
        board_position: BoardPosition,
        direction: Direction,
    ) -> &mut Self {
        if let Some(target_pos) = direction.get_next_pos(board_position) {
            self.bitmap_mut().set(target_pos.as_pos_index(), true);
        }
        self
    }
    pub fn fill_double_from_pos(
        &mut self,
        board_position: BoardPosition,
        direction: Direction,
    ) -> &mut Self {
        let first = direction.get_next_pos(board_position);
        if let Some(first_pos) = first {
            if let Some(target_pos) = direction.get_next_pos(first_pos) {
                // make sure both are available before settings
                self.bitmap_mut().set(first_pos.as_pos_index(), true);
                self.bitmap_mut().set(target_pos.as_pos_index(), true);
            }
        }
        self
    }
    pub fn fill_single_diags_from_pos(
        &mut self,
        board_position: BoardPosition,
        direction: Direction,
    ) -> &mut Self {
        let (a, b) = direction.split();
        if let Some(target_pos) = a.get_next_pos(board_position) {
            self.bitmap_mut().set(target_pos.as_pos_index(), true);
        }
        if let Some(target_pos) = b.get_next_pos(board_position) {
            self.bitmap_mut().set(target_pos.as_pos_index(), true);
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
    pub const fn as_iter(&self) -> CustomStructIterator<BitBoard> {
        CustomStructIterator {
            data: self,
            index: 0,
        }
    }
}

impl Iterator for CustomStructIterator<'_, BitBoard> {
    type Item = (BoardPosition, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let res = Some(match self.index {
            0 => (position::A1, self.data.bitmap().get(position::A1.as_pos_index())),
            1 => (position::A2, self.data.bitmap().get(position::A2.as_pos_index())),
            2 => (position::A3, self.data.bitmap().get(position::A3.as_pos_index())),
            3 => (position::A4, self.data.bitmap().get(position::A4.as_pos_index())),
            4 => (position::A5, self.data.bitmap().get(position::A5.as_pos_index())),
            5 => (position::A6, self.data.bitmap().get(position::A6.as_pos_index())),
            6 => (position::A7, self.data.bitmap().get(position::A7.as_pos_index())),
            7 => (position::A8, self.data.bitmap().get(position::A8.as_pos_index())),
            8 => (position::B1, self.data.bitmap().get(position::B1.as_pos_index())),
            9 => (position::B2, self.data.bitmap().get(position::B2.as_pos_index())),
            10 => (position::B3, self.data.bitmap().get(position::B3.as_pos_index())),
            11 => (position::B4, self.data.bitmap().get(position::B4.as_pos_index())),
            12 => (position::B5, self.data.bitmap().get(position::B5.as_pos_index())),
            13 => (position::B6, self.data.bitmap().get(position::B6.as_pos_index())),
            14 => (position::B7, self.data.bitmap().get(position::B7.as_pos_index())),
            15 => (position::B8, self.data.bitmap().get(position::B8.as_pos_index())),
            16 => (position::C1, self.data.bitmap().get(position::C1.as_pos_index())),
            17 => (position::C2, self.data.bitmap().get(position::C2.as_pos_index())),
            18 => (position::C3, self.data.bitmap().get(position::C3.as_pos_index())),
            19 => (position::C4, self.data.bitmap().get(position::C4.as_pos_index())),
            20 => (position::C5, self.data.bitmap().get(position::C5.as_pos_index())),
            21 => (position::C6, self.data.bitmap().get(position::C6.as_pos_index())),
            22 => (position::C7, self.data.bitmap().get(position::C7.as_pos_index())),
            23 => (position::C8, self.data.bitmap().get(position::C8.as_pos_index())),
            24 => (position::D1, self.data.bitmap().get(position::D1.as_pos_index())),
            25 => (position::D2, self.data.bitmap().get(position::D2.as_pos_index())),
            26 => (position::D3, self.data.bitmap().get(position::D3.as_pos_index())),
            27 => (position::D4, self.data.bitmap().get(position::D4.as_pos_index())),
            28 => (position::D5, self.data.bitmap().get(position::D5.as_pos_index())),
            29 => (position::D6, self.data.bitmap().get(position::D6.as_pos_index())),
            30 => (position::D7, self.data.bitmap().get(position::D7.as_pos_index())),
            31 => (position::D8, self.data.bitmap().get(position::D8.as_pos_index())),
            32 => (position::E1, self.data.bitmap().get(position::E1.as_pos_index())),
            33 => (position::E2, self.data.bitmap().get(position::E2.as_pos_index())),
            34 => (position::E3, self.data.bitmap().get(position::E3.as_pos_index())),
            35 => (position::E4, self.data.bitmap().get(position::E4.as_pos_index())),
            36 => (position::E5, self.data.bitmap().get(position::E5.as_pos_index())),
            37 => (position::E6, self.data.bitmap().get(position::E6.as_pos_index())),
            38 => (position::E7, self.data.bitmap().get(position::E7.as_pos_index())),
            39 => (position::E8, self.data.bitmap().get(position::E8.as_pos_index())),
            40 => (position::F1, self.data.bitmap().get(position::F1.as_pos_index())),
            41 => (position::F2, self.data.bitmap().get(position::F2.as_pos_index())),
            42 => (position::F3, self.data.bitmap().get(position::F3.as_pos_index())),
            43 => (position::F4, self.data.bitmap().get(position::F4.as_pos_index())),
            44 => (position::F5, self.data.bitmap().get(position::F5.as_pos_index())),
            45 => (position::F6, self.data.bitmap().get(position::F6.as_pos_index())),
            46 => (position::F7, self.data.bitmap().get(position::F7.as_pos_index())),
            47 => (position::F8, self.data.bitmap().get(position::F8.as_pos_index())),
            48 => (position::H1, self.data.bitmap().get(position::H1.as_pos_index())),
            49 => (position::H2, self.data.bitmap().get(position::H2.as_pos_index())),
            50 => (position::H3, self.data.bitmap().get(position::H3.as_pos_index())),
            51 => (position::H4, self.data.bitmap().get(position::H4.as_pos_index())),
            52 => (position::H5, self.data.bitmap().get(position::H5.as_pos_index())),
            53 => (position::H6, self.data.bitmap().get(position::H6.as_pos_index())),
            54 => (position::H7, self.data.bitmap().get(position::H7.as_pos_index())),
            55 => (position::H8, self.data.bitmap().get(position::H8.as_pos_index())),
            56 => (position::G1, self.data.bitmap().get(position::G1.as_pos_index())),
            57 => (position::G2, self.data.bitmap().get(position::G2.as_pos_index())),
            58 => (position::G3, self.data.bitmap().get(position::G3.as_pos_index())),
            59 => (position::G4, self.data.bitmap().get(position::G4.as_pos_index())),
            60 => (position::G5, self.data.bitmap().get(position::G5.as_pos_index())),
            61 => (position::G6, self.data.bitmap().get(position::G6.as_pos_index())),
            62 => (position::G7, self.data.bitmap().get(position::G7.as_pos_index())),
            63 => (position::G8, self.data.bitmap().get(position::G8.as_pos_index())),
            _ => return None,
        });
        self.index += 1;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::position::*;
    use rstest::rstest;
    use crate::state::game_state::GameState;

    #[test]
    fn print_match_ljumps() {
        let mut game_state = GameState::empty();
        for (pos, maybe_piece) in game_state.board.as_iter() {
            let mut bit_board = BitBoard::new();
            bit_board.fill_l_jump_from_pos(pos);
            println!("{pos} => 0b{:064b},", bit_board.0.as_value());
        }
    }

    #[test]
    fn print_match_diagonals() {
        let mut game_state = GameState::empty();
        for (pos, maybe_piece) in game_state.board.as_iter() {
            let mut bit_board = BitBoard::new();
            bit_board.fill_diag_from_pos(pos);
            println!("{pos} => 0b{:064b},", bit_board.0.as_value());
        }
    }

    #[test]
    fn print_match_straights() {
        let mut game_state = GameState::empty();
        for (pos, maybe_piece) in game_state.board.as_iter() {
            let mut bit_board = BitBoard::new();
            bit_board.fill_plus_from_pos(pos);
            println!("{pos} => 0b{:064b},", bit_board.0.as_value());
        }
    }

    #[test]
    fn diagonals() {
        println!("-7: \n{}", BitBoard::from_value(FULL_DIAG_LEFT >> 8 * 7).as_multiline_str()); // a1
        println!("-6: \n{}", BitBoard::from_value(FULL_DIAG_LEFT >> 8 * 6).as_multiline_str()); // a2 - b1
        println!("-5: \n{}", BitBoard::from_value(FULL_DIAG_LEFT >> 8 * 5).as_multiline_str()); // a3 - c1
        println!("-4: \n{}", BitBoard::from_value(FULL_DIAG_LEFT >> 8 * 4).as_multiline_str()); // a4 - d1
        println!("-3: \n{}", BitBoard::from_value(FULL_DIAG_LEFT >> 8 * 3).as_multiline_str()); // a5 - e1
        println!("-2: \n{}", BitBoard::from_value(FULL_DIAG_LEFT >> 8 * 2).as_multiline_str()); // a6 - f1
        println!("-1: \n{}", BitBoard::from_value(FULL_DIAG_LEFT >> 8 * 1).as_multiline_str()); // a7 - g1
        println!("0: \n{}",  BitBoard::from_value(FULL_DIAG_LEFT).as_multiline_str());          // h1 - a8
        println!("1: \n{}",  BitBoard::from_value(FULL_DIAG_LEFT << 8 * 1).as_multiline_str()); // h2 - b8
        println!("2: \n{}",  BitBoard::from_value(FULL_DIAG_LEFT << 8 * 2).as_multiline_str()); // h3 - c8
        println!("3: \n{}",  BitBoard::from_value(FULL_DIAG_LEFT << 8 * 3).as_multiline_str()); // h4 - d8
        println!("4: \n{}",  BitBoard::from_value(FULL_DIAG_LEFT << 8 * 4).as_multiline_str()); // h5 - e8
        println!("5: \n{}",  BitBoard::from_value(FULL_DIAG_LEFT << 8 * 5).as_multiline_str()); // h6 - f8
        println!("6: \n{}",  BitBoard::from_value(FULL_DIAG_LEFT << 8 * 6).as_multiline_str()); // h7 - g8
        println!("7: \n{}",  BitBoard::from_value(FULL_DIAG_LEFT << 8 * 7).as_multiline_str()); // h8

        println!("--");

        println!("-7: \n{}", BitBoard::from_value(FULL_DIAG_RIGHT >> 8 * 7).as_multiline_str()); // h1
        println!("-6: \n{}", BitBoard::from_value(FULL_DIAG_RIGHT >> 8 * 6).as_multiline_str()); // h2 - g1
        println!("-5: \n{}", BitBoard::from_value(FULL_DIAG_RIGHT >> 8 * 5).as_multiline_str()); // h3 - f1
        println!("-4: \n{}", BitBoard::from_value(FULL_DIAG_RIGHT >> 8 * 4).as_multiline_str()); // h4 - e1
        println!("-3: \n{}", BitBoard::from_value(FULL_DIAG_RIGHT >> 8 * 3).as_multiline_str()); // h5 - d1
        println!("-2: \n{}", BitBoard::from_value(FULL_DIAG_RIGHT >> 8 * 2).as_multiline_str()); // h6 - c1
        println!("-1: \n{}", BitBoard::from_value(FULL_DIAG_RIGHT >> 8 * 1).as_multiline_str()); // h7 - b1
        println!("0: \n{}",  BitBoard::from_value(FULL_DIAG_RIGHT).as_multiline_str());          // a1 - h8
        println!("1: \n{}",  BitBoard::from_value(FULL_DIAG_RIGHT << 8 * 1).as_multiline_str()); // a2 - g8
        println!("2: \n{}",  BitBoard::from_value(FULL_DIAG_RIGHT << 8 * 2).as_multiline_str()); // a3 - f8
        println!("3: \n{}",  BitBoard::from_value(FULL_DIAG_RIGHT << 8 * 3).as_multiline_str()); // a4 - e8
        println!("4: \n{}",  BitBoard::from_value(FULL_DIAG_RIGHT << 8 * 4).as_multiline_str()); // a5 - d8
        println!("5: \n{}",  BitBoard::from_value(FULL_DIAG_RIGHT << 8 * 5).as_multiline_str()); // a6 - c8
        println!("6: \n{}",  BitBoard::from_value(FULL_DIAG_RIGHT << 8 * 6).as_multiline_str()); // a7 - b8
        println!("7: \n{}",  BitBoard::from_value(FULL_DIAG_RIGHT << 8 * 7).as_multiline_str()); // a8
    }

    #[rstest]
    #[case(
        A1,
        " ┌ABCDEFGH|0\n\
         1|        |8\n\
         2| #      |16\n\
         3|  #     |24\n\
         4|   #    |32\n\
         5|    #   |40\n\
         6|     #  |48\n\
         7|      # |56\n\
         8|       #|64"
    )]
    #[case(
        B2,
        " ┌ABCDEFGH|0\n\
         1|# #     |8\n\
         2|        |16\n\
         3|# #     |24\n\
         4|   #    |32\n\
         5|    #   |40\n\
         6|     #  |48\n\
         7|      # |56\n\
         8|       #|64"
    )]
    #[case(
        E4,
        " ┌ABCDEFGH|0\n\
         1| #     #|8\n\
         2|  #   # |16\n\
         3|   # #  |24\n\
         4|        |32\n\
         5|   # #  |40\n\
         6|  #   # |48\n\
         7| #     #|56\n\
         8|#       |64"
    )]
    #[case(
        A8,
        " ┌ABCDEFGH|0\n\
         1|       #|8\n\
         2|      # |16\n\
         3|     #  |24\n\
         4|    #   |32\n\
         5|   #    |40\n\
         6|  #     |48\n\
         7| #      |56\n\
         8|        |64"
    )]
    #[case(
        H8,
        " ┌ABCDEFGH|0\n\
         1|#       |8\n\
         2| #      |16\n\
         3|  #     |24\n\
         4|   #    |32\n\
         5|    #   |40\n\
         6|     #  |48\n\
         7|      # |56\n\
         8|        |64"
    )]
    fn fill_diag_from_pos(
        #[case] pos: BoardPosition,
        #[case] expected: &'static str,
    ) {
        assert_eq!(
            expected,
            BitBoard::default()
                .fill_diag_from_pos(pos)
                .as_multiline_str()
        )
    }

    #[rstest]
    #[case(
        BoardFile::A,
        " ┌ABCDEFGH|0\n\
         1|#       |8\n\
         2|#       |16\n\
         3|#       |24\n\
         4|#       |32\n\
         5|#       |40\n\
         6|#       |48\n\
         7|#       |56\n\
         8|#       |64"
    )]
    #[case(
        BoardFile::E,
        " ┌ABCDEFGH|0\n\
         1|    #   |8\n\
         2|    #   |16\n\
         3|    #   |24\n\
         4|    #   |32\n\
         5|    #   |40\n\
         6|    #   |48\n\
         7|    #   |56\n\
         8|    #   |64"
    )]
    fn fill_file(
        #[case] file: BoardFile,
        #[case] expected: &'static str,
    ) {
        assert_eq!(
            expected,
            BitBoard::default().fill_file(file).as_multiline_str()
        )
    }

    #[rstest]
    #[case(
        BoardRank::One,
        " ┌ABCDEFGH|0\n\
         1|########|8\n\
         2|        |16\n\
         3|        |24\n\
         4|        |32\n\
         5|        |40\n\
         6|        |48\n\
         7|        |56\n\
         8|        |64"
    )]
    #[case(
        BoardRank::Five,
        " ┌ABCDEFGH|0\n\
         1|        |8\n\
         2|        |16\n\
         3|        |24\n\
         4|        |32\n\
         5|########|40\n\
         6|        |48\n\
         7|        |56\n\
         8|        |64"
    )]
    fn fill_rank(
        #[case] rank: BoardRank,
        #[case] expected: &'static str,
    ) {
        assert_eq!(
            expected,
            BitBoard::default().fill_rank(rank).as_multiline_str()
        )
    }

    #[rstest]
    #[case(
        A1,
        " ┌ABCDEFGH|0\n\
         1| #######|8\n\
         2|#       |16\n\
         3|#       |24\n\
         4|#       |32\n\
         5|#       |40\n\
         6|#       |48\n\
         7|#       |56\n\
         8|#       |64"
    )]
    #[case(
        E5,
        " ┌ABCDEFGH|0\n\
         1|    #   |8\n\
         2|    #   |16\n\
         3|    #   |24\n\
         4|    #   |32\n\
         5|#### ###|40\n\
         6|    #   |48\n\
         7|    #   |56\n\
         8|    #   |64"
    )]
    fn fill_plus_from_pos(
        #[case] pos: BoardPosition,
        #[case] expected: &'static str,
    ) {
        assert_eq!(
            expected,
            BitBoard::default()
                .fill_plus_from_pos(pos)
                .as_multiline_str()
        )
    }

    #[rstest]
    #[case(
        A1,
        " ┌ABCDEFGH|0\n\
         1| #      |8\n\
         2|##      |16\n\
         3|        |24\n\
         4|        |32\n\
         5|        |40\n\
         6|        |48\n\
         7|        |56\n\
         8|        |64"
    )]
    #[case(
        B2,
        " ┌ABCDEFGH|0\n\
         1|###     |8\n\
         2|# #     |16\n\
         3|###     |24\n\
         4|        |32\n\
         5|        |40\n\
         6|        |48\n\
         7|        |56\n\
         8|        |64"
    )]
    #[case(
        G7,
        " ┌ABCDEFGH|0\n\
         1|        |8\n\
         2|        |16\n\
         3|        |24\n\
         4|        |32\n\
         5|        |40\n\
         6|     ###|48\n\
         7|     # #|56\n\
         8|     ###|64"
    )]
    #[case(
        H8,
        " ┌ABCDEFGH|0\n\
         1|        |8\n\
         2|        |16\n\
         3|        |24\n\
         4|        |32\n\
         5|        |40\n\
         6|        |48\n\
         7|      ##|56\n\
         8|      # |64"
    )]
    #[case(
        H1,
        " ┌ABCDEFGH|0\n\
         1|      # |8\n\
         2|      ##|16\n\
         3|        |24\n\
         4|        |32\n\
         5|        |40\n\
         6|        |48\n\
         7|        |56\n\
         8|        |64"
    )]
    fn fill_3x3_from_pos(
        #[case] pos: BoardPosition,
        #[case] expected: &'static str,
    ) {
        assert_eq!(
            expected,
            BitBoard::default()
                .fill_3x3_from_pos(pos)
                .as_multiline_str()
        )
    }

    #[rstest]
    #[case(
        A1,
        " ┌ABCDEFGH|0\n\
         1|        |8\n\
         2|  #     |16\n\
         3| #      |24\n\
         4|        |32\n\
         5|        |40\n\
         6|        |48\n\
         7|        |56\n\
         8|        |64"
    )]
    #[case(
        E5,
        " ┌ABCDEFGH|0\n\
         1|        |8\n\
         2|        |16\n\
         3|   # #  |24\n\
         4|  #   # |32\n\
         5|        |40\n\
         6|  #   # |48\n\
         7|   # #  |56\n\
         8|        |64"
    )]
    fn fill_l_jump_from_pos(
        #[case] pos: BoardPosition,
        #[case] expected: &'static str,
    ) {
        assert_eq!(
            expected,
            BitBoard::default()
                .fill_l_jump_from_pos(pos)
                .as_multiline_str()
        )
    }

    #[rstest]
    #[case(
        A1,
        Direction::North,
        " ┌ABCDEFGH|0\n\
         1|        |8\n\
         2|#       |16\n\
         3|        |24\n\
         4|        |32\n\
         5|        |40\n\
         6|        |48\n\
         7|        |56\n\
         8|        |64"
    )]
    #[case(
        E5,
        Direction::North,
        " ┌ABCDEFGH|0\n\
         1|        |8\n\
         2|        |16\n\
         3|        |24\n\
         4|        |32\n\
         5|        |40\n\
         6|    #   |48\n\
         7|        |56\n\
         8|        |64"
    )]
    fn fill_single_from_pos(
        #[case] pos: BoardPosition,
        #[case] direction: Direction,
        #[case] expected: &'static str,
    ) {
        assert_eq!(
            expected,
            BitBoard::default()
                .fill_single_from_pos(pos, direction)
                .as_multiline_str()
        )
    }

    #[rstest]
    #[case(
        A1,
        Direction::North,
        " ┌ABCDEFGH|0\n\
         1|        |8\n\
         2|#       |16\n\
         3|#       |24\n\
         4|        |32\n\
         5|        |40\n\
         6|        |48\n\
         7|        |56\n\
         8|        |64"
    )]
    #[case(
        E5,
        Direction::North,
        " ┌ABCDEFGH|0\n\
         1|        |8\n\
         2|        |16\n\
         3|        |24\n\
         4|        |32\n\
         5|        |40\n\
         6|    #   |48\n\
         7|    #   |56\n\
         8|        |64"
    )]
    #[case(
        A8,
        Direction::North,
        " ┌ABCDEFGH|0\n\
         1|        |8\n\
         2|        |16\n\
         3|        |24\n\
         4|        |32\n\
         5|        |40\n\
         6|        |48\n\
         7|        |56\n\
         8|        |64"
    )]
    fn fill_double_from_pos(
        #[case] pos: BoardPosition,
        #[case] direction: Direction,
        #[case] expected: &'static str,
    ) {
        assert_eq!(
            expected,
            BitBoard::default()
                .fill_double_from_pos(pos, direction)
                .as_multiline_str()
        )
    }

    #[rstest]
    #[case(
        A1,
        Direction::North,
        " ┌ABCDEFGH|0\n\
         1|        |8\n\
         2| #      |16\n\
         3|        |24\n\
         4|        |32\n\
         5|        |40\n\
         6|        |48\n\
         7|        |56\n\
         8|        |64"
    )]
    #[case(
        E5,
        Direction::North,
        " ┌ABCDEFGH|0\n\
         1|        |8\n\
         2|        |16\n\
         3|        |24\n\
         4|        |32\n\
         5|        |40\n\
         6|   # #  |48\n\
         7|        |56\n\
         8|        |64"
    )]
    #[case(
        A8,
        Direction::North,
        " ┌ABCDEFGH|0\n\
         1|        |8\n\
         2|        |16\n\
         3|        |24\n\
         4|        |32\n\
         5|        |40\n\
         6|        |48\n\
         7|        |56\n\
         8|        |64"
    )]
    fn fill_single_diags_from_pos(
        #[case] pos: BoardPosition,
        #[case] direction: Direction,
        #[case] expected: &'static str,
    ) {
        assert_eq!(
            expected,
            BitBoard::default()
                .fill_single_diags_from_pos(pos, direction)
                .as_multiline_str()
        )
    }

    #[rstest]
    #[case(
        E5,
        " ┌ABCDEFGH|0\n\
         1|#   #   |8\n\
         2| #  #  #|16\n\
         3|  # # # |24\n\
         4|   ###  |32\n\
         5|#### ###|40\n\
         6|   ###  |48\n\
         7|  # # # |56\n\
         8| #  #  #|64"
    )]
    fn full_cross(
        #[case] pos: BoardPosition,
        #[case] expected: &'static str,
    ) {
        let BoardPosition(file, rank) = pos;
        let b = BitBoard::default()
            .fill_rank(rank)
            .fill_file(file)
            .fill_diag_from_pos(pos)
            .as_multiline_str();
        assert_eq!(expected, b)
    }
}
