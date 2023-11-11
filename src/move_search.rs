use std::fmt::Display;
use crate::board_position::BoardPosition;
use crate::board_rank::BoardRank;
use crate::board_scanner::BoardScanner;
use crate::castle_rights::CastleRights;
use crate::castle_side::CastleSide;
use crate::chess_piece::ChessPiece;
use crate::chess_piece_move_ruleset::ChessPieceMoveSet;
use crate::color::Color;
use crate::game_state::GameState;
use crate::invalid_move_error::InvalidMoveError;
use crate::move_ruleset::{CaptureOnlyType, DirectionRestriction, MoveRuleset, MoveDefType};
use crate::piece::Piece;
use crate::promotion_piece::PromotionPiece;
use crate::r#move::{Move, MoveType};
use crate::utils::print_slice_elements_using_display;

#[derive(Debug, Default, Copy, Clone)]
pub struct MoveSearchOptions {
    pub skip_active_color_check: bool,
    pub active_color_override: Option<Color>,
}


pub fn provisional_moves_for_normal(game_state: &GameState, from_pos: BoardPosition, ruleset: &MoveRuleset, options: Option<MoveSearchOptions>) -> Vec<Move> {
    let options = options.unwrap_or_default();
    let mut unchecked_moves = Vec::new();
    let Some(directional_restriction) = ruleset.directional_restriction else {
        todo!("not implemented or bad state?")
    };
    let piece = game_state.board.get(from_pos).expect("expected piece at pos");
    if let Some(required_rank) = ruleset.requires_rank {
        if required_rank != *from_pos.rank() {
            return Vec::new();
        }
    }
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
                        unchecked_moves.push(Move::create_normal_capture(piece, from_pos, last_pos, *blocking_piece))
                    }
                } else {
                    unchecked_moves.push(Move::create_normal(piece, from_pos, last_pos))
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
                            unchecked_moves.push(Move::create_normal_capture(piece, from_pos, pos, *blocking_piece))
                        }
                        break;
                    },
                    None => {
                        if amount_left == 0 {
                            if let Some(promote_to) = ruleset.promote_to {
                                unchecked_moves.push(Move::create_promotion(piece, from_pos, pos, promote_to));
                            } else {
                                unchecked_moves.push(Move::create_normal(piece, from_pos, pos));
                            }
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
                            unchecked_moves.push(Move::create_normal_capture(piece, from_pos, pos, *blocking_piece))
                        }
                        break;
                    },
                    None => {
                        unchecked_moves.push(Move::create_normal(piece, from_pos, pos));
                    },
                }
            }
        }
    }
    unchecked_moves
}

pub fn provisional_moves_for_capture_only(game_state: &GameState, from_pos: BoardPosition, ruleset: &MoveRuleset, capture_only_type: CaptureOnlyType, options: Option<MoveSearchOptions>) -> Vec<Move> {
    let options = options.unwrap_or_default();
    let mut unchecked_moves = Vec::new();
    let Some(directional_restriction) = ruleset.directional_restriction else {
        todo!("not implemented or bad state?")
    };
    if !ruleset.can_capture {
        panic!("bad config!")
    }
    let piece = game_state.board.get(from_pos).expect("expected piece at pos");
    // return early if ruleset is for en passant and there is no en passant or we arent the active color
    // purposefully ignoring override
    if capture_only_type == CaptureOnlyType::EnPassant && (game_state.en_passant_target_pos.is_none() || game_state.active_color != piece.as_color()) {
        return unchecked_moves;
    }
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
                        unchecked_moves.push(Move::create_normal_capture(piece, from_pos, last_pos, *blocking_piece))
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
                            if capture_only_type == CaptureOnlyType::EnPassant {
                                // en passant square should be empty
                                break;
                            }
                            unchecked_moves.push(Move::create_normal_capture(piece, from_pos, pos, *blocking_piece))
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
                            if *en_passant_target_pos.rank() == BoardRank::Six && game_state.active_color == Color::Black
                                || *en_passant_target_pos.rank() == BoardRank::Three && game_state.active_color == Color::White
                            {
                                // TODO: how are we getting here?
                                continue;
                            }
                            if pos != en_passant_target_pos {
                                continue;
                            }
                            let Some(en_passant_capture_pos) = en_passant_target_pos.next_pos(piece.as_facing_direction().as_simple_direction().as_direction().reverse()) else {
                                panic!("bad en passant state: position not within board!");
                            };
                            let Some(capture_piece) = game_state.board.get(en_passant_capture_pos) else {
                                panic!("bad en passant state: no capture piece at {en_passant_capture_pos}");
                            };
                            unchecked_moves.push(Move::create_en_passant(piece, from_pos, pos, en_passant_capture_pos, *capture_piece))
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
                            unchecked_moves.push(Move::create_normal_capture(piece, from_pos, pos, *blocking_piece));
                        }
                        break;
                    },
                    None => {},
                }
            }
        }
    }
    unchecked_moves
}

pub fn provisional_moves_for_castle(game_state: &GameState, from_pos: BoardPosition, ruleset: &MoveRuleset, options: Option<MoveSearchOptions>) -> Vec<Move> {
    let options = options.unwrap_or_default();
    let mut unchecked_moves = Vec::new();
    let piece = game_state.board.get(from_pos).expect("expected piece at pos");
    let active_color = options.active_color_override.unwrap_or(game_state.active_color);
    let Some(castle_rights) = game_state.castle_rights.for_color(active_color)
        else { return unchecked_moves; };
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
                return unchecked_moves;
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
                        else { return unchecked_moves };
                    if blocking_piece.as_color() != piece.as_color() || blocking_piece.as_piece() != Piece::Rook {
                        return unchecked_moves;
                    }
                    // if starting pos requirement is set, check the rook as well
                    if ruleset.only_from_starting_pos && game_state.board.is_pos_starting_pos(pos) {
                        unchecked_moves.push(Move::create_castle(piece, from_pos, target_pos, castle_side));
                    }
                    break;
                }
            }
        }
    }
    unchecked_moves
}

pub fn provisional_moves_from_rulesets(game_state: &GameState, from_pos: BoardPosition, move_rulesets: &[MoveRuleset], options: Option<MoveSearchOptions>) -> Vec<Move> {
    let options = options.unwrap_or_default();
    let mut valid_moves = Vec::new();
    for ruleset in move_rulesets {
        if ruleset.only_from_starting_pos && !game_state.board.is_pos_starting_pos(from_pos) {
            continue;
        }
        let mut new_valid_moves = match ruleset.move_type {
            MoveDefType::Normal => provisional_moves_for_normal(game_state, from_pos, ruleset, Some(options)),
            MoveDefType::WhenCapturingOnly(capture_only_type) => provisional_moves_for_capture_only(game_state, from_pos, ruleset, capture_only_type, Some(options)),
            MoveDefType::Castle => provisional_moves_for_castle(game_state, from_pos, ruleset, Some(options)),
        };
        valid_moves.append(&mut new_valid_moves);
    }
    valid_moves
}

pub fn unchecked_move_search(game_state: &GameState, options: Option<MoveSearchOptions>) -> Vec<Move> {
    let mut unchecked_moves: Vec<Move> = Vec::new();
    for (pos, maybe_piece) in game_state.board.as_iter() {
        unchecked_moves.append(&mut unchecked_move_search_from_pos(game_state, pos, options))
    }
    unchecked_moves
}

pub fn unchecked_move_search_from_pos(game_state: &GameState, pos: BoardPosition, options: Option<MoveSearchOptions>) -> Vec<Move> {
    let options = options.unwrap_or_default();
    let maybe_piece = game_state.board.get(pos);
    let Some(piece) = maybe_piece else {
        return Vec::new();
    };
    let active_color = options.active_color_override.unwrap_or(game_state.active_color);
    // not this colors turn
    if !options.skip_active_color_check && piece.as_color() != active_color {
        return Vec::new();
    }
    match piece.as_move_set() {
        ChessPieceMoveSet::Set10(ms) => provisional_moves_from_rulesets(&game_state, pos, ms.move_rulesets.as_slice(), Some(options)),
        ChessPieceMoveSet::Set8(ms) => provisional_moves_from_rulesets(&game_state, pos, ms.move_rulesets.as_slice(), Some(options)),
        ChessPieceMoveSet::Set6(ms) => provisional_moves_from_rulesets(&game_state, pos, ms.move_rulesets.as_slice(), Some(options)),
        ChessPieceMoveSet::Set4(ms) => provisional_moves_from_rulesets(&game_state, pos, ms.move_rulesets.as_slice(), Some(options)),
    }
}

pub fn find_move(game_state: &GameState, from: BoardPosition, to: BoardPosition, options: Option<MoveSearchOptions>, promotion_piece: Option<PromotionPiece>) -> Result<Move, InvalidMoveError> {
    let provisional_moves = unchecked_move_search_from_pos(game_state, from, options);
    match provisional_moves.iter().find(|&m| {
        if m.from != from || m.to != to {
            false
        } else {
            match m.move_type {
                MoveType::Promotion(promotion) => promotion_piece.is_some() && promotion_piece.unwrap() == promotion,
                _ => promotion_piece.is_none(),
            }
        }
    }) {
        None => Err(InvalidMoveError::InvalidMove(from, to)),
        Some(matched_move) => Ok(matched_move.to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use itertools::Itertools;
    use rstest::rstest;
    use crate::board_position::BoardPosition;
    use crate::r#move::Move;
    use crate::position::*;
    use crate::chess_piece::ChessPiece;
    use crate::castle_side::CastleSide;
    use crate::invalid_move_error::InvalidMoveError;
    use crate::fen::{FEN_STARTING_POS, deserialize};
    use crate::utils::print_slice_elements_using_display;
    use super::*;

    fn sort_moves(a: &Move, b: &Move) -> Ordering {
        a.piece.partial_cmp(&b.piece).unwrap_or(Ordering::Equal)
            .then_with(|| a.from.partial_cmp(&b.from).unwrap_or(Ordering::Equal))
            .then_with(|| a.to.partial_cmp(&b.to).unwrap_or(Ordering::Equal))
            .then_with(|| a.move_type.partial_cmp(&b.move_type).unwrap_or(Ordering::Equal))
    }

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
    #[case("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3Kn1R w KQkq - 0 1", WHITE_KING_SIDE_ROOK_SQUARE, vec![
        Move::create_normal(ChessPiece::WhiteRook, WHITE_KING_SIDE_ROOK_SQUARE, WHITE_KING_SIDE_KNIGHT_SQUARE),
        Move::create_normal_capture(ChessPiece::WhiteRook, WHITE_KING_SIDE_ROOK_SQUARE, WHITE_KING_SIDE_BISHOP_SQUARE, ChessPiece::BlackKnight),
    ])]
    #[case("3k4/8/2p5/3Pp3/8/8/8/3K4 w - e6 0 1", D5, vec![
        Move::create_normal(ChessPiece::WhitePawn, D5, D6),
        Move::create_normal_capture(ChessPiece::WhitePawn, D5, C6, ChessPiece::BlackPawn),
        Move::create_en_passant(ChessPiece::WhitePawn, D5, E6, E5, ChessPiece::BlackPawn),
    ])]
    #[case("8/8/8/8/3N4/8/8/8 w - - 0 1", D4, vec![
        Move::create_normal(ChessPiece::WhiteKnight, D4, C6),
        Move::create_normal(ChessPiece::WhiteKnight, D4, E6),
        Move::create_normal(ChessPiece::WhiteKnight, D4, F5),
        Move::create_normal(ChessPiece::WhiteKnight, D4, F3),
        Move::create_normal(ChessPiece::WhiteKnight, D4, C2),
        Move::create_normal(ChessPiece::WhiteKnight, D4, E2),
        Move::create_normal(ChessPiece::WhiteKnight, D4, B3),
        Move::create_normal(ChessPiece::WhiteKnight, D4, B5),
    ])]
    fn test_unchecked_move_search_from_pos(
        #[case] fen_str: &'static str,
        #[case] pos: BoardPosition,
        #[case] expected: Vec<Move>,
    ) {
        let game_state = deserialize(fen_str).expect("bad fen string!");
        let expected = expected.into_iter().sorted_by(sort_moves).collect_vec();
        let unchecked_moves = unchecked_move_search_from_pos(&game_state, pos, None).into_iter().map(|m| m.to_owned()).sorted_by(sort_moves).collect_vec();
        if expected != unchecked_moves {
            println!("expected:"); print_slice_elements_using_display(&expected);
            println!("got:"); print_slice_elements_using_display(&unchecked_moves);
        }
        assert_eq!(expected, unchecked_moves)
    }

    #[rstest]
    #[case("rnb1kbnr/ppppqppp/8/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1", E7, vec![
        Move::create_normal(ChessPiece::BlackQueen, E7, D8),
        Move::create_normal(ChessPiece::BlackQueen, E7, E6),
        Move::create_normal(ChessPiece::BlackQueen, E7, E5),
        Move::create_normal(ChessPiece::BlackQueen, E7, E4),
        Move::create_normal(ChessPiece::BlackQueen, E7, E3),
        Move::create_normal(ChessPiece::BlackQueen, E7, E2),
        Move::create_normal_capture(ChessPiece::BlackQueen, E7, E1, ChessPiece::WhiteKing),
    ])]
    fn test_contains_unchecked_move_search_from_pos(
        #[case] fen_str: &'static str,
        #[case] pos: BoardPosition,
        #[case] contains_expected: Vec<Move>,
    ) {
        let game_state = deserialize(fen_str).expect("bad fen string!");
        let contains_expected = contains_expected.into_iter().sorted_by(sort_moves).collect_vec();
        let unchecked_moves = unchecked_move_search_from_pos(&game_state, pos, None).into_iter().filter(|m| contains_expected.contains(m)).sorted_by(sort_moves).collect_vec();
        if contains_expected != unchecked_moves {
            println!("expected:"); print_slice_elements_using_display(&contains_expected);
            println!("got:"); print_slice_elements_using_display(&unchecked_moves);
        }
        for expected_move in contains_expected.as_slice() {
            if unchecked_moves.contains(expected_move) {
                continue;
            }
            println!("missing: {expected_move}");
        }
        assert_eq!(contains_expected, unchecked_moves);
    }

    #[rstest]
    #[case(FEN_STARTING_POS, A2, A3, None, Ok(Move::create_normal(ChessPiece::WhitePawn, A2, A3)))]
    #[case("7k/P7/8/8/8/8/8/7K w - - 0 1", A7, A8, Some(PromotionPiece::Queen), Ok(Move::create_promotion(ChessPiece::WhitePawn, A7, A8, PromotionPiece::Queen)))]
    fn test_find_move(
        #[case] fen_str: &'static str,
        #[case] from: BoardPosition,
        #[case] to: BoardPosition,
        #[case] promotion_piece: Option<PromotionPiece>,
        #[case] expected: Result<Move, InvalidMoveError>,
    ) {
        let game_state = deserialize(fen_str).expect("bad fen string!");
        assert_eq!(expected, find_move(&game_state, from, to, None, promotion_piece))
    }

    #[rstest]
    #[case(FEN_STARTING_POS, A2, A5, InvalidMoveError::InvalidMove(A2, A5))]
    fn test_find_move_fail(
        #[case] fen_str: &'static str,
        #[case] from: BoardPosition,
        #[case] to: BoardPosition,
        #[case] expected: InvalidMoveError,
    ) {
        let game_state = deserialize(fen_str).expect("bad fen string!");
        let matched_move = find_move(&game_state, from, to, None, None);
        assert!(matched_move.is_err());
        assert_eq!(expected, matched_move.err().expect("expected error!"));
    }
}
