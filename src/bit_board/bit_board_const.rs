use crate::bit_board::bit_board::BitBoard;
use crate::board::board_position::BoardPosition;
use crate::board::position;
use crate::utils::custom_struct_iterator::CustomStructIterator;

pub type BitBoardConstData = u64;
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BitBoardConst {
    data: BitBoardConstData,
}

#[inline]
const fn set(mut bitboard: BitBoardConst, index: usize, value: bool) -> BitBoardConst {
    let mask = 1 << index;
    let _prev = bitboard.data & mask;
    if value {
        bitboard.data |= mask;
    } else {
        bitboard.data &= !mask;
    }
    bitboard
}

#[inline]
pub const fn set_pos(
    bitboard: BitBoardConst,
    board_position: BoardPosition,
    value: bool,
) -> BitBoardConst {
    set(bitboard, board_position.as_pos_index(), value)
}

impl BitBoardConst {
    pub fn as_bitboard(&self) -> BitBoard {
        BitBoard::from_value(self.data)
    }
    pub const fn data(&self) -> BitBoardConstData {
        self.data
    }
    pub const fn new() -> Self {
        Self { data: 0 }
    }
    pub const fn from(data: BitBoardConstData) -> Self {
        Self { data }
    }
    #[inline]
    pub const fn get_pos(&self, board_position: BoardPosition) -> bool {
        self.get(board_position.as_pos_index())
    }
    #[inline]
    pub const fn set_pos(self, board_position: BoardPosition, value: bool) -> Self {
        set(self, board_position.as_pos_index(), value)
    }
    #[inline]
    pub const fn get(&self, index: usize) -> bool {
        self.data & (1 << index) != 0
    }

    #[inline]
    pub const fn set(self, index: usize, value: bool) -> Self {
        set(self, index, value)
    }

    pub const fn as_iter(&self) -> CustomStructIterator<BitBoardConst> {
        CustomStructIterator {
            data: self,
            index: 0,
        }
    }
}

impl Iterator for CustomStructIterator<'_, BitBoardConst> {
    type Item = (BoardPosition, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let res = Some(match self.index {
            0 => (position::A1, self.data.get(0)),
            1 => (position::A2, self.data.get(1)),
            2 => (position::A3, self.data.get(2)),
            3 => (position::A4, self.data.get(3)),
            4 => (position::A5, self.data.get(4)),
            5 => (position::A6, self.data.get(5)),
            6 => (position::A7, self.data.get(6)),
            7 => (position::A8, self.data.get(7)),
            8 => (position::B1, self.data.get(8)),
            9 => (position::B2, self.data.get(9)),
            10 => (position::B3, self.data.get(10)),
            11 => (position::B4, self.data.get(11)),
            12 => (position::B5, self.data.get(12)),
            13 => (position::B6, self.data.get(13)),
            14 => (position::B7, self.data.get(14)),
            15 => (position::B8, self.data.get(15)),
            16 => (position::C1, self.data.get(16)),
            17 => (position::C2, self.data.get(17)),
            18 => (position::C3, self.data.get(18)),
            19 => (position::C4, self.data.get(19)),
            20 => (position::C5, self.data.get(20)),
            21 => (position::C6, self.data.get(21)),
            22 => (position::C7, self.data.get(22)),
            23 => (position::C8, self.data.get(23)),
            24 => (position::D1, self.data.get(24)),
            25 => (position::D2, self.data.get(25)),
            26 => (position::D3, self.data.get(26)),
            27 => (position::D4, self.data.get(27)),
            28 => (position::D5, self.data.get(28)),
            29 => (position::D6, self.data.get(29)),
            30 => (position::D7, self.data.get(30)),
            31 => (position::D8, self.data.get(31)),
            32 => (position::E1, self.data.get(32)),
            33 => (position::E2, self.data.get(33)),
            34 => (position::E3, self.data.get(34)),
            35 => (position::E4, self.data.get(35)),
            36 => (position::E5, self.data.get(36)),
            37 => (position::E6, self.data.get(37)),
            38 => (position::E7, self.data.get(38)),
            39 => (position::E8, self.data.get(39)),
            40 => (position::F1, self.data.get(40)),
            41 => (position::F2, self.data.get(41)),
            42 => (position::F3, self.data.get(42)),
            43 => (position::F4, self.data.get(43)),
            44 => (position::F5, self.data.get(44)),
            45 => (position::F6, self.data.get(45)),
            46 => (position::F7, self.data.get(46)),
            47 => (position::F8, self.data.get(47)),
            48 => (position::H1, self.data.get(48)),
            49 => (position::H2, self.data.get(49)),
            50 => (position::H3, self.data.get(50)),
            51 => (position::H4, self.data.get(51)),
            52 => (position::H5, self.data.get(52)),
            53 => (position::H6, self.data.get(53)),
            54 => (position::H7, self.data.get(54)),
            55 => (position::H8, self.data.get(55)),
            56 => (position::G1, self.data.get(56)),
            57 => (position::G2, self.data.get(57)),
            58 => (position::G3, self.data.get(58)),
            59 => (position::G4, self.data.get(59)),
            60 => (position::G5, self.data.get(60)),
            61 => (position::G6, self.data.get(61)),
            62 => (position::G7, self.data.get(62)),
            63 => (position::G8, self.data.get(63)),
            _ => return None,
        });
        self.index += 1;
        res
    }
}
