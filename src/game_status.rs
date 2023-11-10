use crate::color::Color;
use crate::game_state::GameState;
use crate::move_search::unchecked_move_search_from_pos;
use crate::piece::Piece;

#[derive(Copy, Clone, PartialEq)]
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
        let Some(chess_piece) = maybe_piece else { continue };
        if chess_piece.as_color() == game_state.active_color {
            // only care about the color that last moved
            continue;
        }
        let moves = unchecked_move_search_from_pos(game_state, pos);
        for provisional_move in moves {
            if let Some(captured_piece) = provisional_move.captured_piece {
                if captured_piece.as_piece() == Piece::King {
                    return true;
                }
            }
        }
    }
    return false;
}

pub fn is_check_for_color(game_state: &GameState, for_color: Color) -> bool {
    let mut game_state = game_state.clone();
    // only care about the color that last moved / attacking
    game_state.active_color = for_color.as_inverse();
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
        let Some(chess_piece) = maybe_piece else { continue };
        if chess_piece.as_color() != game_state.active_color {
            // only care about the color that is in check
            continue;
        }
        let moves = unchecked_move_search_from_pos(game_state, pos);
        for provisional_move in moves {
            todo!("check if move removes check")
        }
    }
    return false;
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
        let Some(chess_piece) = maybe_piece else { continue };
        if chess_piece.as_color() != game_state.active_color {
            // only care about the color to move
            continue;
        }
        let moves = unchecked_move_search_from_pos(game_state, pos);
        for provisional_move in moves {
            // return false;
        }
        todo!("check if move puts into check");
    }
    return true;
}
