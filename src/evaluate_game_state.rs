use crate::game_state::GameState;
use crate::game_status::GameStatus;

pub fn evaluate_game_state(game_state: &GameState) -> i32 {
    let mut score: i32 = 0;
    for (pos, maybe_piece) in game_state.board.as_iter() {
        let Some(piece) = maybe_piece else { continue };
        let piece_score = piece.as_score();
        score += piece_score + piece_score * pos.as_score();
    }
    score += match game_state.game_status {
        GameStatus::InProgress | GameStatus::New => 0,
        GameStatus::Check(color) => 5 * -color.as_score(),
        GameStatus::CheckMate(color) => 999 * -color.as_score(),
        // encourage losing bots to go for stalemate/draw
        GameStatus::Stalemate | GameStatus::Draw => -score + score.signum(),
    };
    score
}
