use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
pub enum PieceError {
    #[error("Invalid character for Piece: {0}")]
    InvalidChar(char),
}


#[derive(Copy, Clone)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    pub fn as_char(&self) -> char {
        match self {
            Self::Pawn => 'P',
            Self::Knight => 'N',
            Self::Bishop => 'B',
            Self::Rook => 'R',
            Self::Queen => 'Q',
            Self::King => 'K',
        }
    }
    pub fn from_char(char: char) -> Result<Self, PieceError> {
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
}
