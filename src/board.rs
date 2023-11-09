use crate::bit_board::{BitBoard, BitBoardData};
use crate::board_position::BoardPosition;
use crate::chess_piece::ChessPiece;
use crate::full_color_piece_bit_board::FullColorPieceBitBoard;
use crate::full_piece_bit_board::FullPieceBitBoard;
use crate::piece::Piece;
use crate::position;
use crate::utils::CustomStructIterator;

#[derive(Default, Clone)]
pub struct Board {
    a1: Option<ChessPiece>,
    a2: Option<ChessPiece>,
    a3: Option<ChessPiece>,
    a4: Option<ChessPiece>,
    a5: Option<ChessPiece>,
    a6: Option<ChessPiece>,
    a7: Option<ChessPiece>,
    a8: Option<ChessPiece>,

    b1: Option<ChessPiece>,
    b2: Option<ChessPiece>,
    b3: Option<ChessPiece>,
    b4: Option<ChessPiece>,
    b5: Option<ChessPiece>,
    b6: Option<ChessPiece>,
    b7: Option<ChessPiece>,
    b8: Option<ChessPiece>,

    c1: Option<ChessPiece>,
    c2: Option<ChessPiece>,
    c3: Option<ChessPiece>,
    c4: Option<ChessPiece>,
    c5: Option<ChessPiece>,
    c6: Option<ChessPiece>,
    c7: Option<ChessPiece>,
    c8: Option<ChessPiece>,

    d1: Option<ChessPiece>,
    d2: Option<ChessPiece>,
    d3: Option<ChessPiece>,
    d4: Option<ChessPiece>,
    d5: Option<ChessPiece>,
    d6: Option<ChessPiece>,
    d7: Option<ChessPiece>,
    d8: Option<ChessPiece>,

    e1: Option<ChessPiece>,
    e2: Option<ChessPiece>,
    e3: Option<ChessPiece>,
    e4: Option<ChessPiece>,
    e5: Option<ChessPiece>,
    e6: Option<ChessPiece>,
    e7: Option<ChessPiece>,
    e8: Option<ChessPiece>,

    f1: Option<ChessPiece>,
    f2: Option<ChessPiece>,
    f3: Option<ChessPiece>,
    f4: Option<ChessPiece>,
    f5: Option<ChessPiece>,
    f6: Option<ChessPiece>,
    f7: Option<ChessPiece>,
    f8: Option<ChessPiece>,

    h1: Option<ChessPiece>,
    h2: Option<ChessPiece>,
    h3: Option<ChessPiece>,
    h4: Option<ChessPiece>,
    h5: Option<ChessPiece>,
    h6: Option<ChessPiece>,
    h7: Option<ChessPiece>,
    h8: Option<ChessPiece>,

    g1: Option<ChessPiece>,
    g2: Option<ChessPiece>,
    g3: Option<ChessPiece>,
    g4: Option<ChessPiece>,
    g5: Option<ChessPiece>,
    g6: Option<ChessPiece>,
    g7: Option<ChessPiece>,
    g8: Option<ChessPiece>,
}

impl Board {
    pub const fn new() -> Board {
        Board {
            a1: None,
            a2: None,
            a3: None,
            a4: None,
            a5: None,
            a6: None,
            a7: None,
            a8: None,
            b1: None,
            b2: None,
            b3: None,
            b4: None,
            b5: None,
            b6: None,
            b7: None,
            b8: None,
            c1: None,
            c2: None,
            c3: None,
            c4: None,
            c5: None,
            c6: None,
            c7: None,
            c8: None,
            d1: None,
            d2: None,
            d3: None,
            d4: None,
            d5: None,
            d6: None,
            d7: None,
            d8: None,
            e1: None,
            e2: None,
            e3: None,
            e4: None,
            e5: None,
            e6: None,
            e7: None,
            e8: None,
            f1: None,
            f2: None,
            f3: None,
            f4: None,
            f5: None,
            f6: None,
            f7: None,
            f8: None,
            h1: None,
            h2: None,
            h3: None,
            h4: None,
            h5: None,
            h6: None,
            h7: None,
            h8: None,
            g1: None,
            g2: None,
            g3: None,
            g4: None,
            g5: None,
            g6: None,
            g7: None,
            g8: None,
        }
    }
    pub const fn all(&self) -> [&Option<ChessPiece>; 64] {
        [
            &self.a1, &self.b1, &self.c1, &self.d1, &self.e1, &self.f1, &self.g1, &self.h1,
            &self.a2, &self.b2, &self.c2, &self.d2, &self.e2, &self.f2, &self.g2, &self.h2,
            &self.a3, &self.b3, &self.c3, &self.d3, &self.e3, &self.f3, &self.g3, &self.h3,
            &self.a4, &self.b4, &self.c4, &self.d4, &self.e4, &self.f4, &self.g4, &self.h4,
            &self.a5, &self.b5, &self.c5, &self.d5, &self.e5, &self.f5, &self.g5, &self.h5,
            &self.a6, &self.b6, &self.c6, &self.d6, &self.e6, &self.f6, &self.g6, &self.h6,
            &self.a7, &self.b7, &self.c7, &self.d7, &self.e7, &self.f7, &self.g7, &self.h7,
            &self.a8, &self.b8, &self.c8, &self.d8, &self.e8, &self.f8, &self.g8, &self.h8,
        ]
    }
    pub const fn as_slice(&self) -> [[&Option<ChessPiece>; 8]; 8] {
        [
            [&self.a1, &self.b1, &self.c1, &self.d1, &self.e1, &self.f1, &self.g1, &self.h1],
            [&self.a2, &self.b2, &self.c2, &self.d2, &self.e2, &self.f2, &self.g2, &self.h2],
            [&self.a3, &self.b3, &self.c3, &self.d3, &self.e3, &self.f3, &self.g3, &self.h3],
            [&self.a4, &self.b4, &self.c4, &self.d4, &self.e4, &self.f4, &self.g4, &self.h4],
            [&self.a5, &self.b5, &self.c5, &self.d5, &self.e5, &self.f5, &self.g5, &self.h5],
            [&self.a6, &self.b6, &self.c6, &self.d6, &self.e6, &self.f6, &self.g6, &self.h6],
            [&self.a7, &self.b7, &self.c7, &self.d7, &self.e7, &self.f7, &self.g7, &self.h7],
            [&self.a8, &self.b8, &self.c8, &self.d8, &self.e8, &self.f8, &self.g8, &self.h8],
        ]
    }
    pub const fn get(&self, board_position: BoardPosition) -> &Option<ChessPiece> {
        match board_position {
            position::A1 => &self.a1,
            position::A2 => &self.a2,
            position::A3 => &self.a3,
            position::A4 => &self.a4,
            position::A5 => &self.a5,
            position::A6 => &self.a6,
            position::A7 => &self.a7,
            position::A8 => &self.a8,
            position::B1 => &self.b1,
            position::B2 => &self.b2,
            position::B3 => &self.b3,
            position::B4 => &self.b4,
            position::B5 => &self.b5,
            position::B6 => &self.b6,
            position::B7 => &self.b7,
            position::B8 => &self.b8,
            position::C1 => &self.c1,
            position::C2 => &self.c2,
            position::C3 => &self.c3,
            position::C4 => &self.c4,
            position::C5 => &self.c5,
            position::C6 => &self.c6,
            position::C7 => &self.c7,
            position::C8 => &self.c8,
            position::D1 => &self.d1,
            position::D2 => &self.d2,
            position::D3 => &self.d3,
            position::D4 => &self.d4,
            position::D5 => &self.d5,
            position::D6 => &self.d6,
            position::D7 => &self.d7,
            position::D8 => &self.d8,
            position::E1 => &self.e1,
            position::E2 => &self.e2,
            position::E3 => &self.e3,
            position::E4 => &self.e4,
            position::E5 => &self.e5,
            position::E6 => &self.e6,
            position::E7 => &self.e7,
            position::E8 => &self.e8,
            position::F1 => &self.f1,
            position::F2 => &self.f2,
            position::F3 => &self.f3,
            position::F4 => &self.f4,
            position::F5 => &self.f5,
            position::F6 => &self.f6,
            position::F7 => &self.f7,
            position::F8 => &self.f8,
            position::H1 => &self.h1,
            position::H2 => &self.h2,
            position::H3 => &self.h3,
            position::H4 => &self.h4,
            position::H5 => &self.h5,
            position::H6 => &self.h6,
            position::H7 => &self.h7,
            position::H8 => &self.h8,
            position::G1 => &self.g1,
            position::G2 => &self.g2,
            position::G3 => &self.g3,
            position::G4 => &self.g4,
            position::G5 => &self.g5,
            position::G6 => &self.g6,
            position::G7 => &self.g7,
            position::G8 => &self.g8,
        }
    }
    pub const fn get_mut(&mut self, board_position: BoardPosition) -> &mut Option<ChessPiece> {
        match board_position {
            position::A1 => &mut self.a1,
            position::A2 => &mut self.a2,
            position::A3 => &mut self.a3,
            position::A4 => &mut self.a4,
            position::A5 => &mut self.a5,
            position::A6 => &mut self.a6,
            position::A7 => &mut self.a7,
            position::A8 => &mut self.a8,
            position::B1 => &mut self.b1,
            position::B2 => &mut self.b2,
            position::B3 => &mut self.b3,
            position::B4 => &mut self.b4,
            position::B5 => &mut self.b5,
            position::B6 => &mut self.b6,
            position::B7 => &mut self.b7,
            position::B8 => &mut self.b8,
            position::C1 => &mut self.c1,
            position::C2 => &mut self.c2,
            position::C3 => &mut self.c3,
            position::C4 => &mut self.c4,
            position::C5 => &mut self.c5,
            position::C6 => &mut self.c6,
            position::C7 => &mut self.c7,
            position::C8 => &mut self.c8,
            position::D1 => &mut self.d1,
            position::D2 => &mut self.d2,
            position::D3 => &mut self.d3,
            position::D4 => &mut self.d4,
            position::D5 => &mut self.d5,
            position::D6 => &mut self.d6,
            position::D7 => &mut self.d7,
            position::D8 => &mut self.d8,
            position::E1 => &mut self.e1,
            position::E2 => &mut self.e2,
            position::E3 => &mut self.e3,
            position::E4 => &mut self.e4,
            position::E5 => &mut self.e5,
            position::E6 => &mut self.e6,
            position::E7 => &mut self.e7,
            position::E8 => &mut self.e8,
            position::F1 => &mut self.f1,
            position::F2 => &mut self.f2,
            position::F3 => &mut self.f3,
            position::F4 => &mut self.f4,
            position::F5 => &mut self.f5,
            position::F6 => &mut self.f6,
            position::F7 => &mut self.f7,
            position::F8 => &mut self.f8,
            position::H1 => &mut self.h1,
            position::H2 => &mut self.h2,
            position::H3 => &mut self.h3,
            position::H4 => &mut self.h4,
            position::H5 => &mut self.h5,
            position::H6 => &mut self.h6,
            position::H7 => &mut self.h7,
            position::H8 => &mut self.h8,
            position::G1 => &mut self.g1,
            position::G2 => &mut self.g2,
            position::G3 => &mut self.g3,
            position::G4 => &mut self.g4,
            position::G5 => &mut self.g5,
            position::G6 => &mut self.g6,
            position::G7 => &mut self.g7,
            position::G8 => &mut self.g8,
        }
    }
    pub const fn as_iter(&self) -> CustomStructIterator<Board> {
        CustomStructIterator::from(self)
    }
    pub const fn set(&mut self, board_position: BoardPosition, chess_piece: Option<ChessPiece>) {
        *self.get_mut(board_position) = chess_piece;
    }
    pub const fn replace(&mut self, board_position: BoardPosition, chess_piece: Option<ChessPiece>) -> Option<ChessPiece> {
        let &removed_piece = self.get(board_position).clone();
        self.set(board_position, chess_piece);
        removed_piece
    }
    pub const fn from_bit_boards() {

    }

    pub const fn as_bit_boards(&self) -> FullColorPieceBitBoard {
        let mut white_king: BitBoard = BitBoard::from(BitBoardData::new());
        let mut white_queen: BitBoard = BitBoard::from(BitBoardData::new());
        let mut white_rook: BitBoard = BitBoard::from(BitBoardData::new());
        let mut white_bishop: BitBoard = BitBoard::from(BitBoardData::new());
        let mut white_knight: BitBoard = BitBoard::from(BitBoardData::new());
        let mut white_pawn: BitBoard = BitBoard::from(BitBoardData::new());
        let mut black_king: BitBoard = BitBoard::from(BitBoardData::new());
        let mut black_queen: BitBoard = BitBoard::from(BitBoardData::new());
        let mut black_rook: BitBoard = BitBoard::from(BitBoardData::new());
        let mut black_bishop: BitBoard = BitBoard::from(BitBoardData::new());
        let mut black_knight: BitBoard = BitBoard::from(BitBoardData::new());
        let mut black_pawn: BitBoard = BitBoard::from(BitBoardData::new());

        for (pos, maybePiece) in self.as_iter() {
            let Some(piece) = maybePiece else { continue };
            match piece {
                ChessPiece::WhiteKing => white_king.set_pos(pos, true),
                ChessPiece::WhiteQueen => white_queen.set_pos(pos, true),
                ChessPiece::WhiteRook => white_rook.set_pos(pos, true),
                ChessPiece::WhiteBishop => white_bishop.set_pos(pos, true),
                ChessPiece::WhiteKnight => white_knight.set_pos(pos, true),
                ChessPiece::WhitePawn => white_pawn.set_pos(pos, true),
                ChessPiece::BlackKing => black_king.set_pos(pos, true),
                ChessPiece::BlackQueen => black_queen.set_pos(pos, true),
                ChessPiece::BlackRook => black_rook.set_pos(pos, true),
                ChessPiece::BlackBishop => black_bishop.set_pos(pos, true),
                ChessPiece::BlackKnight => black_knight.set_pos(pos, true),
                ChessPiece::BlackPawn => black_pawn.set_pos(pos, true),
            }
        }

        FullColorPieceBitBoard {
            white: FullPieceBitBoard {
                king: white_king,
                queen: white_queen,
                rook: white_rook,
                bishop: white_bishop,
                knight: white_knight,
                pawn: white_pawn,
            },
            black: FullPieceBitBoard {
                king: black_king,
                queen: black_queen,
                rook: black_rook,
                bishop: black_bishop,
                knight: black_knight,
                pawn: black_pawn,
            }
        }
    }
}

impl Iterator for CustomStructIterator<Board> {
    type Item = (BoardPosition, Option<ChessPiece>);

    fn next(&mut self) -> Option<Self::Item> {
        let res = Some(Self::Item(match self.index {
            0 => (position::A1, &self.data.a1),
            1 => (position::A2, &self.data.a2),
            2 => (position::A3, &self.data.a3),
            3 => (position::A4, &self.data.a4),
            4 => (position::A5, &self.data.a5),
            5 => (position::A6, &self.data.a6),
            6 => (position::A7, &self.data.a7),
            7 => (position::A8, &self.data.a8),
            8 => (position::B1, &self.data.b1),
            9 => (position::B2, &self.data.b2),
            10 => (position::B3, &self.data.b3),
            11 => (position::B4, &self.data.b4),
            12 => (position::B5, &self.data.b5),
            13 => (position::B6, &self.data.b6),
            14 => (position::B7, &self.data.b7),
            15 => (position::B8, &self.data.b8),
            16 => (position::C1, &self.data.c1),
            17 => (position::C2, &self.data.c2),
            18 => (position::C3, &self.data.c3),
            19 => (position::C4, &self.data.c4),
            20 => (position::C5, &self.data.c5),
            21 => (position::C6, &self.data.c6),
            22 => (position::C7, &self.data.c7),
            23 => (position::C8, &self.data.c8),
            24 => (position::D1, &self.data.d1),
            25 => (position::D2, &self.data.d2),
            26 => (position::D3, &self.data.d3),
            27 => (position::D4, &self.data.d4),
            28 => (position::D5, &self.data.d5),
            29 => (position::D6, &self.data.d6),
            30 => (position::D7, &self.data.d7),
            31 => (position::D8, &self.data.d8),
            32 => (position::E1, &self.data.e1),
            33 => (position::E2, &self.data.e2),
            34 => (position::E3, &self.data.e3),
            35 => (position::E4, &self.data.e4),
            36 => (position::E5, &self.data.e5),
            37 => (position::E6, &self.data.e6),
            38 => (position::E7, &self.data.e7),
            39 => (position::E8, &self.data.e8),
            40 => (position::F1, &self.data.f1),
            41 => (position::F2, &self.data.f2),
            42 => (position::F3, &self.data.f3),
            43 => (position::F4, &self.data.f4),
            44 => (position::F5, &self.data.f5),
            45 => (position::F6, &self.data.f6),
            46 => (position::F7, &self.data.f7),
            47 => (position::F8, &self.data.f8),
            48 => (position::H1, &self.data.h1),
            49 => (position::H2, &self.data.h2),
            50 => (position::H3, &self.data.h3),
            51 => (position::H4, &self.data.h4),
            52 => (position::H5, &self.data.h5),
            53 => (position::H6, &self.data.h6),
            54 => (position::H7, &self.data.h7),
            55 => (position::H8, &self.data.h8),
            56 => (position::G1, &self.data.g1),
            57 => (position::G2, &self.data.g2),
            58 => (position::G3, &self.data.g3),
            59 => (position::G4, &self.data.g4),
            60 => (position::G5, &self.data.g5),
            61 => (position::G6, &self.data.g6),
            62 => (position::G7, &self.data.g7),
            63 => (position::G8, &self.data.g8),
            _ => return None,
        }));
        self.index += 1;
        res
    }
}
