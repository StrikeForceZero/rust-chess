use crate::chess_piece_move_ruleset::{ChessPieceMoveRuleset, PAWN_SIZE, KNIGHT_SIZE, BISHOP_SIZE, ROOK_SIZE, QUEEN_SIZE, KING_SIZE};
use crate::color::Color;

pub const WHITE_KING: ChessPieceMoveRuleset<KING_SIZE> = ChessPieceMoveRuleset::king(Color::White);
pub const WHITE_QUEEN: ChessPieceMoveRuleset<QUEEN_SIZE> = ChessPieceMoveRuleset::queen(Color::White);
pub const WHITE_ROOK: ChessPieceMoveRuleset<ROOK_SIZE> = ChessPieceMoveRuleset::rook(Color::White);
pub const WHITE_BISHOP: ChessPieceMoveRuleset<BISHOP_SIZE> = ChessPieceMoveRuleset::bishop(Color::White);
pub const WHITE_KNIGHT: ChessPieceMoveRuleset<KNIGHT_SIZE> = ChessPieceMoveRuleset::knight(Color::White);
pub const WHITE_PAWN: ChessPieceMoveRuleset<PAWN_SIZE> = ChessPieceMoveRuleset::pawn(Color::White);

pub const BLACK_KING: ChessPieceMoveRuleset<KING_SIZE> = ChessPieceMoveRuleset::king(Color::Black);
pub const BLACK_QUEEN: ChessPieceMoveRuleset<QUEEN_SIZE> = ChessPieceMoveRuleset::queen(Color::Black);
pub const BLACK_ROOK: ChessPieceMoveRuleset<ROOK_SIZE> = ChessPieceMoveRuleset::rook(Color::Black);
pub const BLACK_BISHOP: ChessPieceMoveRuleset<BISHOP_SIZE> = ChessPieceMoveRuleset::bishop(Color::Black);
pub const BLACK_KNIGHT: ChessPieceMoveRuleset<KNIGHT_SIZE> = ChessPieceMoveRuleset::knight(Color::Black);
pub const BLACK_PAWN: ChessPieceMoveRuleset<PAWN_SIZE> = ChessPieceMoveRuleset::pawn(Color::Black);
