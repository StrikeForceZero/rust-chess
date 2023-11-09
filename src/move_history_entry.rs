use crate::board_position::BoardPosition;
use crate::chess_piece::ChessPiece;

#[derive(Clone)]
pub struct MoveHistoryEntry {
    pub piece: ChessPiece,
    pub from: BoardPosition,
    pub to: BoardPosition,
    pub capture: Option<ChessPiece>,
}
