use crate::chess_move::chess_move_ruleset::ChessMoveRuleset;
use crate::color::Color;
use crate::piece::chess_piece::ChessPiece;
use crate::piece::piece::Piece;
use crate::piece::promotion_piece::PromotionPiece;

pub enum ChessPieceMoveSet {
    Set10(ChessPieceMoveRuleset<TEN>),
    Set8(ChessPieceMoveRuleset<EIGHT>),
    Set6(ChessPieceMoveRuleset<SIX>),
    Set4(ChessPieceMoveRuleset<FOUR>),
    // Other sets as needed
}

pub struct ChessPieceMoveRuleset<const SIZE: usize> {
    pub chess_piece: ChessPiece,
    pub move_rulesets: [ChessMoveRuleset; SIZE],
}

const FOUR: usize = 4;
const SIX: usize = 6;
const EIGHT: usize = 8;
const TEN: usize = 10;

pub const PAWN_SIZE: usize = TEN;
pub const KNIGHT_SIZE: usize = EIGHT;
pub const BISHOP_SIZE: usize = FOUR;
pub const ROOK_SIZE: usize = FOUR;
pub const QUEEN_SIZE: usize = EIGHT;
pub const KING_SIZE: usize = TEN;

impl ChessPieceMoveRuleset<FOUR> {
    pub const fn rook(color: Color) -> Self {
        Self {
            chess_piece: ChessPiece::from(color, Piece::Rook),
            move_rulesets: ChessMoveRuleset::any_full_straight(),
        }
    }
    pub const fn bishop(color: Color) -> Self {
        Self {
            chess_piece: ChessPiece::from(color, Piece::Bishop),
            move_rulesets: ChessMoveRuleset::any_full_diagonal(),
        }
    }
}

impl ChessPieceMoveRuleset<TEN> {
    pub const fn pawn(color: Color) -> Self {
        let [left_diagonal_attack, right_diagonal_attack] =
            ChessMoveRuleset::any_facing_diagonal_capture(color.as_facing_direction());
        let [left_en_passant, right_en_passant] =
            ChessMoveRuleset::any_en_passant(color.as_facing_direction());
        Self {
            chess_piece: ChessPiece::from(color, Piece::Pawn),
            move_rulesets: [
                ChessMoveRuleset::promotion(color.as_facing_direction(), PromotionPiece::Queen),
                ChessMoveRuleset::promotion(color.as_facing_direction(), PromotionPiece::Rook),
                ChessMoveRuleset::promotion(color.as_facing_direction(), PromotionPiece::Knight),
                ChessMoveRuleset::promotion(color.as_facing_direction(), PromotionPiece::Rook),
                ChessMoveRuleset::forward(color.as_facing_direction()),
                ChessMoveRuleset::double(
                    color
                        .as_facing_direction()
                        .as_simple_direction()
                        .as_direction(),
                ),
                left_diagonal_attack,
                right_diagonal_attack,
                left_en_passant,
                right_en_passant,
            ],
        }
    }
}

impl ChessPieceMoveRuleset<EIGHT> {
    pub const fn knight(color: Color) -> Self {
        Self {
            chess_piece: ChessPiece::from(color, Piece::Knight),
            move_rulesets: ChessMoveRuleset::any_l_jump(),
        }
    }
    pub const fn queen(color: Color) -> Self {
        Self {
            chess_piece: ChessPiece::from(color, Piece::Queen),
            move_rulesets: ChessMoveRuleset::any_full(),
        }
    }
}

impl ChessPieceMoveRuleset<TEN> {
    pub const fn king(color: Color) -> Self {
        let [a, b, c, d, e, f, g, h] = ChessMoveRuleset::any_single();
        let [castle_left, castle_right] = ChessMoveRuleset::any_castle();
        Self {
            chess_piece: ChessPiece::from(color, Piece::King),
            move_rulesets: [a, b, c, d, e, f, g, h, castle_left, castle_right],
        }
    }
}
