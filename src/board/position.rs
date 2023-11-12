use crate::board::board_file::BoardFile;
use crate::board::board_position::BoardPosition;
use crate::board::board_rank::BoardRank;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Position {
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    H7,
    H8,
    G1,
    G2,
    G3,
    G4,
    G5,
    G6,
    G7,
    G8,
}

impl Position {
    pub const fn to_board_pos(&self) -> BoardPosition {
        match self {
            Self::A1 => A1,
            Self::A2 => A2,
            Self::A3 => A3,
            Self::A4 => A4,
            Self::A5 => A5,
            Self::A6 => A6,
            Self::A7 => A7,
            Self::A8 => A8,
            Self::B1 => B1,
            Self::B2 => B2,
            Self::B3 => B3,
            Self::B4 => B4,
            Self::B5 => B5,
            Self::B6 => B6,
            Self::B7 => B7,
            Self::B8 => B8,
            Self::C1 => C1,
            Self::C2 => C2,
            Self::C3 => C3,
            Self::C4 => C4,
            Self::C5 => C5,
            Self::C6 => C6,
            Self::C7 => C7,
            Self::C8 => C8,
            Self::D1 => D1,
            Self::D2 => D2,
            Self::D3 => D3,
            Self::D4 => D4,
            Self::D5 => D5,
            Self::D6 => D6,
            Self::D7 => D7,
            Self::D8 => D8,
            Self::E1 => E1,
            Self::E2 => E2,
            Self::E3 => E3,
            Self::E4 => E4,
            Self::E5 => E5,
            Self::E6 => E6,
            Self::E7 => E7,
            Self::E8 => E8,
            Self::F1 => F1,
            Self::F2 => F2,
            Self::F3 => F3,
            Self::F4 => F4,
            Self::F5 => F5,
            Self::F6 => F6,
            Self::F7 => F7,
            Self::F8 => F8,
            Self::H1 => H1,
            Self::H2 => H2,
            Self::H3 => H3,
            Self::H4 => H4,
            Self::H5 => H5,
            Self::H6 => H6,
            Self::H7 => H7,
            Self::H8 => H8,
            Self::G1 => G1,
            Self::G2 => G2,
            Self::G3 => G3,
            Self::G4 => G4,
            Self::G5 => G5,
            Self::G6 => G6,
            Self::G7 => G7,
            Self::G8 => G8,
        }
    }
    pub const fn from_board_pos(board_pos: BoardPosition) -> Position {
        match board_pos {
            A1 => Self::A1,
            A2 => Self::A2,
            A3 => Self::A3,
            A4 => Self::A4,
            A5 => Self::A5,
            A6 => Self::A6,
            A7 => Self::A7,
            A8 => Self::A8,
            B1 => Self::B1,
            B2 => Self::B2,
            B3 => Self::B3,
            B4 => Self::B4,
            B5 => Self::B5,
            B6 => Self::B6,
            B7 => Self::B7,
            B8 => Self::B8,
            C1 => Self::C1,
            C2 => Self::C2,
            C3 => Self::C3,
            C4 => Self::C4,
            C5 => Self::C5,
            C6 => Self::C6,
            C7 => Self::C7,
            C8 => Self::C8,
            D1 => Self::D1,
            D2 => Self::D2,
            D3 => Self::D3,
            D4 => Self::D4,
            D5 => Self::D5,
            D6 => Self::D6,
            D7 => Self::D7,
            D8 => Self::D8,
            E1 => Self::E1,
            E2 => Self::E2,
            E3 => Self::E3,
            E4 => Self::E4,
            E5 => Self::E5,
            E6 => Self::E6,
            E7 => Self::E7,
            E8 => Self::E8,
            F1 => Self::F1,
            F2 => Self::F2,
            F3 => Self::F3,
            F4 => Self::F4,
            F5 => Self::F5,
            F6 => Self::F6,
            F7 => Self::F7,
            F8 => Self::F8,
            H1 => Self::H1,
            H2 => Self::H2,
            H3 => Self::H3,
            H4 => Self::H4,
            H5 => Self::H5,
            H6 => Self::H6,
            H7 => Self::H7,
            H8 => Self::H8,
            G1 => Self::G1,
            G2 => Self::G2,
            G3 => Self::G3,
            G4 => Self::G4,
            G5 => Self::G5,
            G6 => Self::G6,
            G7 => Self::G7,
            G8 => Self::G8,
        }
    }
}

pub const A1: BoardPosition = BoardPosition(BoardFile::A, BoardRank::One);
pub const A2: BoardPosition = BoardPosition(BoardFile::A, BoardRank::Two);
pub const A3: BoardPosition = BoardPosition(BoardFile::A, BoardRank::Three);
pub const A4: BoardPosition = BoardPosition(BoardFile::A, BoardRank::Four);
pub const A5: BoardPosition = BoardPosition(BoardFile::A, BoardRank::Five);
pub const A6: BoardPosition = BoardPosition(BoardFile::A, BoardRank::Six);
pub const A7: BoardPosition = BoardPosition(BoardFile::A, BoardRank::Seven);
pub const A8: BoardPosition = BoardPosition(BoardFile::A, BoardRank::Eight);
pub const B1: BoardPosition = BoardPosition(BoardFile::B, BoardRank::One);
pub const B2: BoardPosition = BoardPosition(BoardFile::B, BoardRank::Two);
pub const B3: BoardPosition = BoardPosition(BoardFile::B, BoardRank::Three);
pub const B4: BoardPosition = BoardPosition(BoardFile::B, BoardRank::Four);
pub const B5: BoardPosition = BoardPosition(BoardFile::B, BoardRank::Five);
pub const B6: BoardPosition = BoardPosition(BoardFile::B, BoardRank::Six);
pub const B7: BoardPosition = BoardPosition(BoardFile::B, BoardRank::Seven);
pub const B8: BoardPosition = BoardPosition(BoardFile::B, BoardRank::Eight);
pub const C1: BoardPosition = BoardPosition(BoardFile::C, BoardRank::One);
pub const C2: BoardPosition = BoardPosition(BoardFile::C, BoardRank::Two);
pub const C3: BoardPosition = BoardPosition(BoardFile::C, BoardRank::Three);
pub const C4: BoardPosition = BoardPosition(BoardFile::C, BoardRank::Four);
pub const C5: BoardPosition = BoardPosition(BoardFile::C, BoardRank::Five);
pub const C6: BoardPosition = BoardPosition(BoardFile::C, BoardRank::Six);
pub const C7: BoardPosition = BoardPosition(BoardFile::C, BoardRank::Seven);
pub const C8: BoardPosition = BoardPosition(BoardFile::C, BoardRank::Eight);
pub const D1: BoardPosition = BoardPosition(BoardFile::D, BoardRank::One);
pub const D2: BoardPosition = BoardPosition(BoardFile::D, BoardRank::Two);
pub const D3: BoardPosition = BoardPosition(BoardFile::D, BoardRank::Three);
pub const D4: BoardPosition = BoardPosition(BoardFile::D, BoardRank::Four);
pub const D5: BoardPosition = BoardPosition(BoardFile::D, BoardRank::Five);
pub const D6: BoardPosition = BoardPosition(BoardFile::D, BoardRank::Six);
pub const D7: BoardPosition = BoardPosition(BoardFile::D, BoardRank::Seven);
pub const D8: BoardPosition = BoardPosition(BoardFile::D, BoardRank::Eight);
pub const E1: BoardPosition = BoardPosition(BoardFile::E, BoardRank::One);
pub const E2: BoardPosition = BoardPosition(BoardFile::E, BoardRank::Two);
pub const E3: BoardPosition = BoardPosition(BoardFile::E, BoardRank::Three);
pub const E4: BoardPosition = BoardPosition(BoardFile::E, BoardRank::Four);
pub const E5: BoardPosition = BoardPosition(BoardFile::E, BoardRank::Five);
pub const E6: BoardPosition = BoardPosition(BoardFile::E, BoardRank::Six);
pub const E7: BoardPosition = BoardPosition(BoardFile::E, BoardRank::Seven);
pub const E8: BoardPosition = BoardPosition(BoardFile::E, BoardRank::Eight);
pub const F1: BoardPosition = BoardPosition(BoardFile::F, BoardRank::One);
pub const F2: BoardPosition = BoardPosition(BoardFile::F, BoardRank::Two);
pub const F3: BoardPosition = BoardPosition(BoardFile::F, BoardRank::Three);
pub const F4: BoardPosition = BoardPosition(BoardFile::F, BoardRank::Four);
pub const F5: BoardPosition = BoardPosition(BoardFile::F, BoardRank::Five);
pub const F6: BoardPosition = BoardPosition(BoardFile::F, BoardRank::Six);
pub const F7: BoardPosition = BoardPosition(BoardFile::F, BoardRank::Seven);
pub const F8: BoardPosition = BoardPosition(BoardFile::F, BoardRank::Eight);
pub const H1: BoardPosition = BoardPosition(BoardFile::H, BoardRank::One);
pub const H2: BoardPosition = BoardPosition(BoardFile::H, BoardRank::Two);
pub const H3: BoardPosition = BoardPosition(BoardFile::H, BoardRank::Three);
pub const H4: BoardPosition = BoardPosition(BoardFile::H, BoardRank::Four);
pub const H5: BoardPosition = BoardPosition(BoardFile::H, BoardRank::Five);
pub const H6: BoardPosition = BoardPosition(BoardFile::H, BoardRank::Six);
pub const H7: BoardPosition = BoardPosition(BoardFile::H, BoardRank::Seven);
pub const H8: BoardPosition = BoardPosition(BoardFile::H, BoardRank::Eight);
pub const G1: BoardPosition = BoardPosition(BoardFile::G, BoardRank::One);
pub const G2: BoardPosition = BoardPosition(BoardFile::G, BoardRank::Two);
pub const G3: BoardPosition = BoardPosition(BoardFile::G, BoardRank::Three);
pub const G4: BoardPosition = BoardPosition(BoardFile::G, BoardRank::Four);
pub const G5: BoardPosition = BoardPosition(BoardFile::G, BoardRank::Five);
pub const G6: BoardPosition = BoardPosition(BoardFile::G, BoardRank::Six);
pub const G7: BoardPosition = BoardPosition(BoardFile::G, BoardRank::Seven);
pub const G8: BoardPosition = BoardPosition(BoardFile::G, BoardRank::Eight);

pub const WHITE_QUEEN_SIDE_ROOK_SQUARE: BoardPosition = A1;
pub const WHITE_QUEEN_SIDE_KNIGHT_SQUARE: BoardPosition = B1;
pub const WHITE_QUEEN_SIDE_BISHOP_SQUARE: BoardPosition = C1;
pub const WHITE_QUEEN_SQUARE: BoardPosition = D1;
pub const WHITE_KING_SQUARE: BoardPosition = E1;
pub const WHITE_KING_SIDE_BISHOP_SQUARE: BoardPosition = F1;
pub const WHITE_KING_SIDE_KNIGHT_SQUARE: BoardPosition = G1;
pub const WHITE_KING_SIDE_ROOK_SQUARE: BoardPosition = H1;

pub const WHITE_QUEEN_SIDE_KING_CASTLE_SQUARE: BoardPosition = WHITE_QUEEN_SIDE_BISHOP_SQUARE;
pub const WHITE_QUEEN_SIDE_ROOK_CASTLE_SQUARE: BoardPosition = WHITE_QUEEN_SQUARE;
pub const WHITE_KING_SIDE_KING_CASTLE_SQUARE: BoardPosition = WHITE_KING_SIDE_KNIGHT_SQUARE;
pub const WHITE_KING_SIDE_ROOK_CASTLE_SQUARE: BoardPosition = WHITE_KING_SIDE_BISHOP_SQUARE;

pub const BLACK_QUEEN_SIDE_ROOK_SQUARE: BoardPosition = A8;
pub const BLACK_QUEEN_SIDE_KNIGHT_SQUARE: BoardPosition = B8;
pub const BLACK_QUEEN_SIDE_BISHOP_SQUARE: BoardPosition = C8;
pub const BLACK_QUEEN_SQUARE: BoardPosition = D8;
pub const BLACK_KING_SQUARE: BoardPosition = E8;
pub const BLACK_KING_SIDE_BISHOP_SQUARE: BoardPosition = F8;
pub const BLACK_KING_SIDE_KNIGHT_SQUARE: BoardPosition = G8;
pub const BLACK_KING_SIDE_ROOK_SQUARE: BoardPosition = H8;

pub const BLACK_QUEEN_SIDE_KING_CASTLE_SQUARE: BoardPosition = BLACK_QUEEN_SIDE_BISHOP_SQUARE;
pub const BLACK_QUEEN_SIDE_ROOK_CASTLE_SQUARE: BoardPosition = BLACK_QUEEN_SQUARE;
pub const BLACK_KING_SIDE_KING_CASTLE_SQUARE: BoardPosition = BLACK_KING_SIDE_KNIGHT_SQUARE;
pub const BLACK_KING_SIDE_ROOK_CASTLE_SQUARE: BoardPosition = BLACK_KING_SIDE_BISHOP_SQUARE;
