use crate::board::Board;
use crate::board_position::BoardPosition;
use crate::board_scanner::BoardScanner;
use crate::chess_piece_move_ruleset::ChessPieceMoveSet;
use crate::game_state::GameState;
use crate::move_ruleset::{CaptureOnlyType, DirectionRestriction, MoveRuleset, MoveType};
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
                        valid_moves.push(Move {
                            piece,
                            from: from_pos,
                            to: last_pos,
                            captured_piece: Some(*blocking_piece),
                        })
                    }
                } else {
                    valid_moves.push(Move {
                        piece,
                        from: from_pos,
                        to: last_pos,
                        captured_piece: None,
                    })
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
                            valid_moves.push(Move {
                                piece,
                                from: from_pos,
                                to: pos,
                                captured_piece: Some(*blocking_piece),
                            })
                        }
                        break;
                    },
                    None => {
                        if amount_left == 0 {
                            valid_moves.push(Move {
                                piece,
                                from: from_pos,
                                to: pos,
                                captured_piece: None,
                            });
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
                            valid_moves.push(Move {
                                piece,
                                from: from_pos,
                                to: pos,
                                captured_piece: Some(*blocking_piece),
                            })
                        }
                        break;
                    },
                    None => {
                        valid_moves.push(Move {
                            piece,
                            from: from_pos,
                            to: pos,
                            captured_piece: None,
                        });
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
                        valid_moves.push(Move {
                            piece,
                            from: from_pos,
                            to: last_pos,
                            captured_piece: Some(*blocking_piece),
                        })
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
                            valid_moves.push(Move {
                                piece,
                                from: from_pos,
                                to: pos,
                                captured_piece: Some(*blocking_piece),
                            })
                        }
                        break;
                    },
                    None => {},
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
                            valid_moves.push(Move {
                                piece,
                                from: from_pos,
                                to: pos,
                                captured_piece: Some(*blocking_piece),
                            })
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
    let Some(castle_rights) = game_state.castle_rights.for_color(game_state.active_color)
        else { return valid_moves; };

    todo!("not implemented")
}

pub fn valid_moves_from_rulesets(game_state: &GameState, from_pos: BoardPosition, move_rulesets: &[MoveRuleset]) -> Vec<Move> {
    let mut valid_moves = Vec::new();
    for ruleset in move_rulesets {
        if ruleset.only_from_starting_pos && game_state.board.is_pos_starting_pos(from_pos) {
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

pub fn move_search(game_state: GameState) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    for (pos, maybe_piece) in game_state.board.as_iter() {
        let Some(piece) = maybe_piece
            else { continue };
        // not this colors turn
        if piece.as_color() != game_state.active_color {
            continue;
        }
        let valid_moves = match piece.as_move_set() {
            ChessPieceMoveSet::Set10(ms) => valid_moves_from_rulesets(&game_state, pos, ms.move_rulesets.as_slice()),
            ChessPieceMoveSet::Set8(ms) => valid_moves_from_rulesets(&game_state, pos, ms.move_rulesets.as_slice()),
            ChessPieceMoveSet::Set6(ms) => valid_moves_from_rulesets(&game_state, pos, ms.move_rulesets.as_slice()),
            ChessPieceMoveSet::Set4(ms) => valid_moves_from_rulesets(&game_state, pos, ms.move_rulesets.as_slice()),
        };
    }
    moves
}
