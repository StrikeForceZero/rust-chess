use crate::color::Color;
use crate::game_state::GameState;
use crate::game_status::GameStatus;
use crate::move_handler::try_handle_move_and_apply;
use crate::move_search::unchecked_move_search;
use crate::r#move::Move;



pub fn evaluate_game_state(game_state: &GameState, maximizing_color: Color) -> i32 {
    let mut score: i32 = 0;
    for (pos, maybe_piece) in game_state.board.as_iter() {
        let Some(piece) = maybe_piece else { continue };
        let piece_score = piece.as_piece().as_score() * piece.as_color().as_score(maximizing_color);
        let piece_pos_score = piece_score + pos.as_score();
        score += piece_pos_score + piece_pos_score;
    }
    score = score.saturating_add(match game_state.game_status {
        GameStatus::InProgress | GameStatus::New => 0,
        GameStatus::Check(color) => 99 * -color.as_score(maximizing_color),
        GameStatus::CheckMate(color) => i32::MAX * -color.as_score(maximizing_color),
        // encourage losing bots to go for stalemate/draw
        GameStatus::Stalemate | GameStatus::Draw => -score + score.signum(),
    });
    // println!("#{} {:?} - score: {score} {:?}", game_state.history.move_history.len(), game_state.history.move_history.last(), game_state.game_status);
    score
}

fn minimax_with_alpha_beta(
    game_state: &GameState, depth: u8, alpha: i32, beta: i32, maximizing_color: Color
) -> i32 {
    if depth == 0 || game_state.game_status.is_game_over() {
        return evaluate_game_state(game_state, maximizing_color);
    }

    if maximizing_color == game_state.active_color {
        let mut max_eval = i32::MIN;
        for move_ in unchecked_move_search(game_state, None) {
            let mut new_game_state = game_state.clone();
            if let Ok(_) = try_handle_move_and_apply(&mut new_game_state, &move_, None) {
                let eval = minimax_with_alpha_beta(&new_game_state, depth - 1, alpha, beta, maximizing_color);
                max_eval = max_eval.max(eval);

                if max_eval >= beta {
                    // println!("prune beta: {max_eval}>={beta}");
                    break; // Beta cutoff
                }
            }
        }
        max_eval
    } else {
        let mut min_eval = i32::MAX;
        for move_ in unchecked_move_search(game_state, None) {
            let mut new_game_state = game_state.clone();
            if let Ok(_) = try_handle_move_and_apply(&mut new_game_state, &move_, None) {
                let eval = minimax_with_alpha_beta(&new_game_state, depth - 1, alpha, beta, maximizing_color);
                min_eval = min_eval.min(eval);

                if min_eval <= alpha {
                    // println!("prune alpha: {min_eval}>={alpha}");
                    break; // Alpha cutoff
                }
            }
        }
        min_eval
    }
}

fn find_best_move(game_state: &GameState, depth: u8) -> Move {
    let mut best_move = None;
    let mut best_eval = i32::MIN;

    let maximizing_player_color = game_state.active_color;
    for move_ in unchecked_move_search(game_state, None) {
        let mut new_game_state = game_state.clone();
        if let Ok(_) = try_handle_move_and_apply(&mut new_game_state, &move_, None) {
            let eval = minimax_with_alpha_beta(&new_game_state, depth - 1, i32::MIN, i32::MAX, maximizing_player_color);

            if eval > best_eval {
                best_eval = eval;
                best_move = Some(move_);
            }
        }
    }

    best_move.expect("No legal moves available")
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::board_position::BoardPosition;
    use super::*;
    use crate::position::*;
    use crate::fen::{FEN_STARTING_POS, deserialize};
    use crate::promotion_piece::PromotionPiece;
    use crate::r#move::MoveType;

    #[rstest]
    // TODO: requires depth of 4 which takes 30s
    // #[case(FEN_STARTING_POS, E2, E4, None)]
    #[case("8/8/1R5p/1P2pkp1/7P/5KP1/1r6/8 w - - 0 1", G3, G4, None)]
    #[case("8/1P2R3/k7/8/1Q6/8/8/7K w - - 0 1", B7, B8, Some(PromotionPiece::Knight))]
    fn test__find_best_move__first_move(
        #[case] fen_str: &'static str,
        #[case] expected_from: BoardPosition,
        #[case] expected_to: BoardPosition,
        #[case] expected_promotion: Option<PromotionPiece>,
    ) {
        let game_state = deserialize(fen_str).expect("bad fen string!");
        let start = std::time::Instant::now();
        let best_move = find_best_move(&game_state, 2);
        let duration = start.elapsed();
        println!("Time taken: {:?}", duration);
        println!("{best_move}");
        assert_eq!(expected_from, best_move.from);
        assert_eq!(expected_to, best_move.to);
        match best_move.move_type {
            MoveType::Promotion(promotion) => {
                assert_eq!(expected_promotion, Some(promotion))
            },
            _ => assert_eq!(expected_promotion, None),
        }
    }
}
