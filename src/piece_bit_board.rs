use crate::bit_board::BitBoard;
use crate::piece::Piece;

pub struct PieceBitBoard(Piece, BitBoard);

impl PieceBitBoard {
    pub fn piece(&self) -> &Piece {
        &self.0
    }
    pub fn bit_board(&self) -> &BitBoard {
        &self.1
    }
    pub fn from(piece: Piece, bit_board: BitBoard) -> Self {
        PieceBitBoard(piece, bit_board)
    }
}
