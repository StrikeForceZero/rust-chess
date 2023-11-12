use crate::piece::piece::Piece;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PromotionPiece {
    Queen,
    Rook,
    Bishop,
    Knight,
}

impl PromotionPiece {
    pub const fn as_piece(&self) -> Piece {
        match self {
            Self::Queen => Piece::Queen,
            Self::Rook => Piece::Rook,
            Self::Bishop => Piece::Bishop,
            Self::Knight => Piece::Knight,
        }
    }
}
