use crate::board::board_file::BoardFile;
use crate::board::board_position::BoardPosition;
use crate::board::board_rank::BoardRank;
use crate::chess_move::chess_move::{ChessMove, ChessMoveType};
use crate::chess_move::invalid_chess_move_error::InvalidChessMoveError;
use crate::color::Color;
use crate::direction::castle_side::CastleSide;
use crate::piece::chess_piece::ChessPiece;
use crate::piece::piece::Piece;
use crate::state::castle_rights::CastleRights;
use crate::state::game_state::GameState;
use crate::state::game_status::{
    is_check, is_check_for_color, is_check_mate, is_stalemate, GameStatus,
};
use crate::state::move_history_entry::MoveHistoryEntry;

fn chess_move_unchecked(
    game_state: &mut GameState,
    from: BoardPosition,
    to: BoardPosition,
) -> Option<ChessPiece> {
    let moving_piece = game_state.board.get_mut(from).take();
    game_state.game_status = GameStatus::InProgress;
    game_state.board.replace(to, moving_piece)
}

#[derive(Copy, Clone, Default)]
pub struct ChessMoveHandlerOptions {
    pub color_override: Option<Color>,
    pub skip_updating_game_status: bool,
    pub skip_check_mate_check: bool,
    pub skip_stale_mate_check: bool,
}

pub fn default_chess_move_handler(
    game_state: &mut GameState,
    requested_chess_move: &ChessMove,
    options: Option<ChessMoveHandlerOptions>,
) -> Result<(), InvalidChessMoveError> {
    if game_state.game_status.is_game_over() {
        return Err(InvalidChessMoveError::GameOver(game_state.game_status));
    }

    let options = options.unwrap_or_default();
    let active_color = options.color_override.unwrap_or(game_state.active_color);

    // Scoping the immutable borrow
    let moving_piece_color;
    let moving_piece_type;
    let moving_piece_facing_direction;
    {
        let moving_piece = game_state.board.get(requested_chess_move.from).ok_or(
            InvalidChessMoveError::NoPieceAtOrigin(requested_chess_move.from),
        )?;
        moving_piece_color = moving_piece.as_color();
        moving_piece_type = moving_piece.as_piece();
        moving_piece_facing_direction = moving_piece.as_facing_direction();
    }
    if moving_piece_color != active_color {
        return Err(InvalidChessMoveError::NotCurrentTurn(active_color));
    }
    let was_at_starting_pos = game_state
        .board
        .is_pos_starting_pos(requested_chess_move.from);
    let is_in_check = is_check_for_color(game_state, moving_piece_color);
    let maybe_capture = match requested_chess_move.move_type {
        ChessMoveType::Castle(castle_side) => {
            if is_in_check {
                return Err(InvalidChessMoveError::CastleWhileInCheck);
            }
            let mut maybe_capture: Option<ChessPiece>;
            let mut last_pos = requested_chess_move.from;
            while last_pos != requested_chess_move.to {
                if let Some(next_pos) =
                    last_pos.next_pos(castle_side.as_simple_direction().as_direction())
                {
                    maybe_capture = chess_move_unchecked(game_state, last_pos, next_pos);
                    // shouldn't have replaced any pieces a long the way
                    if maybe_capture != requested_chess_move.captured_piece {
                        return Err(InvalidChessMoveError::UnexpectedCapture(
                            requested_chess_move.captured_piece,
                            maybe_capture,
                        ));
                    }
                    last_pos = next_pos;
                    if is_check_for_color(game_state, moving_piece_color) {
                        if is_in_check {
                            return Err(InvalidChessMoveError::StillInCheck);
                        }
                        return Err(InvalidChessMoveError::MoveIntoCheck);
                    }
                }
            }
            // handle moving the rook
            let Some(rook_end_pos) =
                last_pos.next_pos(castle_side.as_simple_direction().as_direction().reverse())
            else {
                panic!("bad game state or requested chess_move")
            };
            let Some(mut rook_start_pos) =
                last_pos.next_pos(castle_side.as_simple_direction().as_direction())
            else {
                panic!("bad game state or requested chess_move");
            };
            // go west one square for queen side rook
            if *rook_start_pos.file() == BoardFile::B {
                rook_start_pos = BoardPosition(BoardFile::A, *rook_start_pos.rank());
            }
            maybe_capture = chess_move_unchecked(game_state, rook_start_pos, rook_end_pos);
            // shouldn't have replaced any pieces
            if maybe_capture != requested_chess_move.captured_piece {
                return Err(InvalidChessMoveError::UnexpectedCapture(
                    requested_chess_move.captured_piece,
                    maybe_capture,
                ));
            }
            maybe_capture
        }
        ChessMoveType::EnPassant(capture_pos) => {
            let mut maybe_capture = chess_move_unchecked(
                game_state,
                requested_chess_move.from,
                requested_chess_move.to,
            );
            if maybe_capture.is_some() {
                return Err(InvalidChessMoveError::UnexpectedCapture(
                    requested_chess_move.captured_piece,
                    maybe_capture,
                ));
            }
            maybe_capture = game_state.board.replace(capture_pos, None);
            if maybe_capture != requested_chess_move.captured_piece {
                return Err(InvalidChessMoveError::UnexpectedCapture(
                    requested_chess_move.captured_piece,
                    maybe_capture,
                ));
            }
            if is_check_for_color(game_state, moving_piece_color) {
                if is_in_check {
                    return Err(InvalidChessMoveError::StillInCheck);
                }
                return Err(InvalidChessMoveError::MoveIntoCheck);
            }
            maybe_capture
        }
        ChessMoveType::Promotion(promote_to) => {
            let maybe_capture = chess_move_unchecked(
                game_state,
                requested_chess_move.from,
                requested_chess_move.to,
            );
            game_state.board.replace(
                requested_chess_move.to,
                Some(promote_to.as_piece().as_chess_piece(moving_piece_color)),
            );
            if maybe_capture != requested_chess_move.captured_piece {
                return Err(InvalidChessMoveError::UnexpectedCapture(
                    requested_chess_move.captured_piece,
                    maybe_capture,
                ));
            }
            if is_check_for_color(game_state, moving_piece_color) {
                if is_in_check {
                    return Err(InvalidChessMoveError::StillInCheck);
                }
                return Err(InvalidChessMoveError::MoveIntoCheck);
            }
            maybe_capture
        }
        _ => {
            let maybe_capture = chess_move_unchecked(
                game_state,
                requested_chess_move.from,
                requested_chess_move.to,
            );
            if maybe_capture != requested_chess_move.captured_piece {
                return Err(InvalidChessMoveError::UnexpectedCapture(
                    requested_chess_move.captured_piece,
                    maybe_capture,
                ));
            }
            if is_check_for_color(game_state, moving_piece_color) {
                if is_in_check {
                    return Err(InvalidChessMoveError::StillInCheck);
                }
                return Err(InvalidChessMoveError::MoveIntoCheck);
            }
            maybe_capture
        }
    };
    // clear en passant so future simulations won't fail trying to capture a piece that doesnt exist.
    game_state.en_passant_target_pos.take();
    // TODO: technically redundant
    if maybe_capture != requested_chess_move.captured_piece {
        return Err(InvalidChessMoveError::UnexpectedCapture(
            requested_chess_move.captured_piece,
            maybe_capture,
        ));
    }
    if moving_piece_type == Piece::Pawn || maybe_capture.is_some() {
        if moving_piece_type == Piece::Pawn
            && was_at_starting_pos
            && (*requested_chess_move.to.rank() == BoardRank::Four
                || *requested_chess_move.to.rank() == BoardRank::Five)
        {
            game_state.en_passant_target_pos = requested_chess_move.to.next_pos(
                moving_piece_facing_direction
                    .as_simple_direction()
                    .as_direction()
                    .reverse(),
            );
        }
        // reset since its impossible to revisit any states in the past after a pawn chess_move / capture
        game_state
            .history
            .state_history
            .as_mut()
            .expect("missing state history")
            .clear();
        game_state.move_counter.half_move = 0;
    } else {
        game_state.move_counter.half_move += 1;
    }

    if active_color == Color::Black {
        game_state.move_counter.full_move += 1;
    }

    if moving_piece_type == Piece::King && was_at_starting_pos {
        // remove castle rights when the king moves
        game_state.castle_rights.for_color_mut(active_color).take();
    }

    if moving_piece_type == Piece::Rook && was_at_starting_pos {
        // empty out the castle rights
        if let Some(castle_rights) = game_state.castle_rights.for_color_mut(active_color).take() {
            // re apply new rights, if any
            *game_state.castle_rights.for_color_mut(active_color) = castle_rights.without(
                CastleRights::from_castle_side(CastleSide::from_pos(requested_chess_move.from)),
            );
        }
    }

    game_state.active_color = active_color.as_inverse();

    if !options.skip_updating_game_status {
        if is_check(game_state) {
            game_state.game_status = GameStatus::Check(moving_piece_color.as_inverse());
            if !options.skip_check_mate_check && is_check_mate(game_state) {
                game_state.game_status = GameStatus::CheckMate(moving_piece_color.as_inverse());
            }
        }

        if !options.skip_stale_mate_check && is_stalemate(game_state) {
            game_state.game_status = GameStatus::Stalemate;
        }

        if !game_state.game_status.is_game_over() {
            game_state
                .history
                .move_history
                .push(MoveHistoryEntry::from_move(requested_chess_move));
            let seen_state_count = game_state
                .history
                .state_history
                .as_mut()
                .expect("missing state history")
                .increment(game_state.board.as_bit_boards_const());

            // 3 fold repetition
            if seen_state_count >= 3 {
                game_state.game_status = GameStatus::Draw;
            }
        }
    }

    Ok(())
}

pub fn try_handle_chess_move(
    game_state: &GameState,
    requested_move: &ChessMove,
    options: Option<ChessMoveHandlerOptions>,
) -> Result<GameState, InvalidChessMoveError> {
    let mut game_state_copy = game_state.clone();
    default_chess_move_handler(&mut game_state_copy, requested_move, options)?;
    Ok(game_state_copy)
}

pub fn try_handle_chess_move_and_apply(
    game_state: &mut GameState,
    requested_move: &ChessMove,
    options: Option<ChessMoveHandlerOptions>,
) -> Result<(), InvalidChessMoveError> {
    *game_state = try_handle_chess_move(&game_state, requested_move, options)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::position::*;
    use crate::chess_move::chess_move_search::find_move;
    use crate::notation::fen::{deserialize, serialize, FEN_STARTING_POS};
    use crate::piece::promotion_piece::PromotionPiece;
    use rstest::rstest;

    #[rstest]
    #[case(FEN_STARTING_POS, A2, A3, Ok(()))]
    #[case(FEN_STARTING_POS, A2, A4, Ok(()))]
    #[case(
        "rnb1kbnr/ppppqppp/8/8/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1",
        E1,
        E2,
        Err(InvalidChessMoveError::StillInCheck)
    )]
    #[case(
        "rnb1kbnr/ppppqppp/8/8/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1",
        D2,
        D3,
        Err(InvalidChessMoveError::StillInCheck)
    )]
    #[case("rnb1kbnr/ppppqppp/8/8/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1", D1, E2, Ok(()))]
    #[case("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w KQkq - 0 1", E1, F1, Ok(()))]
    #[case(
        "rnbqkbnr/pppppppp/4q3/8/8/8/PPPP2PP/RNBQK2R w KQkq - 1 1",
        E1,
        G1,
        Err(InvalidChessMoveError::CastleWhileInCheck)
    )]
    #[case(
        "rnbqkbnr/pppppppp/5q2/8/8/8/PPPPP1PP/RNBQK2R w KQkq - 0 1",
        E1,
        G1,
        Err(InvalidChessMoveError::MoveIntoCheck)
    )]
    #[case("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 1", E5, D6, Ok(()))]
    fn test_try_handle_move(
        #[case] fen_str: &'static str,
        #[case] from: BoardPosition,
        #[case] to: BoardPosition,
        #[case] expected: Result<(), InvalidChessMoveError>,
    ) -> Result<(), InvalidChessMoveError> {
        let game_state = deserialize(fen_str).expect("bad fen string!");
        println!("{from} -> {to}");
        let matched_move = find_move(&game_state, from, to, None, None)?;
        match try_handle_chess_move(&game_state, &matched_move, None) {
            Ok(gs) => {
                println!("{}", serialize(&gs));
                assert_eq!(expected, Ok(()))
            }
            Err(err) => assert_eq!(expected, Err(err)),
        }
        Ok(())
    }

    #[rstest]
    #[case(FEN_STARTING_POS, A2, A3, Ok(()))]
    fn test_try_handle_move_and_apply(
        #[case] fen_str: &'static str,
        #[case] from: BoardPosition,
        #[case] to: BoardPosition,
        #[case] expected: Result<(), InvalidChessMoveError>,
    ) -> Result<(), InvalidChessMoveError> {
        let mut game_state = deserialize(fen_str).expect("bad fen string!");
        let matched_move = find_move(&game_state, from, to, None, None)?;
        assert_eq!(
            expected,
            try_handle_chess_move_and_apply(&mut game_state, &matched_move, None)
        );
        Ok(())
    }

    #[rstest]
    #[case(
        "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 1",
        E5,
        D6,
        "rnbqkbnr/ppp1pppp/3P4/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
    )]
    #[case(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w KQkq - 0 1",
        E1,
        G1,
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQ1RK1 b kq - 1 1"
    )]
    fn test_state_change(
        #[case] fen_str: &'static str,
        #[case] from: BoardPosition,
        #[case] to: BoardPosition,
        #[case] expected: &'static str,
    ) -> Result<(), InvalidChessMoveError> {
        let mut game_state = deserialize(fen_str).expect("bad fen string!");
        let matched_move = find_move(&game_state, from, to, None, None)?;
        try_handle_chess_move_and_apply(&mut game_state, &matched_move, None)?;
        assert_eq!(expected, serialize(&game_state).get_str());
        Ok(())
    }

    #[rstest]
    #[case(
        "8/1P2R3/k7/8/1Q6/8/8/7K w - - 0 1",
        B7,
        B8,
        PromotionPiece::Knight,
        "1N6/4R3/k7/8/1Q6/8/8/7K b - - 0 1"
    )]
    fn test_state_change_promotion(
        #[case] fen_str: &'static str,
        #[case] from: BoardPosition,
        #[case] to: BoardPosition,
        #[case] promotion_piece: PromotionPiece,
        #[case] expected: &'static str,
    ) -> Result<(), InvalidChessMoveError> {
        let mut game_state = deserialize(fen_str).expect("bad fen string!");
        let matched_move = find_move(&game_state, from, to, None, Some(promotion_piece))?;
        try_handle_chess_move_and_apply(&mut game_state, &matched_move, None)?;
        assert_eq!(expected, serialize(&game_state).get_str());
        Ok(())
    }
}
