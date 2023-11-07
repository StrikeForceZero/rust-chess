use crate::color::Color;
use crate::piece::Piece;
use crate::piece_bit_board::PieceBitBoard;

pub struct ColorPieceBitBoard(Color, PieceBitBoard);

impl ColorPieceBitBoard {
    pub fn color(&self) -> &Color {
        &self.0
    }
    pub fn piece_bit_board(&self) -> &PieceBitBoard {
        &self.1
    }
    pub fn from(color: Color, piece_bit_board: PieceBitBoard) -> Self {
        ColorPieceBitBoard(color, piece_bit_board)
    }
}
