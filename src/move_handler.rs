use thiserror::Error;
use crate::board_position::BoardPosition;
use crate::board_rank::BoardRank;
use crate::castle_rights::CastleRights;
use crate::castle_side::CastleSide;
use crate::chess_piece::ChessPiece;
use crate::color::Color;
use crate::game_state::GameState;
use crate::game_status::{GameStatus, is_check, is_check_for_color, is_check_mate, is_stalemate};
use crate::move_history_entry::MoveHistoryEntry;
use crate::piece::Piece;
use crate::r#move::{Move, MoveType};

#[derive(Error, Debug, Clone, Copy)]
pub enum  InvalidMoveError {
    #[error("Game is over")]
    GameOver,
    #[error("Invalid Move: No Piece at origin {0}")]
    NoPieceAtOrigin(BoardPosition),
    #[error("Invalid Move: {0:?}'s turn")]
    NotCurrentTurn(Color),
    #[error("Invalid Move: Expected capture: {0:?} but got {1:?}")]
    UnexpectedCapture(Option<ChessPiece>, Option<ChessPiece>),
    #[error("Invalid Move: can't castle out of check")]
    CastleOutOfCheck,
    #[error("Invalid Move: can't move into check")]
    MoveIntoCheck,
    #[error("Invalid Move: still in check")]
    StillInCheck,
}

fn move_unchecked(game_state: &mut GameState, from: BoardPosition, to: BoardPosition) -> Option<ChessPiece> {
    game_state.board.replace(to, *game_state.board.get(from))
}

pub fn default_move_handler(game_state: &mut GameState, requested_move: Move) -> Result<(), InvalidMoveError> {
    if game_state.game_status.is_game_over() {
        return Err(InvalidMoveError::GameOver);
    }

    // Scoping the immutable borrow
    let moving_piece_color;
    let moving_piece_type;
    let moving_piece_facing_direction;
    {
        let moving_piece = game_state.board.get(requested_move.from)
            .ok_or(InvalidMoveError::NoPieceAtOrigin(requested_move.from))?;
        moving_piece_color = moving_piece.as_color();
        moving_piece_type = moving_piece.as_piece();
        moving_piece_facing_direction = moving_piece.as_facing_direction();
    }
    if moving_piece_color != game_state.active_color {
        return Err(InvalidMoveError::NotCurrentTurn(game_state.active_color));
    }
    let is_in_check = game_state.game_status.is_check();
    let maybe_capture = match requested_move.move_type {
        MoveType::Castle(castle_side) => {
            if is_in_check {
                return Err(InvalidMoveError::CastleOutOfCheck);
            }
            let mut maybe_capture = None;
            let mut last_pos = requested_move.from;
            while last_pos != requested_move.to {
                if let Some(next_pos) = last_pos.next_pos(castle_side.as_simple_direction().as_direction()) {
                    maybe_capture = move_unchecked(game_state, requested_move.from, requested_move.from);
                    if maybe_capture != requested_move.captured_piece {
                        return Err(InvalidMoveError::UnexpectedCapture(requested_move.captured_piece, maybe_capture));
                    }
                    last_pos = next_pos;
                    if is_check_for_color(game_state, moving_piece_color) {
                        if is_in_check {
                            return Err(InvalidMoveError::StillInCheck);
                        }
                        return Err(InvalidMoveError::MoveIntoCheck);
                    }
                }
            }
            maybe_capture
        },
        _ => {
            let maybe_capture = move_unchecked(game_state, requested_move.from, requested_move.from);
            if maybe_capture != requested_move.captured_piece {
                return Err(InvalidMoveError::UnexpectedCapture(requested_move.captured_piece, maybe_capture));
            }
            if is_check_for_color(game_state, moving_piece_color) {
                if is_in_check {
                    return Err(InvalidMoveError::StillInCheck);
                }
                return Err(InvalidMoveError::MoveIntoCheck);
            }
            maybe_capture
        },
    };
    if moving_piece_type == Piece::Pawn {
        if game_state.board.is_pos_starting_pos(requested_move.from) && (*requested_move.to.rank() == BoardRank::Four || *requested_move.to.rank() == BoardRank::Five) {
            game_state.en_passant_target_pos = requested_move.to.next_pos(moving_piece_facing_direction.as_simple_direction().as_direction().reverse());
        }
        game_state.move_clock.half_move = 0;
    }
    else {
        game_state.move_clock.half_move += 1;
    }

    if game_state.active_color == Color::Black {
        game_state.move_clock.full_move += 1;
    }

    if moving_piece_type == Piece::King && game_state.board.is_pos_starting_pos(requested_move.from) {
        // remove castle rights when the king moves
        game_state.castle_rights.for_color_mut(game_state.active_color).take();
    }

    if moving_piece_type == Piece::Rook && game_state.board.is_pos_starting_pos(requested_move.from) {
        // empty out the castle rights
        if let Some(castle_rights) = game_state.castle_rights.for_color_mut(game_state.active_color).take() {
            // re apply new rights, if any
            *game_state.castle_rights.for_color_mut(game_state.active_color) = castle_rights.without(CastleRights::from_castle_side(CastleSide::from_pos(requested_move.from)));
        }
    }

    game_state.active_color = game_state.active_color.as_inverse();

    if is_check(game_state) {
        game_state.game_status = GameStatus::Check(moving_piece_color.as_inverse());
        if is_check_mate(game_state) {
            game_state.game_status = GameStatus::CheckMate(moving_piece_color.as_inverse());
        }
    }

    if is_stalemate(game_state) {
        game_state.game_status = GameStatus::Stalemate;
    }

    if !game_state.game_status.is_game_over() {
        game_state.history.move_history.push(MoveHistoryEntry::from_move(requested_move));
        let seen_state_count = game_state.history.state_history.as_mut().expect("missing state history").increment(game_state.board.as_bit_boards_const());

        // 3 fold repetition
        if seen_state_count >= 3 {
            game_state.game_status = GameStatus::Draw;
        }
    }

    Ok(())
}
