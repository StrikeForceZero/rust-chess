use crate::chess_piece::ChessPiece;
use crate::chess_piece_move_ruleset::ChessPieceMoveRuleset;
use crate::direction::Direction;
use crate::direction_amount::DirectionAmount;
use crate::move_ruleset::{DirectionRestriction, MoveRuleset};

const WHITE_PAWN: ChessPieceMoveRuleset<1> = ChessPieceMoveRuleset {
    chess_piece: ChessPiece::WhitePawn,
    move_rulesets: [
        MoveRuleset {
            directional_restriction: Some(DirectionRestriction::Amount(DirectionAmount(Direction::North, 1))),
            ..MoveRuleset::default()
        },
    ],
};


const BLACK_PAWN: ChessPieceMoveRuleset<1> = ChessPieceMoveRuleset {
    chess_piece: ChessPiece::BlackPawn,
    move_rulesets: [
        MoveRuleset {
            directional_restriction: Some(DirectionRestriction::Amount(DirectionAmount(Direction::South, 1))),
            ..MoveRuleset::default()
        },
    ],
};
