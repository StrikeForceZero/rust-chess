use crate::game_state::GameState;
use crate::game_status::GameStatus;

pub fn evaluate_game_state(game_state: &GameState) -> f64 {
    let mut score: f64 = 0_f64;
    for (pos, maybe_piece) in game_state.board.as_iter() {
        let Some(piece) = maybe_piece else { continue };
        score += piece.as_score() as f64 * pos.as_score();
    }
    score += match game_state.game_status {
        GameStatus::InProgress | GameStatus::New => 0_f64,
        GameStatus::Check(color) => 5_f64 * -color.as_score() as f64,
        GameStatus::CheckMate(color) => 999_f64 * -color.as_score() as f64,
        // encourage losing bots to go for stalemate/draw
        GameStatus::Stalemate | GameStatus::Draw => -score + score.signum(),
    };
    score
}
