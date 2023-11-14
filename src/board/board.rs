use crate::bit_board::bit_board_const::BitBoardConst;
use crate::bit_board::full_color_piece_bit_board::FullColorPieceBitBoard;
use crate::bit_board::full_piece_bit_board::FullPieceBitBoard;
use crate::board::board_position::BoardPosition;
use crate::board::position;
use crate::piece::chess_piece::ChessPiece;
use crate::piece::chess_piece::ChessPiece::{
    BlackBishop, BlackKing, BlackKnight, BlackPawn, BlackQueen, BlackRook, WhiteBishop, WhiteKing,
    WhiteKnight, WhitePawn, WhiteQueen, WhiteRook,
};
use crate::utils::custom_struct_iterator::{CustomStructIterator, CustomStructIteratorMut};

const STARTING_BOARD: Board = Board::new();

#[derive(Default, Clone, Debug)]
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

    g1: Option<ChessPiece>,
    g2: Option<ChessPiece>,
    g3: Option<ChessPiece>,
    g4: Option<ChessPiece>,
    g5: Option<ChessPiece>,
    g6: Option<ChessPiece>,
    g7: Option<ChessPiece>,
    g8: Option<ChessPiece>,

    h1: Option<ChessPiece>,
    h2: Option<ChessPiece>,
    h3: Option<ChessPiece>,
    h4: Option<ChessPiece>,
    h5: Option<ChessPiece>,
    h6: Option<ChessPiece>,
    h7: Option<ChessPiece>,
    h8: Option<ChessPiece>,
}

impl Board {
    pub const fn empty() -> Board {
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
            g1: None,
            g2: None,
            g3: None,
            g4: None,
            g5: None,
            g6: None,
            g7: None,
            g8: None,
            h1: None,
            h2: None,
            h3: None,
            h4: None,
            h5: None,
            h6: None,
            h7: None,
            h8: None,
        }
    }
    pub const fn new() -> Board {
        Board {
            a1: Some(WhiteRook),
            b1: Some(WhiteKnight),
            c1: Some(WhiteBishop),
            d1: Some(WhiteQueen),
            e1: Some(WhiteKing),
            f1: Some(WhiteBishop),
            h1: Some(WhiteRook),
            g1: Some(WhiteKnight),

            a2: Some(WhitePawn),
            b2: Some(WhitePawn),
            c2: Some(WhitePawn),
            d2: Some(WhitePawn),
            e2: Some(WhitePawn),
            f2: Some(WhitePawn),
            g2: Some(WhitePawn),
            h2: Some(WhitePawn),

            a3: None,
            b3: None,
            c3: None,
            d3: None,
            e3: None,
            f3: None,
            g3: None,
            h3: None,

            a4: None,
            b4: None,
            c4: None,
            d4: None,
            e4: None,
            f4: None,
            g4: None,
            h4: None,

            a5: None,
            b5: None,
            c5: None,
            e5: None,
            d5: None,
            f5: None,
            g5: None,
            h5: None,

            a6: None,
            b6: None,
            c6: None,
            d6: None,
            e6: None,
            f6: None,
            g6: None,
            h6: None,

            a7: Some(BlackPawn),
            b7: Some(BlackPawn),
            c7: Some(BlackPawn),
            d7: Some(BlackPawn),
            e7: Some(BlackPawn),
            f7: Some(BlackPawn),
            g7: Some(BlackPawn),
            h7: Some(BlackPawn),

            a8: Some(BlackRook),
            b8: Some(BlackKnight),
            c8: Some(BlackBishop),
            d8: Some(BlackQueen),
            e8: Some(BlackKing),
            f8: Some(BlackBishop),
            g8: Some(BlackKnight),
            h8: Some(BlackRook),
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
            position::G1 => &self.g1,
            position::G2 => &self.g2,
            position::G3 => &self.g3,
            position::G4 => &self.g4,
            position::G5 => &self.g5,
            position::G6 => &self.g6,
            position::G7 => &self.g7,
            position::G8 => &self.g8,
            position::H1 => &self.h1,
            position::H2 => &self.h2,
            position::H3 => &self.h3,
            position::H4 => &self.h4,
            position::H5 => &self.h5,
            position::H6 => &self.h6,
            position::H7 => &self.h7,
            position::H8 => &self.h8,
        }
    }
    pub fn get_mut(&mut self, board_position: BoardPosition) -> &mut Option<ChessPiece> {
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
            position::G1 => &mut self.g1,
            position::G2 => &mut self.g2,
            position::G3 => &mut self.g3,
            position::G4 => &mut self.g4,
            position::G5 => &mut self.g5,
            position::G6 => &mut self.g6,
            position::G7 => &mut self.g7,
            position::G8 => &mut self.g8,
            position::H1 => &mut self.h1,
            position::H2 => &mut self.h2,
            position::H3 => &mut self.h3,
            position::H4 => &mut self.h4,
            position::H5 => &mut self.h5,
            position::H6 => &mut self.h6,
            position::H7 => &mut self.h7,
            position::H8 => &mut self.h8,
        }
    }
    pub const fn as_iter(&self) -> CustomStructIterator<Board> {
        CustomStructIterator::from(self)
    }
    pub fn set(&mut self, board_position: BoardPosition, chess_piece: Option<ChessPiece>) {
        self.replace(board_position, chess_piece);
    }
    pub fn replace(
        &mut self,
        board_position: BoardPosition,
        chess_piece_option: Option<ChessPiece>,
    ) -> Option<ChessPiece> {
        if let Some(chess_piece) = chess_piece_option {
            self.get_mut(board_position).replace(chess_piece)
        } else {
            self.get_mut(board_position).take()
        }
    }
    pub const fn is_pos_starting_pos(&self, board_position: BoardPosition) -> bool {
        match (self.get(board_position), STARTING_BOARD.get(board_position)) {
            (None, None) => true,
            (Some(WhiteKing), Some(WhiteKing)) => true,
            (Some(WhiteQueen), Some(WhiteQueen)) => true,
            (Some(WhiteRook), Some(WhiteRook)) => true,
            (Some(WhiteBishop), Some(WhiteBishop)) => true,
            (Some(WhiteKnight), Some(WhiteKnight)) => true,
            (Some(WhitePawn), Some(WhitePawn)) => true,
            (Some(BlackKing), Some(BlackKing)) => true,
            (Some(BlackQueen), Some(BlackQueen)) => true,
            (Some(BlackRook), Some(BlackRook)) => true,
            (Some(BlackBishop), Some(BlackBishop)) => true,
            (Some(BlackKnight), Some(BlackKnight)) => true,
            (Some(BlackPawn), Some(BlackPawn)) => true,
            _ => false,
        }
    }
    pub fn from_bit_boards(full_color_piece_bit_board: FullColorPieceBitBoard) -> Self {
        let mut board = Board::empty();
        for (color, piece_bit_board) in full_color_piece_bit_board.as_iter() {
            for (piece, bit_board) in piece_bit_board.as_iter() {
                for (pos, value) in bit_board.as_iter() {
                    if value {
                        board.set(pos, Some(ChessPiece::from(color, piece)));
                    }
                }
            }
        }
        board
    }

    pub fn as_bit_boards(&self) -> FullColorPieceBitBoard {
        let mut white_king = BitBoardConst::new();
        let mut white_queen = BitBoardConst::new();
        let mut white_rook = BitBoardConst::new();
        let mut white_bishop = BitBoardConst::new();
        let mut white_knight = BitBoardConst::new();
        let mut white_pawn = BitBoardConst::new();
        let mut black_king = BitBoardConst::new();
        let mut black_queen = BitBoardConst::new();
        let mut black_rook = BitBoardConst::new();
        let mut black_bishop = BitBoardConst::new();
        let mut black_knight = BitBoardConst::new();
        let mut black_pawn = BitBoardConst::new();

        for (pos, maybe_piece) in self.as_iter() {
            let Some(piece) = maybe_piece else { continue };
            match piece {
                WhiteKing => white_king = BitBoardConst::set_pos(white_king, pos, true),
                WhiteQueen => white_queen = BitBoardConst::set_pos(white_queen, pos, true),
                WhiteRook => white_rook = BitBoardConst::set_pos(white_rook, pos, true),
                WhiteBishop => white_bishop = BitBoardConst::set_pos(white_bishop, pos, true),
                WhiteKnight => white_knight = BitBoardConst::set_pos(white_knight, pos, true),
                WhitePawn => white_pawn = BitBoardConst::set_pos(white_pawn, pos, true),
                BlackKing => black_king = BitBoardConst::set_pos(black_king, pos, true),
                BlackQueen => black_queen = BitBoardConst::set_pos(black_queen, pos, true),
                BlackRook => black_rook = BitBoardConst::set_pos(black_rook, pos, true),
                BlackBishop => black_bishop = BitBoardConst::set_pos(black_bishop, pos, true),
                BlackKnight => black_knight = BitBoardConst::set_pos(black_knight, pos, true),
                BlackPawn => black_pawn = BitBoardConst::set_pos(black_pawn, pos, true),
            };
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
            },
        }
    }
    pub const fn as_bit_boards_const(&self) -> FullColorPieceBitBoard {
        let mut fcpbb = FullColorPieceBitBoard {
            white: FullPieceBitBoard {
                king: BitBoardConst::new(),
                queen: BitBoardConst::new(),
                rook: BitBoardConst::new(),
                bishop: BitBoardConst::new(),
                knight: BitBoardConst::new(),
                pawn: BitBoardConst::new(),
            },
            black: FullPieceBitBoard {
                king: BitBoardConst::new(),
                queen: BitBoardConst::new(),
                rook: BitBoardConst::new(),
                bishop: BitBoardConst::new(),
                knight: BitBoardConst::new(),
                pawn: BitBoardConst::new(),
            },
        };

        fcpbb = fcpbb.update(position::A1, &self.a1);
        fcpbb = fcpbb.update(position::A2, &self.a2);
        fcpbb = fcpbb.update(position::A3, &self.a3);
        fcpbb = fcpbb.update(position::A4, &self.a4);
        fcpbb = fcpbb.update(position::A5, &self.a5);
        fcpbb = fcpbb.update(position::A6, &self.a6);
        fcpbb = fcpbb.update(position::A7, &self.a7);
        fcpbb = fcpbb.update(position::A8, &self.a8);
        fcpbb = fcpbb.update(position::B1, &self.b1);
        fcpbb = fcpbb.update(position::B2, &self.b2);
        fcpbb = fcpbb.update(position::B3, &self.b3);
        fcpbb = fcpbb.update(position::B4, &self.b4);
        fcpbb = fcpbb.update(position::B5, &self.b5);
        fcpbb = fcpbb.update(position::B6, &self.b6);
        fcpbb = fcpbb.update(position::B7, &self.b7);
        fcpbb = fcpbb.update(position::B8, &self.b8);
        fcpbb = fcpbb.update(position::C1, &self.c1);
        fcpbb = fcpbb.update(position::C2, &self.c2);
        fcpbb = fcpbb.update(position::C3, &self.c3);
        fcpbb = fcpbb.update(position::C4, &self.c4);
        fcpbb = fcpbb.update(position::C5, &self.c5);
        fcpbb = fcpbb.update(position::C6, &self.c6);
        fcpbb = fcpbb.update(position::C7, &self.c7);
        fcpbb = fcpbb.update(position::C8, &self.c8);
        fcpbb = fcpbb.update(position::D1, &self.d1);
        fcpbb = fcpbb.update(position::D2, &self.d2);
        fcpbb = fcpbb.update(position::D3, &self.d3);
        fcpbb = fcpbb.update(position::D4, &self.d4);
        fcpbb = fcpbb.update(position::D5, &self.d5);
        fcpbb = fcpbb.update(position::D6, &self.d6);
        fcpbb = fcpbb.update(position::D7, &self.d7);
        fcpbb = fcpbb.update(position::D8, &self.d8);
        fcpbb = fcpbb.update(position::E1, &self.e1);
        fcpbb = fcpbb.update(position::E2, &self.e2);
        fcpbb = fcpbb.update(position::E3, &self.e3);
        fcpbb = fcpbb.update(position::E4, &self.e4);
        fcpbb = fcpbb.update(position::E5, &self.e5);
        fcpbb = fcpbb.update(position::E6, &self.e6);
        fcpbb = fcpbb.update(position::E7, &self.e7);
        fcpbb = fcpbb.update(position::E8, &self.e8);
        fcpbb = fcpbb.update(position::F1, &self.f1);
        fcpbb = fcpbb.update(position::F2, &self.f2);
        fcpbb = fcpbb.update(position::F3, &self.f3);
        fcpbb = fcpbb.update(position::F4, &self.f4);
        fcpbb = fcpbb.update(position::F5, &self.f5);
        fcpbb = fcpbb.update(position::F6, &self.f6);
        fcpbb = fcpbb.update(position::F7, &self.f7);
        fcpbb = fcpbb.update(position::F8, &self.f8);
        fcpbb = fcpbb.update(position::G1, &self.g1);
        fcpbb = fcpbb.update(position::G2, &self.g2);
        fcpbb = fcpbb.update(position::G3, &self.g3);
        fcpbb = fcpbb.update(position::G4, &self.g4);
        fcpbb = fcpbb.update(position::G5, &self.g5);
        fcpbb = fcpbb.update(position::G6, &self.g6);
        fcpbb = fcpbb.update(position::G7, &self.g7);
        fcpbb = fcpbb.update(position::G8, &self.g8);
        fcpbb = fcpbb.update(position::H1, &self.h1);
        fcpbb = fcpbb.update(position::H2, &self.h2);
        fcpbb = fcpbb.update(position::H3, &self.h3);
        fcpbb = fcpbb.update(position::H4, &self.h4);
        fcpbb = fcpbb.update(position::H5, &self.h5);
        fcpbb = fcpbb.update(position::H6, &self.h6);
        fcpbb = fcpbb.update(position::H7, &self.h7);
        fcpbb = fcpbb.update(position::H8, &self.h8);

        fcpbb
    }
}

impl Iterator for CustomStructIterator<'_, Board> {
    type Item = (BoardPosition, Option<ChessPiece>);

    fn next(&mut self) -> Option<Self::Item> {
        let res = Some(match self.index {
            0 => (position::A1, self.data.a1),
            1 => (position::A2, self.data.a2),
            2 => (position::A3, self.data.a3),
            3 => (position::A4, self.data.a4),
            4 => (position::A5, self.data.a5),
            5 => (position::A6, self.data.a6),
            6 => (position::A7, self.data.a7),
            7 => (position::A8, self.data.a8),
            8 => (position::B1, self.data.b1),
            9 => (position::B2, self.data.b2),
            10 => (position::B3, self.data.b3),
            11 => (position::B4, self.data.b4),
            12 => (position::B5, self.data.b5),
            13 => (position::B6, self.data.b6),
            14 => (position::B7, self.data.b7),
            15 => (position::B8, self.data.b8),
            16 => (position::C1, self.data.c1),
            17 => (position::C2, self.data.c2),
            18 => (position::C3, self.data.c3),
            19 => (position::C4, self.data.c4),
            20 => (position::C5, self.data.c5),
            21 => (position::C6, self.data.c6),
            22 => (position::C7, self.data.c7),
            23 => (position::C8, self.data.c8),
            24 => (position::D1, self.data.d1),
            25 => (position::D2, self.data.d2),
            26 => (position::D3, self.data.d3),
            27 => (position::D4, self.data.d4),
            28 => (position::D5, self.data.d5),
            29 => (position::D6, self.data.d6),
            30 => (position::D7, self.data.d7),
            31 => (position::D8, self.data.d8),
            32 => (position::E1, self.data.e1),
            33 => (position::E2, self.data.e2),
            34 => (position::E3, self.data.e3),
            35 => (position::E4, self.data.e4),
            36 => (position::E5, self.data.e5),
            37 => (position::E6, self.data.e6),
            38 => (position::E7, self.data.e7),
            39 => (position::E8, self.data.e8),
            40 => (position::F1, self.data.f1),
            41 => (position::F2, self.data.f2),
            42 => (position::F3, self.data.f3),
            43 => (position::F4, self.data.f4),
            44 => (position::F5, self.data.f5),
            45 => (position::F6, self.data.f6),
            46 => (position::F7, self.data.f7),
            47 => (position::F8, self.data.f8),
            48 => (position::G1, self.data.g1),
            49 => (position::G2, self.data.g2),
            50 => (position::G3, self.data.g3),
            51 => (position::G4, self.data.g4),
            52 => (position::G5, self.data.g5),
            53 => (position::G6, self.data.g6),
            54 => (position::G7, self.data.g7),
            55 => (position::G8, self.data.g8),
            56 => (position::H1, self.data.h1),
            57 => (position::H2, self.data.h2),
            58 => (position::H3, self.data.h3),
            59 => (position::H4, self.data.h4),
            60 => (position::H5, self.data.h5),
            61 => (position::H6, self.data.h6),
            62 => (position::H7, self.data.h7),
            63 => (position::H8, self.data.h8),
            _ => return None,
        });
        self.index += 1;
        res
    }
}
