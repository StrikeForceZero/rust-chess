use std::hash::{Hash, Hasher};
use crate::bit_board_const::BitBoardConst;
use crate::board_position::BoardPosition;
use crate::chess_piece::ChessPiece;
use crate::color::Color;
use crate::full_piece_bit_board::FullPieceBitBoard;
use crate::utils::{CustomStructIterator, CustomStructIteratorMut, hash_64bit_numbers};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FullColorPieceBitBoard {
    pub white: FullPieceBitBoard,
    pub black: FullPieceBitBoard,
}

impl Hash for FullColorPieceBitBoard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let hash = hash_64bit_numbers(&[
            self.white.pawn.data(),
            self.white.knight.data(),
            self.white.bishop.data(),
            self.white.rook.data(),
            self.white.queen.data(),
            self.white.king.data(),

            self.black.pawn.data(),
            self.black.knight.data(),
            self.black.bishop.data(),
            self.black.rook.data(),
            self.black.queen.data(),
            self.black.king.data(),
        ]);
        hash.hash(state)
    }
}

impl FullColorPieceBitBoard {
    pub const fn as_iter(&self) -> CustomStructIterator<Self> {
        CustomStructIterator {
            data: self,
            index: 0,
        }
    }

    pub const fn update(mut self, board_position: BoardPosition, maybe_chess_piece: &Option<ChessPiece>) -> Self {
        let Some(chess_piece) = maybe_chess_piece else {
            return self;
        };
        match chess_piece {
            ChessPiece::WhiteKing => self.white.king = BitBoardConst::set_pos(self.white.king, board_position, true),
            ChessPiece::WhiteQueen => self.white.queen = BitBoardConst::set_pos(self.white.queen, board_position, true),
            ChessPiece::WhiteRook => self.white.rook = BitBoardConst::set_pos(self.white.rook, board_position, true),
            ChessPiece::WhiteBishop => self.white.bishop = BitBoardConst::set_pos(self.white.bishop, board_position, true),
            ChessPiece::WhiteKnight => self.white.knight = BitBoardConst::set_pos(self.white.knight, board_position, true),
            ChessPiece::WhitePawn => self.white.pawn = BitBoardConst::set_pos(self.white.pawn, board_position, true),
            ChessPiece::BlackKing => self.black.king = BitBoardConst::set_pos(self.black.king, board_position, true),
            ChessPiece::BlackQueen => self.black.queen = BitBoardConst::set_pos(self.black.queen, board_position, true),
            ChessPiece::BlackRook => self.black.rook = BitBoardConst::set_pos(self.black.rook, board_position, true),
            ChessPiece::BlackBishop => self.black.bishop = BitBoardConst::set_pos(self.black.bishop, board_position, true),
            ChessPiece::BlackKnight => self.black.knight = BitBoardConst::set_pos(self.black.knight, board_position, true),
            ChessPiece::BlackPawn => self.black.pawn = BitBoardConst::set_pos(self.black.pawn, board_position, true),
        };
        self
    }
}

impl<'a> Iterator for CustomStructIterator<'a, FullColorPieceBitBoard> {
    type Item = (Color, &'a FullPieceBitBoard);

    fn next(&mut self) -> Option<Self::Item> {
        let res = Some(match self.index {
            0 => (Color::White, &self.data.white),
            1 => (Color::Black, &self.data.black),
            _ => return None,
        });
        self.index += 1;
        res
    }
}
