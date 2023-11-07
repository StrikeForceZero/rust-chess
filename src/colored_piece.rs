use crate::bit_board;
use crate::bit_board::BitBoard;
use crate::board_rank::BoardRank;
use crate::color::Color;
use crate::piece::Piece;

pub struct ColoredPiece(Color, Piece);

impl ColoredPiece {
    pub fn as_char(&self) -> char {
        match self {
            Self(Color::White, piece) => piece.as_char(),
            Self(Color::Black, piece) => piece.as_char().to_ascii_lowercase(),
        }
    }
    pub fn from(color: Color, piece: Piece) -> Self {
        Self(color, piece)
    }
    pub fn as_starting_bitboard(&self) -> BitBoard {
        match self {
            Self(Color::White, Piece::Pawn) => BitBoard::from_value(bit_board::PAWN << BoardRank::Two.as_shift_offset()),
            Self(Color::White, Piece::Rook) => BitBoard::from_value(bit_board::ROOK),
            Self(Color::White, Piece::Knight) => BitBoard::from_value(bit_board::KNIGHT),
            Self(Color::White, Piece::Bishop) => BitBoard::from_value(bit_board::BISHOP),
            Self(Color::White, Piece::Queen) => BitBoard::from_value(bit_board::QUEEN),
            Self(Color::White, Piece::King) => BitBoard::from_value(bit_board::KING),
            Self(Color::Black, Piece::Pawn) => BitBoard::from_value(bit_board::PAWN << BoardRank::Seven.as_shift_offset()),
            Self(Color::Black, Piece::Rook) => BitBoard::from_value(bit_board::ROOK << BoardRank::Eight.as_shift_offset()),
            Self(Color::Black, Piece::Knight) => BitBoard::from_value(bit_board::KNIGHT << BoardRank::Eight.as_shift_offset()),
            Self(Color::Black, Piece::Bishop) => BitBoard::from_value(bit_board::BISHOP << BoardRank::Eight.as_shift_offset()),
            Self(Color::Black, Piece::Queen) => BitBoard::from_value(bit_board::QUEEN << BoardRank::Eight.as_shift_offset()),
            Self(Color::Black, Piece::King) => BitBoard::from_value(bit_board::KING << BoardRank::Eight.as_shift_offset()),
        }
    }
}
