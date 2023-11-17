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
#[derive(Default, Clone, Debug)]
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
            0 => (position::A1, self.data.bitmap().get(0)),
            1 => (position::A2, self.data.bitmap().get(1)),
            2 => (position::A3, self.data.bitmap().get(2)),
            3 => (position::A4, self.data.bitmap().get(3)),
            4 => (position::A5, self.data.bitmap().get(4)),
            5 => (position::A6, self.data.bitmap().get(5)),
            6 => (position::A7, self.data.bitmap().get(6)),
            7 => (position::A8, self.data.bitmap().get(7)),
            8 => (position::B1, self.data.bitmap().get(8)),
            9 => (position::B2, self.data.bitmap().get(9)),
            10 => (position::B3, self.data.bitmap().get(10)),
            11 => (position::B4, self.data.bitmap().get(11)),
            12 => (position::B5, self.data.bitmap().get(12)),
            13 => (position::B6, self.data.bitmap().get(13)),
            14 => (position::B7, self.data.bitmap().get(14)),
            15 => (position::B8, self.data.bitmap().get(15)),
            16 => (position::C1, self.data.bitmap().get(16)),
            17 => (position::C2, self.data.bitmap().get(17)),
            18 => (position::C3, self.data.bitmap().get(18)),
            19 => (position::C4, self.data.bitmap().get(19)),
            20 => (position::C5, self.data.bitmap().get(20)),
            21 => (position::C6, self.data.bitmap().get(21)),
            22 => (position::C7, self.data.bitmap().get(22)),
            23 => (position::C8, self.data.bitmap().get(23)),
            24 => (position::D1, self.data.bitmap().get(24)),
            25 => (position::D2, self.data.bitmap().get(25)),
            26 => (position::D3, self.data.bitmap().get(26)),
            27 => (position::D4, self.data.bitmap().get(27)),
            28 => (position::D5, self.data.bitmap().get(28)),
            29 => (position::D6, self.data.bitmap().get(29)),
            30 => (position::D7, self.data.bitmap().get(30)),
            31 => (position::D8, self.data.bitmap().get(31)),
            32 => (position::E1, self.data.bitmap().get(32)),
            33 => (position::E2, self.data.bitmap().get(33)),
            34 => (position::E3, self.data.bitmap().get(34)),
            35 => (position::E4, self.data.bitmap().get(35)),
            36 => (position::E5, self.data.bitmap().get(36)),
            37 => (position::E6, self.data.bitmap().get(37)),
            38 => (position::E7, self.data.bitmap().get(38)),
            39 => (position::E8, self.data.bitmap().get(39)),
            40 => (position::F1, self.data.bitmap().get(40)),
            41 => (position::F2, self.data.bitmap().get(41)),
            42 => (position::F3, self.data.bitmap().get(42)),
            43 => (position::F4, self.data.bitmap().get(43)),
            44 => (position::F5, self.data.bitmap().get(44)),
            45 => (position::F6, self.data.bitmap().get(45)),
            46 => (position::F7, self.data.bitmap().get(46)),
            47 => (position::F8, self.data.bitmap().get(47)),
            48 => (position::H1, self.data.bitmap().get(48)),
            49 => (position::H2, self.data.bitmap().get(49)),
            50 => (position::H3, self.data.bitmap().get(50)),
            51 => (position::H4, self.data.bitmap().get(51)),
            52 => (position::H5, self.data.bitmap().get(52)),
            53 => (position::H6, self.data.bitmap().get(53)),
            54 => (position::H7, self.data.bitmap().get(54)),
            55 => (position::H8, self.data.bitmap().get(55)),
            56 => (position::G1, self.data.bitmap().get(56)),
            57 => (position::G2, self.data.bitmap().get(57)),
            58 => (position::G3, self.data.bitmap().get(58)),
            59 => (position::G4, self.data.bitmap().get(59)),
            60 => (position::G5, self.data.bitmap().get(60)),
            61 => (position::G6, self.data.bitmap().get(61)),
            62 => (position::G7, self.data.bitmap().get(62)),
            63 => (position::G8, self.data.bitmap().get(63)),
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
