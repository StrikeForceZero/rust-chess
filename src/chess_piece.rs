use crate::bit_board;
use crate::bit_board::BitBoard;
use crate::board_rank::{BLACK_BACK_RANK, BLACK_PAWNN_RANK, WHITE_BACK_RANK, WHITE_PAWNN_RANK};
use crate::color::Color;
use crate::piece::Piece;

pub enum ChessPiece {
    WhiteKing,
    WhiteQueen,
    WhiteRook,
    WhiteBishop,
    WhiteKnight,
    WhitePawn,

    BlackKing,
    BlackQueen,
    BlackRook,
    BlackBishop,
    BlackKnight,
    BlackPawn,
}

impl ChessPiece {
    pub fn as_char(&self) -> char {
        let piece = self.as_piece();
        match self.as_color() {
            Color::White => piece.as_char(),
            Color::Black => piece.as_char().to_ascii_lowercase(),
        }
    }
    pub fn as_color(&self) -> Color {
        match self {
            Self::WhiteKing => Color::White,
            Self::WhiteQueen => Color::White,
            Self::WhiteRook => Color::White,
            Self::WhiteBishop => Color::White,
            Self::WhiteKnight => Color::White,
            Self::WhitePawn => Color::White,
            Self::BlackKing => Color::Black,
            Self::BlackQueen => Color::Black,
            Self::BlackRook => Color::Black,
            Self::BlackBishop => Color::Black,
            Self::BlackKnight => Color::Black,
            Self::BlackPawn => Color::Black,
        }
    }
    pub fn as_piece(&self) -> Piece {
        match self {
            Self::WhiteKing => Piece::King,
            Self::WhiteQueen => Piece::Queen,
            Self::WhiteRook => Piece::Rook,
            Self::WhiteBishop => Piece::Bishop,
            Self::WhiteKnight => Piece::Knight,
            Self::WhitePawn => Piece::Pawn,
            Self::BlackKing => Piece::King,
            Self::BlackQueen => Piece::Queen,
            Self::BlackRook => Piece::Rook,
            Self::BlackBishop => Piece::Bishop,
            Self::BlackKnight => Piece::Knight,
            Self::BlackPawn => Piece::Pawn,
        }
    }
    pub fn from(color: Color, piece: Piece) -> Self {
        match color {
            Color::White => match piece {
                Piece::Pawn => Self::WhiteKing,
                Piece::Knight => Self::WhiteQueen,
                Piece::Bishop => Self::WhiteRook,
                Piece::Rook => Self::WhiteBishop,
                Piece::Queen => Self::WhiteKnight,
                Piece::King => Self::WhitePawn,
            },
            Color::Black => match piece {
                Piece::Pawn => Self::BlackKing,
                Piece::Knight => Self::BlackQueen,
                Piece::Bishop => Self::BlackRook,
                Piece::Rook => Self::BlackBishop,
                Piece::Queen => Self::BlackKnight,
                Piece::King => Self::BlackPawn,
            },
        }
    }
    pub fn as_starting_bitboard(&self) -> BitBoard {
        match self {
            Self::WhitePawn => BitBoard::from_value(bit_board::PAWN << WHITE_PAWNN_RANK.as_shift_offset()),
            Self::WhiteRook => BitBoard::from_value(bit_board::ROOK << WHITE_BACK_RANK.as_shift_offset()),
            Self::WhiteKnight => BitBoard::from_value(bit_board::KNIGHT << WHITE_BACK_RANK.as_shift_offset()),
            Self::WhiteBishop => BitBoard::from_value(bit_board::BISHOP << WHITE_BACK_RANK.as_shift_offset()),
            Self::WhiteQueen => BitBoard::from_value(bit_board::QUEEN << WHITE_BACK_RANK.as_shift_offset()),
            Self::WhiteKing => BitBoard::from_value(bit_board::KING << WHITE_BACK_RANK.as_shift_offset()),
            Self::BlackPawn => BitBoard::from_value(bit_board::PAWN << BLACK_PAWNN_RANK.as_shift_offset()),
            Self::BlackRook => BitBoard::from_value(bit_board::ROOK << BLACK_BACK_RANK.as_shift_offset()),
            Self::BlackKnight => BitBoard::from_value(bit_board::KNIGHT << BLACK_BACK_RANK.as_shift_offset()),
            Self::BlackBishop => BitBoard::from_value(bit_board::BISHOP << BLACK_BACK_RANK.as_shift_offset()),
            Self::BlackQueen => BitBoard::from_value(bit_board::QUEEN << BLACK_BACK_RANK.as_shift_offset()),
            Self::BlackKing => BitBoard::from_value(bit_board::KING << BLACK_BACK_RANK.as_shift_offset()),
        }
    }
}
