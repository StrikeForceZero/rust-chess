#[cfg(test)]
mod tests {
    use crate::evaluate_game_state::find_best_move;
    use crate::fen::serialize;
    use crate::game_state::GameState;
    use crate::move_handler::default_move_handler;

    #[test]
    fn bot_vs_bot() {
        let mut game_state = GameState::new();
        while !game_state.game_status.is_game_over() {
            println!("#{} {:?} {:?} - last move: {:?}", game_state.history.move_history.len(), game_state.game_status, game_state.active_color, game_state.history.move_history.last());
            println!("fen: {}", serialize(&game_state));
            if game_state.history.move_history.len() == 132 {
                println!("debugger");
            }
            let best_move = match find_best_move(&game_state, 2) {
                Ok(best_move) => best_move,
                Err(message) => {
                    println!("error: {message}");
                    println!("fen: {}", serialize(&game_state));
                    assert!(game_state.game_status.is_game_over());
                    break;
                }
            };
            if let Err(err) = default_move_handler(&mut game_state, &best_move, None) {
                panic!("{err:?}");
            }
        }
        println!("#{} {:?} {:?} - last move: {:?}", game_state.history.move_history.len(), game_state.game_status, game_state.active_color, game_state.history.move_history.last());
    }
}
