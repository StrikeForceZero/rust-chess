use crate::board::board_position::BoardPosition;
use crate::chess_move::chess_move::{ChessMove, ChessMoveType};
use crate::piece::chess_piece::ChessPiece;

#[derive(Clone, Debug)]
pub struct MoveHistoryEntry {
    pub move_type: ChessMoveType,
    pub piece: ChessPiece,
    pub from: BoardPosition,
    pub to: BoardPosition,
    pub capture: Option<ChessPiece>,
}

impl MoveHistoryEntry {
    pub const fn from_move(some_move: &ChessMove) -> Self {
        Self {
            piece: some_move.piece,
            from: some_move.from,
            to: some_move.to,
            capture: some_move.captured_piece,
            move_type: some_move.move_type,
        }
    }
}
