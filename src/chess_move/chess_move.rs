use crate::board::board_position::BoardPosition;
use crate::direction::castle_side::CastleSide;
use crate::piece::chess_piece::ChessPiece;
use crate::piece::promotion_piece::PromotionPiece;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ChessMoveType {
    Normal,
    EnPassant(BoardPosition),
    Castle(CastleSide),
    Promotion(PromotionPiece),
}

#[derive(Clone, PartialEq, Debug)]
pub struct ChessMove {
    pub move_type: ChessMoveType,
    pub piece: ChessPiece,
    pub from: BoardPosition,
    pub to: BoardPosition,
    pub captured_piece: Option<ChessPiece>,
}

impl ChessMove {
    pub const fn create_normal(piece: ChessPiece, from: BoardPosition, to: BoardPosition) -> Self {
        Self {
            move_type: ChessMoveType::Normal,
            piece,
            from,
            to,
            captured_piece: None,
        }
    }
    pub const fn create_normal_capture(
        piece: ChessPiece,
        from: BoardPosition,
        to: BoardPosition,
        capture_piece: ChessPiece,
    ) -> Self {
        Self {
            move_type: ChessMoveType::Normal,
            piece,
            from,
            to,
            captured_piece: Some(capture_piece),
        }
    }
    pub const fn create_en_passant(
        piece: ChessPiece,
        from: BoardPosition,
        to: BoardPosition,
        capture_pos: BoardPosition,
        capture_piece: ChessPiece,
    ) -> Self {
        Self {
            move_type: ChessMoveType::EnPassant(capture_pos),
            piece,
            from,
            to,
            captured_piece: Some(capture_piece),
        }
    }
    pub const fn create_castle(
        piece: ChessPiece,
        from: BoardPosition,
        to: BoardPosition,
        castle_side: CastleSide,
    ) -> Self {
        Self {
            move_type: ChessMoveType::Castle(castle_side),
            piece,
            from,
            to,
            captured_piece: None,
        }
    }

    pub const fn create_promotion(
        piece: ChessPiece,
        from: BoardPosition,
        to: BoardPosition,
        promotion_piece: PromotionPiece,
    ) -> Self {
        Self {
            move_type: ChessMoveType::Promotion(promotion_piece),
            piece,
            from,
            to,
            captured_piece: None,
        }
    }
}

impl Display for ChessMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ move_type: {:?}, piece: {:?}, from: {}, to: {}, captured_piece: {:?} }}",
            self.move_type, self.piece, self.from, self.to, self.captured_piece
        )
    }
}
