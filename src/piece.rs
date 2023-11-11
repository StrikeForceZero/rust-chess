use thiserror::Error;
use crate::chess_piece::ChessPiece;
use crate::color::Color;

#[derive(Error, Debug, Clone, Copy)]
pub enum PieceError {
    #[error("Invalid character for Piece: {0}")]
    InvalidChar(char),
}


#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    pub const fn as_char(&self) -> char {
        match self {
            Self::Pawn => 'P',
            Self::Knight => 'N',
            Self::Bishop => 'B',
            Self::Rook => 'R',
            Self::Queen => 'Q',
            Self::King => 'K',
        }
    }
    pub const fn from_char(char: char) -> Result<Self, PieceError> {
        Ok(match char {
            'P' | 'p' => Self::Pawn,
            'N' | 'n' => Self::Knight,
            'B' | 'b' => Self::Bishop,
            'R' | 'r' => Self::Rook,
            'Q' | 'q' => Self::Queen,
            'K' | 'k' => Self::King,
            _ => return Err(PieceError::InvalidChar(char))
        })
    }
    pub const fn as_score(&self) -> i32 {
        match self {
            Piece::Pawn => 1,
            Piece::Knight => 3,
            Piece::Bishop => 3,
            Piece::Rook => 4,
            Piece::Queen => 10,
            Piece::King => 100,
        }
    }
    pub const fn as_chess_piece(self, color: Color) -> ChessPiece {
        ChessPiece::from(color, self)
    }
}
