use crate::board_position::BoardPosition;
use crate::castle_side::CastleSide;
use crate::chess_piece::ChessPiece;

#[derive(Copy, Clone)]
pub enum MoveType {
    Normal,
    EnPassant(BoardPosition),
    Castle(CastleSide),
}

#[derive(Clone)]
pub struct Move {
    pub piece: ChessPiece,
    pub from: BoardPosition,
    pub to: BoardPosition,
    pub captured_piece: Option<ChessPiece>,
}
