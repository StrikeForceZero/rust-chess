use std::fmt::Display;
use crate::board_position::BoardPosition;
use crate::board_scanner::BoardScanner;
use crate::castle_rights::CastleRights;
use crate::castle_side::CastleSide;
use crate::chess_piece::ChessPiece;
use crate::chess_piece_move_ruleset::ChessPieceMoveSet;
use crate::game_state::GameState;
use crate::move_ruleset::{CaptureOnlyType, DirectionRestriction, MoveRuleset, MoveType};
use crate::piece::Piece;
use crate::r#move::Move;


pub fn valid_moves_for_normal(game_state: &GameState, from_pos: BoardPosition, ruleset: &MoveRuleset) -> Vec<Move> {
    let mut valid_moves = Vec::new();
    let Some(directional_restriction) = ruleset.directional_restriction else {
        todo!("not implemented or bad state?")
    };
    let piece = game_state.board.get(from_pos).expect("expected piece at pos");
    match directional_restriction {
        DirectionRestriction::LMove(drx, dry) => {
            if !ruleset.is_jump {
                todo!("not implemented")
            }
            let mut last_pos = from_pos;
            let mut out_of_bound = false;
            for _ in 0..drx.amount() {
                let Some(next_pos) = last_pos.next_pos(drx.direction())
                    else {
                        out_of_bound = true;
                        break;
                    };
                last_pos = next_pos;
            }
            if !out_of_bound {
                for _ in 0..dry.amount() {
                    let Some(next_pos) = last_pos.next_pos(dry.direction())
                        else {
                            out_of_bound = true;
                            break;
                        };
                    last_pos = next_pos;
                }
            }
            if !out_of_bound {
                if let Some(blocking_piece) = game_state.board.get(last_pos) {
                    if ruleset.can_capture && blocking_piece.as_color() != piece.as_color() {
                        valid_moves.push(Move::create_normal_capture(piece, from_pos, last_pos, *blocking_piece))
                    }
                } else {
                    valid_moves.push(Move::create_normal(piece, from_pos, last_pos))
                }
            }
        }
        DirectionRestriction::Amount(dra) => {
            let mut amount_left = dra.amount();
            for (pos, maybe_blocking_piece) in BoardScanner::from_pos(&game_state.board, from_pos, dra.direction()) {
                if amount_left == 0 {
                    break;
                }
                amount_left -= 1;
                match maybe_blocking_piece {
                    Some(blocking_piece) => {
                        if amount_left == 0 && ruleset.can_capture && blocking_piece.as_color() != piece.as_color() {
                            valid_moves.push(Move::create_normal_capture(piece, from_pos, pos, *blocking_piece))
                        }
                        break;
                    },
                    None => {
                        if amount_left == 0 {
                            valid_moves.push(Move::create_normal(piece, from_pos, pos));
                            break;
                        }
                    },
                }
            }
        }
        DirectionRestriction::Limit(drl) => {
            let mut amount_left = drl.amount();
            for (pos, maybe_blocking_piece) in BoardScanner::from_pos(&game_state.board, from_pos, drl.direction()) {
                if amount_left == 0 {
                    break;
                }
                amount_left -= 1;
                match maybe_blocking_piece {
                    Some(blocking_piece) => {
                        if ruleset.can_capture && blocking_piece.as_color() != piece.as_color() {
                            valid_moves.push(Move::create_normal_capture(piece, from_pos, pos, *blocking_piece))
                        }
                        break;
                    },
                    None => {
                        valid_moves.push(Move::create_normal(piece, from_pos, pos));
                    },
                }
            }
        }
    }
    valid_moves
}

pub fn valid_moves_for_capture_only(game_state: &GameState, from_pos: BoardPosition, ruleset: &MoveRuleset, capture_only_type: CaptureOnlyType) -> Vec<Move> {
    let mut valid_moves = Vec::new();
    let Some(directional_restriction) = ruleset.directional_restriction else {
        todo!("not implemented or bad state?")
    };
    if !ruleset.can_capture {
        panic!("bad config!")
    }
    if capture_only_type == CaptureOnlyType::EnPassant && game_state.en_passant_target_pos.is_none() {
        return valid_moves;
    }
    let piece = game_state.board.get(from_pos).expect("expected piece at pos");
    match directional_restriction {
        DirectionRestriction::LMove(drx, dry) => {
            if !ruleset.is_jump {
                todo!("not implemented")
            }
            let mut last_pos = from_pos;
            let mut out_of_bound = false;
            for _ in 0..drx.amount() {
                let Some(next_pos) = last_pos.next_pos(drx.direction())
                    else {
                        out_of_bound = true;
                        break;
                    };
                last_pos = next_pos;
            }
            if !out_of_bound {
                for _ in 0..dry.amount() {
                    let Some(next_pos) = last_pos.next_pos(dry.direction())
                        else {
                            out_of_bound = true;
                            break;
                        };
                    last_pos = next_pos;
                }
            }
            if !out_of_bound {
                if let Some(blocking_piece) = game_state.board.get(last_pos) {
                    if ruleset.can_capture && blocking_piece.as_color() != piece.as_color() {
                        valid_moves.push(Move::create_normal_capture(piece, from_pos, last_pos, *blocking_piece))
                    }
                }
            }
        }
        DirectionRestriction::Amount(dra) => {
            let mut amount_left = dra.amount();
            for (pos, maybe_blocking_piece) in BoardScanner::from_pos(&game_state.board, from_pos, dra.direction()) {
                if amount_left == 0 {
                    break;
                }
                amount_left -= 1;
                match maybe_blocking_piece {
                    Some(blocking_piece) => {
                        if amount_left == 0 && ruleset.can_capture && blocking_piece.as_color() != piece.as_color() {
                            valid_moves.push(Move::create_normal_capture(piece, from_pos, pos, *blocking_piece))
                        }
                        break;
                    },
                    None => {
                        if capture_only_type == CaptureOnlyType::EnPassant {
                            if amount_left != 0 {
                                continue;
                            }
                            let Some(en_passant_target_pos) = game_state.en_passant_target_pos else {
                                // should be impossible as we checked earlier
                                panic!("bad en passant state: en_passant_target_pos was expected");
                            };
                            if pos != en_passant_target_pos {
                                continue;
                            }
                            let Some(en_passant_capture_pos) = en_passant_target_pos.next_pos(piece.as_facing_direction().as_simple_direction().as_direction().reverse()) else {
                                panic!("bad en passant state: position not within board!");
                            };
                            let Some(capture_piece) = game_state.board.get(en_passant_capture_pos) else {
                                panic!("bad en passant state: no capture piece at {en_passant_capture_pos}");
                            };
                            valid_moves.push(Move::create_en_passant(piece, from_pos, pos, en_passant_target_pos, *capture_piece))
                        }
                    },
                }
            }
        }
        DirectionRestriction::Limit(drl) => {
            let mut amount_left = drl.amount();
            for (pos, maybe_blocking_piece) in BoardScanner::from_pos(&game_state.board, from_pos, drl.direction()) {
                if amount_left == 0 {
                    break;
                }
                amount_left -= 1;
                match maybe_blocking_piece {
                    Some(blocking_piece) => {
                        if ruleset.can_capture && blocking_piece.as_color() != piece.as_color() {
                            valid_moves.push(Move::create_normal_capture(piece, from_pos, pos, *blocking_piece));
                        }
                        break;
                    },
                    None => {},
                }
            }
        }
    }
    valid_moves
}

pub fn valid_moves_for_castle(game_state: &GameState, from_pos: BoardPosition, ruleset: &MoveRuleset) -> Vec<Move> {
    let mut valid_moves = Vec::new();
    let piece = game_state.board.get(from_pos).expect("expected piece at pos");
    let Some(castle_rights) = game_state.castle_rights.for_color(game_state.active_color)
        else { return valid_moves; };
    let Some(directional_restriction) = ruleset.directional_restriction else {
        todo!("not implemented or bad state?")
    };
    match directional_restriction {
        DirectionRestriction::LMove(_, _) => todo!("not implemented or bad state?"),
        DirectionRestriction::Limit(_) => todo!("not implemented or bad state?"),
        DirectionRestriction::Amount(da) => {
            let Ok(castle_side) = CastleSide::try_from_direction(da.direction()) else {
                panic!("bad ruleset!")
            };
            if !castle_rights.has(CastleRights::from_castle_side(castle_side)) {
                return valid_moves;
            }
            let mut amount_remaining = da.amount();
            let mut target_pos: Option<BoardPosition> = None;
            for (pos, maybe_blocking_piece) in BoardScanner::from_pos(&game_state.board, from_pos, da.direction()) {
                // prevent underflow since we go beyond the original amount to make sure a rook is present
                amount_remaining = amount_remaining.saturating_sub(1);
                if amount_remaining == 0 && target_pos.is_none() {
                    target_pos = Some(pos);
                }
                if let Some(blocking_piece) = maybe_blocking_piece {
                    let Some(target_pos) = target_pos
                        else { return valid_moves };
                    if blocking_piece.as_color() != piece.as_color() || blocking_piece.as_piece() != Piece::Rook {
                        return valid_moves;
                    }
                    // if starting pos requirement is set, check the rook as well
                    if ruleset.only_from_starting_pos && game_state.board.is_pos_starting_pos(pos) {
                        valid_moves.push(Move::create_castle(piece, from_pos, target_pos, castle_side));
                    }
                    break;
                }
            }
        }
    }
    valid_moves
}

pub fn valid_moves_from_rulesets(game_state: &GameState, from_pos: BoardPosition, move_rulesets: &[MoveRuleset]) -> Vec<Move> {
    let mut valid_moves = Vec::new();
    for ruleset in move_rulesets {
        if ruleset.only_from_starting_pos && !game_state.board.is_pos_starting_pos(from_pos) {
            continue;
        }
        let mut new_valid_moves = match ruleset.move_type {
            MoveType::Normal => valid_moves_for_normal(game_state, from_pos, ruleset),
            MoveType::WhenCapturingOnly(capture_only_type) => valid_moves_for_capture_only(game_state, from_pos, ruleset, capture_only_type),
            MoveType::Castle => valid_moves_for_castle(game_state, from_pos, ruleset),
        };
        valid_moves.append(&mut new_valid_moves);
    }
    valid_moves
}

pub fn move_search(game_state: &GameState) -> Vec<Move> {
    let mut valid_moves: Vec<Move> = Vec::new();
    for (pos, maybe_piece) in game_state.board.as_iter() {
        valid_moves.append(&mut move_search_from_pos(game_state, pos))
    }
    valid_moves
}

pub fn move_search_from_pos(game_state: &GameState, pos: BoardPosition) -> Vec<Move> {
    let maybe_piece = game_state.board.get(pos);
    let Some(piece) = maybe_piece else {
        return Vec::new();
    };
    // not this colors turn
    if piece.as_color() != game_state.active_color {
        return Vec::new();
    }
    match piece.as_move_set() {
        ChessPieceMoveSet::Set10(ms) => valid_moves_from_rulesets(&game_state, pos, ms.move_rulesets.as_slice()),
        ChessPieceMoveSet::Set8(ms) => valid_moves_from_rulesets(&game_state, pos, ms.move_rulesets.as_slice()),
        ChessPieceMoveSet::Set6(ms) => valid_moves_from_rulesets(&game_state, pos, ms.move_rulesets.as_slice()),
        ChessPieceMoveSet::Set4(ms) => valid_moves_from_rulesets(&game_state, pos, ms.move_rulesets.as_slice()),
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::board_position::BoardPosition;
    use crate::game_state::GameState;
    use crate::r#move::Move;
    use crate::position::*;
    use crate::chess_piece::ChessPiece;
    use crate::castle_side::CastleSide;
    use crate::fen::{FEN_STARTING_POS, deserialize};
    use crate::utils::print_slice_elements_using_display;
    use super::*;

    #[rstest]
    #[case(FEN_STARTING_POS, A2, vec![
        Move::create_normal(ChessPiece::WhitePawn, A2, A3),
        Move::create_normal(ChessPiece::WhitePawn, A2, A4),
    ])]
    #[case("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1", WHITE_KING_SQUARE, vec![
        Move::create_normal(ChessPiece::WhiteKing, WHITE_KING_SQUARE, WHITE_KING_SIDE_BISHOP_SQUARE),
        Move::create_normal(ChessPiece::WhiteKing, WHITE_KING_SQUARE, WHITE_QUEEN_SQUARE),
        Move::create_castle(ChessPiece::WhiteKing, WHITE_KING_SQUARE, WHITE_KING_SIDE_KING_CASTLE_SQUARE, CastleSide::King),
        Move::create_castle(ChessPiece::WhiteKing, WHITE_KING_SQUARE, WHITE_QUEEN_SIDE_KING_CASTLE_SQUARE, CastleSide::Queen),
    ])]
    fn test_move_search_from_pos(
        #[case] fen_str: &'static str,
        #[case] pos: BoardPosition,
        #[case] expected: Vec<Move>,
    ) {
        let game_state = deserialize(fen_str).expect("bad fen string!");
        let valid_moves = move_search_from_pos(&game_state, pos);
        if expected != valid_moves {
            println!("expected:"); print_slice_elements_using_display(&expected);
            println!("got:"); print_slice_elements_using_display(&valid_moves);
        }
        assert_eq!(expected, valid_moves)
    }
}
