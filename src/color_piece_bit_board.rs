use crate::color::Color;
use crate::piece::Piece;
use crate::piece_bit_board::PieceBitBoard;

pub struct ColorPieceBitBoard {
    pub color: Color,
    pub piece_bit_board: PieceBitBoard,
}

impl ColorPieceBitBoard {
    pub fn color(&self) -> &Color {
        &self.color
    }
    pub fn piece_bit_board(&self) -> &PieceBitBoard {
        &self.piece_bit_board
    }
    pub fn from(color: Color, piece_bit_board: PieceBitBoard) -> Self {
        ColorPieceBitBoard {
            color,
            piece_bit_board,
        }
    }
}
