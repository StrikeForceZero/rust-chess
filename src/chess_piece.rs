use thiserror::Error;
use crate::{bit_board, chess_piece_move_rulesets};
use crate::bit_board::BitBoard;
use crate::bit_board_const::BitBoardConst;
use crate::board_rank::{BLACK_BACK_RANK, BLACK_PAWNN_RANK, WHITE_BACK_RANK, WHITE_PAWNN_RANK};
use crate::chess_piece_move_ruleset::{ChessPieceMoveRuleset, ChessPieceMoveSet};
use crate::color::Color;
use crate::facing_direction::FacingDirection;
use crate::piece::Piece;

#[derive(Error, Debug, Clone, Copy)]
pub enum ChessPieceParseError {
    #[error("Invalid character for ChessPiece: {0}")]
    InvalidChar(char),
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
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
    pub const fn as_char(&self) -> char {
        let piece = self.as_piece();
        match self.as_color() {
            Color::White => piece.as_char(),
            Color::Black => piece.as_char().to_ascii_lowercase(),
        }
    }
    pub const fn as_color(&self) -> Color {
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
    pub const fn as_piece(&self) -> Piece {
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
    pub const fn as_facing_direction(&self) -> FacingDirection {
        match self {
            Self::WhiteKing => FacingDirection::North,
            Self::WhiteQueen => FacingDirection::North,
            Self::WhiteRook => FacingDirection::North,
            Self::WhiteBishop => FacingDirection::North,
            Self::WhiteKnight => FacingDirection::North,
            Self::WhitePawn => FacingDirection::North,
            Self::BlackKing => FacingDirection::South,
            Self::BlackQueen => FacingDirection::South,
            Self::BlackRook => FacingDirection::South,
            Self::BlackBishop => FacingDirection::South,
            Self::BlackKnight => FacingDirection::South,
            Self::BlackPawn => FacingDirection::South,
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

    pub const fn as_starting_bitboard_const(&self) -> BitBoardConst {
        match self {
            Self::WhitePawn => BitBoardConst::from(bit_board::PAWN_STARTING_POS << WHITE_PAWNN_RANK.as_shift_offset()),
            Self::WhiteRook => BitBoardConst::from(bit_board::ROOK_STARTING_POS << WHITE_BACK_RANK.as_shift_offset()),
            Self::WhiteKnight => BitBoardConst::from(bit_board::KNIGHT_STARTING_POS << WHITE_BACK_RANK.as_shift_offset()),
            Self::WhiteBishop => BitBoardConst::from(bit_board::BISHOP_STARTING_POS << WHITE_BACK_RANK.as_shift_offset()),
            Self::WhiteQueen => BitBoardConst::from(bit_board::QUEEN_STARTING_POS << WHITE_BACK_RANK.as_shift_offset()),
            Self::WhiteKing => BitBoardConst::from(bit_board::KING_STARTING_POS << WHITE_BACK_RANK.as_shift_offset()),
            Self::BlackPawn => BitBoardConst::from(bit_board::PAWN_STARTING_POS << BLACK_PAWNN_RANK.as_shift_offset()),
            Self::BlackRook => BitBoardConst::from(bit_board::ROOK_STARTING_POS << BLACK_BACK_RANK.as_shift_offset()),
            Self::BlackKnight => BitBoardConst::from(bit_board::KNIGHT_STARTING_POS << BLACK_BACK_RANK.as_shift_offset()),
            Self::BlackBishop => BitBoardConst::from(bit_board::BISHOP_STARTING_POS << BLACK_BACK_RANK.as_shift_offset()),
            Self::BlackQueen => BitBoardConst::from(bit_board::QUEEN_STARTING_POS << BLACK_BACK_RANK.as_shift_offset()),
            Self::BlackKing => BitBoardConst::from(bit_board::KING_STARTING_POS << BLACK_BACK_RANK.as_shift_offset()),
        }
    }

    pub fn from_char(c: char) -> Result<ChessPiece, ChessPieceParseError> {
        Ok(match c {
            'K' => Self::WhiteKing,
            'Q' => Self::WhiteQueen,
            'R' => Self::WhiteRook,
            'B' => Self::WhiteBishop,
            'N' => Self::WhiteKnight,
            'P' => Self::WhitePawn,
            'k' => Self::BlackKing,
            'q' => Self::BlackQueen,
            'r' => Self::BlackRook,
            'b' => Self::BlackBishop,
            'n' => Self::BlackKnight,
            'p' => Self::BlackPawn,
            _ => return Err(ChessPieceParseError::InvalidChar(c)),
        })
    }

    pub const fn as_score(&self) -> i32 {
        self.as_piece().as_score() * self.as_color().as_score()
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

    pub const fn as_move_set(&self) -> ChessPieceMoveSet {
        match self {
            Self::WhiteRook => ChessPieceMoveSet::Set4(chess_piece_move_rulesets::WHITE_ROOK),
            Self::WhiteBishop => ChessPieceMoveSet::Set4(chess_piece_move_rulesets::WHITE_BISHOP),
            Self::BlackRook => ChessPieceMoveSet::Set4(chess_piece_move_rulesets::BLACK_ROOK),
            Self::BlackBishop => ChessPieceMoveSet::Set4(chess_piece_move_rulesets::BLACK_BISHOP),
            Self::WhitePawn => ChessPieceMoveSet::Set6(chess_piece_move_rulesets::WHITE_PAWN),
            Self::BlackPawn => ChessPieceMoveSet::Set6(chess_piece_move_rulesets::BLACK_PAWN),
            Self::WhiteKnight => ChessPieceMoveSet::Set8(chess_piece_move_rulesets::WHITE_KNIGHT),
            Self::WhiteQueen => ChessPieceMoveSet::Set8(chess_piece_move_rulesets::WHITE_QUEEN),
            Self::BlackKnight => ChessPieceMoveSet::Set8(chess_piece_move_rulesets::BLACK_KNIGHT),
            Self::BlackQueen => ChessPieceMoveSet::Set8(chess_piece_move_rulesets::BLACK_QUEEN),
            Self::WhiteKing => ChessPieceMoveSet::Set10(chess_piece_move_rulesets::WHITE_KING),
            Self::BlackKing => ChessPieceMoveSet::Set10(chess_piece_move_rulesets::BLACK_KING),
        }
    }
}
