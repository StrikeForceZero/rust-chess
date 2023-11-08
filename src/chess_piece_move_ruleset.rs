use crate::chess_piece::ChessPiece;
use crate::move_ruleset::MoveRuleset;

pub struct ChessPieceMoveRuleset {
    pub chess_piece: ChessPiece,
    pub move_rulesets: Vec<MoveRuleset>,
}
