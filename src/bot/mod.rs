#[cfg(test)]
mod tests {
    use crate::chess_move::chess_move_handler::default_chess_move_handler;
    use crate::notation::fen::serialize;
    use crate::state::evaluate_game_state::find_best_move;
    use crate::state::game_state::GameState;

    #[test]
    fn bot_vs_bot() {
        let mut game_state = GameState::new();
        while !game_state.game_status.is_game_over() {
            println!(
                "#{} {:?} {:?} - last chess_move: {:?}",
                game_state.history.move_history.len(),
                game_state.game_status,
                game_state.active_color,
                game_state.history.move_history.last()
            );
            println!("fen: {}", serialize(&game_state));
            let best_move = match find_best_move(&game_state, 2) {
                Ok(best_move) => best_move,
                Err(message) => {
                    println!("error: {message}");
                    println!("fen: {}", serialize(&game_state));
                    assert!(game_state.game_status.is_game_over());
                    break;
                }
            };
            if let Err(err) = default_chess_move_handler(&mut game_state, &best_move, None) {
                panic!("{err:?}");
            }
        }
        println!(
            "#{} {:?} {:?} - last chess_move: {:?}",
            game_state.history.move_history.len(),
            game_state.game_status,
            game_state.active_color,
            game_state.history.move_history.last()
        );
    }
}
