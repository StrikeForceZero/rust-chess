use crate::{bit_board, chess_piece_move_rulesets};
use crate::bit_board::BitBoard;
use crate::board_rank::{BLACK_BACK_RANK, BLACK_PAWNN_RANK, WHITE_BACK_RANK, WHITE_PAWNN_RANK};
use crate::chess_piece_move_ruleset::ChessPieceMoveRuleset;
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
    pub const fn from(color: Color, piece: Piece) -> Self {
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
            Self::WhitePawn => BitBoard::from_value(bit_board::PAWN_STARTING_POS << WHITE_PAWNN_RANK.as_shift_offset()),
            Self::WhiteRook => BitBoard::from_value(bit_board::ROOK_STARTING_POS << WHITE_BACK_RANK.as_shift_offset()),
            Self::WhiteKnight => BitBoard::from_value(bit_board::KNIGHT_STARTING_POS << WHITE_BACK_RANK.as_shift_offset()),
            Self::WhiteBishop => BitBoard::from_value(bit_board::BISHOP_STARTING_POS << WHITE_BACK_RANK.as_shift_offset()),
            Self::WhiteQueen => BitBoard::from_value(bit_board::QUEEN_STARTING_POS << WHITE_BACK_RANK.as_shift_offset()),
            Self::WhiteKing => BitBoard::from_value(bit_board::KING_STARTING_POS << WHITE_BACK_RANK.as_shift_offset()),
            Self::BlackPawn => BitBoard::from_value(bit_board::PAWN_STARTING_POS << BLACK_PAWNN_RANK.as_shift_offset()),
            Self::BlackRook => BitBoard::from_value(bit_board::ROOK_STARTING_POS << BLACK_BACK_RANK.as_shift_offset()),
            Self::BlackKnight => BitBoard::from_value(bit_board::KNIGHT_STARTING_POS << BLACK_BACK_RANK.as_shift_offset()),
            Self::BlackBishop => BitBoard::from_value(bit_board::BISHOP_STARTING_POS << BLACK_BACK_RANK.as_shift_offset()),
            Self::BlackQueen => BitBoard::from_value(bit_board::QUEEN_STARTING_POS << BLACK_BACK_RANK.as_shift_offset()),
            Self::BlackKing => BitBoard::from_value(bit_board::KING_STARTING_POS << BLACK_BACK_RANK.as_shift_offset()),
        }
    }

    const fn as_move_set_10(&self) -> Option<ChessPieceMoveRuleset<10>> {
        Some(match self {
            Self::WhiteKing => chess_piece_move_rulesets::WHITE_KING,
            Self::BlackKing => chess_piece_move_rulesets::BLACK_KING,
            _ => return None,
        })
    }

    const fn as_move_set_8(&self) -> Option<ChessPieceMoveRuleset<8>> {
        Some(match self {
            Self::WhiteKnight => chess_piece_move_rulesets::WHITE_KNIGHT,
            Self::WhiteQueen => chess_piece_move_rulesets::WHITE_QUEEN,
            Self::BlackKnight => chess_piece_move_rulesets::BLACK_KNIGHT,
            Self::BlackQueen => chess_piece_move_rulesets::BLACK_QUEEN,
            _ => return None,
        })
    }


    const fn as_move_set_6(&self) -> Option<ChessPieceMoveRuleset<6>> {
        Some(match self {
            Self::WhitePawn => chess_piece_move_rulesets::WHITE_PAWN,
            Self::BlackPawn => chess_piece_move_rulesets::BLACK_PAWN,
            _ => return None,
        })
    }

    const fn as_move_set_4(&self) -> Option<ChessPieceMoveRuleset<4>> {
        Some(match self {
            Self::WhiteRook => chess_piece_move_rulesets::WHITE_ROOK,
            Self::WhiteBishop => chess_piece_move_rulesets::WHITE_BISHOP,
            Self::BlackRook => chess_piece_move_rulesets::BLACK_ROOK,
            Self::BlackBishop => chess_piece_move_rulesets::BLACK_BISHOP,
            _ => return None,
        })
    }
}
