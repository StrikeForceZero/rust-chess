use crate::bit_board::piece_bit_board::PieceBitBoard;
use crate::color::Color;

pub struct ColorPieceBitBoard {
    pub color: Color,
    pub piece_bit_board: PieceBitBoard,
}

impl ColorPieceBitBoard {
    pub const fn color(&self) -> &Color {
        &self.color
    }
    pub const fn piece_bit_board(&self) -> &PieceBitBoard {
        &self.piece_bit_board
    }
    pub const fn from(color: Color, piece_bit_board: PieceBitBoard) -> Self {
        ColorPieceBitBoard {
            color,
            piece_bit_board,
        }
    }
}
