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
    pub move_type: MoveType,
    pub piece: ChessPiece,
    pub from: BoardPosition,
    pub to: BoardPosition,
    pub captured_piece: Option<ChessPiece>,
}

impl Move {
    pub const fn create_normal(piece: ChessPiece, from: BoardPosition, to: BoardPosition) -> Self {
        Self {
            move_type: MoveType::Normal,
            piece,
            from,
            to,
            captured_piece: None,
        }
    }
    pub const fn create_normal_capture(piece: ChessPiece, from: BoardPosition, to: BoardPosition, capture_piece: ChessPiece) -> Self {
        Self {
            move_type: MoveType::Normal,
            piece,
            from,
            to,
            captured_piece: Some(capture_piece),
        }
    }
    pub const fn create_en_passant(piece: ChessPiece, from: BoardPosition, to: BoardPosition, capture_pos: BoardPosition, capture_piece: ChessPiece) -> Self {
        Self {
            move_type: MoveType::EnPassant(capture_pos),
            piece,
            from,
            to,
            captured_piece: Some(capture_piece),
        }
    }
    pub const fn create_castle(piece: ChessPiece, from: BoardPosition, to: BoardPosition, castle_side: CastleSide) -> Self {
        Self {
            move_type: MoveType::Castle(castle_side),
            piece,
            from,
            to,
            captured_piece: None,
        }
    }
}
