use crate::bit_board::BitBoard;
use crate::piece::Piece;

pub struct PieceBitBoard {
    piece: Piece,
    board: BitBoard,
}

pub struct FullPieceBitBoard {
    pawn: BitBoard,
    knight: BitBoard,
    bishop: BitBoard,
    rook: BitBoard,
    queen: BitBoard,
    king: BitBoard,
}

impl PieceBitBoard {
    pub fn piece(&self) -> &Piece {
        &self.piece
    }
    pub fn bit_board(&self) -> &BitBoard {
        &self.board
    }
    pub fn from(piece: Piece, bit_board: BitBoard) -> Self {
        PieceBitBoard {
            piece,
            board: bit_board,
        }
    }
}
