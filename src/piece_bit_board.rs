use crate::bit_board::BitBoard;
use crate::piece::Piece;

pub struct PieceBitBoard {
    pub piece: Piece,
    pub board: BitBoard,
}

impl PieceBitBoard {
    pub const fn piece(&self) -> &Piece {
        &self.piece
    }
    pub const fn bit_board(&self) -> &BitBoard {
        &self.board
    }
    pub const fn from(piece: Piece, bit_board: BitBoard) -> Self {
        PieceBitBoard {
            piece,
            board: bit_board,
        }
    }
}
