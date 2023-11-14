use std::fmt::{Display, Formatter};
use crate::board::board_file::BoardFile;
use crate::board::board_position::BoardPosition;
use crate::board::board_rank::BoardRank;
use crate::chess_move::chess_move::{ChessMove, ChessMoveType};
use crate::direction::castle_side::CastleSide;
use crate::piece::chess_piece::ChessPiece;
use crate::piece::piece::Piece;
use crate::piece::promotion_piece::PromotionPiece;

#[derive(Copy, Clone, Debug)]
pub enum PgnCheckFlag {
    Check,
    Mate,
}

impl PgnCheckFlag {
    pub const fn as_str(&self) -> &'static str {
        match self {
            PgnCheckFlag::Check => "+",
            PgnCheckFlag::Mate => "#",
        }
    }
}
impl Display for PgnCheckFlag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// TODO: handle castling
#[derive(Debug)]
pub struct PgnMoveDetail {
    pub chess_piece: ChessPiece,
    pub from_board_file: Option<BoardFile>,
    pub from_board_rank: Option<BoardRank>,
    pub to_pos: BoardPosition,
    pub is_capture: bool,
    pub check_flag: Option<PgnCheckFlag>,
    pub comment: Option<String>,
    pub promotion: Option<PromotionPiece>,
}

impl PgnMoveDetail {
    pub fn from_chess_move(chess_move: &ChessMove) -> Self {
        let promotion = match chess_move.move_type {
            ChessMoveType::Castle(_) => todo!("implement"),
            ChessMoveType::Promotion(promotion) => Some(promotion),
            _ => None,
        };
        Self {
            chess_piece: chess_move.piece,
            // TODO: resolve ambiguous
            from_board_file: Some(*chess_move.from.file()),
            from_board_rank: Some(*chess_move.from.rank()),
            to_pos: chess_move.to,
            is_capture: chess_move.captured_piece.is_some(),
            // TODO: identify if check - maybe add to ChessMove? might be out of scope
            check_flag: None,
            comment: None,
            promotion,
        }
    }

    pub fn looks_like_castle(&self) -> bool {
        if self.chess_piece.as_piece() == Piece::King {
            if let Some(board_file) = self.from_board_file {
                if board_file == BoardFile::E
                    // if a king is moving 2 places away its probably a castle
                    && (*self.to_pos.file() == BoardFile::G || *self.to_pos.file() == BoardFile::C
                ) {
                    return true;
                }
            }
        }
        false
    }
}

impl Display for PgnMoveDetail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // handled in PgnMove.to_string()
        // if self.looks_like_castle() {
        //     return write!(f, "{}", CastleSide::from_pos(self.to_pos).as_pgn_str());
        // }
        let piece = match self.chess_piece.as_piece() {
            Piece::Pawn => String::from(""),
            _ => self.chess_piece.as_piece().as_char().to_string(),
        };
        let from_file = if let Some(file) = self.from_board_file {
            file.to_string().to_lowercase()
        } else {
            String::from("")
        };
        let from_rank = if let Some(rank) = self.from_board_rank {
            rank.to_string().to_lowercase()
        } else {
            String::from("")
        };
        let capture = if self.is_capture { "x" } else { "" };
        let to_pos = self.to_pos.to_string().to_lowercase();
        let promotion = if let Some(promotion) = self.promotion {
            format!("={}", promotion.as_piece().as_char())
        } else {
            String::from("")
        };
        let check_flag = if let Some(check_flag) = self.check_flag {
            check_flag.as_str()
        } else { "" };
        let comment = if let Some(comment) = &self.comment {
            format!(" {{{comment}}}")
        } else {
            String::from("")
        };
        write!(f, "{piece}{from_file}{from_rank}{capture}{to_pos}{promotion}{check_flag}{comment}")
    }
}
