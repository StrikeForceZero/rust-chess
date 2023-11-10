use crate::game_state::GameState;
use crate::move_search::unchecked_move_search_from_pos;

pub fn evaluate_check(game_state: &GameState) -> bool {
    for (pos, maybe_piece) in game_state.board.as_iter() {
        let Some(chess_piece) = maybe_piece else { continue };
        if chess_piece.as_color() == game_state.active_color {
            // only care about the color that last moved
            continue;
        }
        let moves = unchecked_move_search_from_pos(game_state, pos);
        for provisional_move in moves {
            
        }
    }
    todo!("not implemented")
}

pub fn evaluate_check_mate(game_state: &GameState) -> bool {
    todo!("not implemented")
}
