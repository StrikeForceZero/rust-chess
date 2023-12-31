use crate::chess_move::chess_move::ChessMove;
use crate::chess_move::chess_move_handler::{try_handle_chess_move, ChessMoveHandlerOptions};
use crate::chess_move::chess_move_search::{
    unchecked_chess_move_search_from_pos, MoveSearchOptions,
};
use crate::color::Color;
use crate::piece::piece::Piece;
use crate::state::game_state::GameState;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GameStatus {
    New,
    InProgress,
    Check(Color),
    CheckMate(Color),
    Stalemate,
    Draw,
}

impl GameStatus {
    pub const fn is_check(&self) -> bool {
        match self {
            GameStatus::Check(_) => true,
            _ => false,
        }
    }

    pub const fn is_check_or_mate(&self) -> bool {
        match self {
            GameStatus::Check(_) => true,
            GameStatus::CheckMate(_) => true,
            _ => false,
        }
    }

    pub const fn is_mate(&self) -> bool {
        match self {
            GameStatus::CheckMate(_) => true,
            _ => false,
        }
    }

    pub const fn is_game_over(&self) -> bool {
        match self {
            GameStatus::CheckMate(_) => true,
            GameStatus::Draw => true,
            GameStatus::Stalemate => true,
            _ => false,
        }
    }
}

fn will_move_clear_check(game_state: &GameState, color: Color, move_to_test: &ChessMove) -> bool {
    let mut game_state_copy = game_state.clone();
    game_state_copy.game_status = GameStatus::InProgress;
    let move_hanlder_options = ChessMoveHandlerOptions {
        color_override: Some(color),
        skip_check_mate_check: true,
        skip_stale_mate_check: true,
        ..Default::default()
    };
    if let Ok(new_game_state) =
        try_handle_chess_move(&game_state_copy, &move_to_test, Some(move_hanlder_options))
    {
        if !new_game_state.game_status.is_check_or_mate() {
            return true;
        }
    }
    false
}

pub fn is_check(game_state: &GameState) -> bool {
    // already determined to be mate
    if game_state.game_status.is_mate() {
        return true;
    }
    // already determined to be check
    if game_state.game_status.is_check() {
        return true;
    }
    // no point in checking if game is over
    if game_state.game_status.is_game_over() {
        return false;
    }
    for (pos, maybe_piece) in game_state.board.as_iter() {
        let Some(chess_piece) = maybe_piece else {
            continue;
        };
        if chess_piece.as_color() == game_state.active_color {
            // only care about the color that last moved
            continue;
        }
        let move_search_options = MoveSearchOptions {
            active_color_override: Some(game_state.active_color.as_inverse()),
            ..Default::default()
        };
        let moves =
            unchecked_chess_move_search_from_pos(game_state, pos, Some(move_search_options));
        for provisional_move in moves {
            if let Some(captured_piece) = provisional_move.captured_piece {
                if captured_piece.as_piece() == Piece::King {
                    return true;
                }
            }
        }
    }
    false
}

pub fn is_check_for_color(game_state: &GameState, for_color: Color) -> bool {
    let mut game_state = game_state.clone();
    // only care about the color that last moved / attacking
    game_state.active_color = for_color;
    is_check(&game_state)
}

pub fn is_check_mate(game_state: &GameState) -> bool {
    // already determined to be mate
    if game_state.game_status.is_mate() {
        return true;
    }
    // cant be in mate if not in check
    if !game_state.game_status.is_check() {
        return false;
    }
    // no point in checking if game is over
    if game_state.game_status.is_game_over() {
        return false;
    }
    for (pos, maybe_piece) in game_state.board.as_iter() {
        let Some(chess_piece) = maybe_piece else {
            continue;
        };
        if chess_piece.as_color() != game_state.active_color {
            // only care about the color that is in check
            continue;
        }
        let moves = unchecked_chess_move_search_from_pos(game_state, pos, None);
        for provisional_move in moves {
            if will_move_clear_check(&game_state, game_state.active_color, &provisional_move) {
                return false;
            }
        }
    }
    true
}

pub fn is_stalemate(game_state: &GameState) -> bool {
    if game_state.game_status == GameStatus::Stalemate {
        return true;
    }
    // no point in checking if game is over
    if game_state.game_status.is_game_over() {
        return false;
    }
    for (pos, maybe_piece) in game_state.board.as_iter() {
        let Some(chess_piece) = maybe_piece else {
            continue;
        };
        if chess_piece.as_color() != game_state.active_color {
            // only care about the color to chess_move
            continue;
        }
        let moves = unchecked_chess_move_search_from_pos(game_state, pos, None);
        for provisional_move in moves {
            if will_move_clear_check(&game_state, game_state.active_color, &provisional_move) {
                return false;
            }
        }
    }
    true
}
