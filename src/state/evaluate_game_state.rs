use crate::chess_move::chess_move::ChessMove;
use crate::chess_move::chess_move_handler::try_handle_chess_move_and_apply;
use crate::chess_move::chess_move_search::unchecked_chess_move_search;
use crate::color::Color;
use crate::state::game_state::GameState;
use crate::state::game_status::GameStatus;

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
    // force to at least pick one legal chess_move no matter what
    /*if score == i32::MAX {
        score -= 1;
    } else if score == i32::MIN {
        score += 1
    }*/
    // println!("#{} {:?} - score: {score} {:?}", state.history.move_history.len(), state.history.move_history.last(), state.game_status);
    score
}

fn minimax_with_alpha_beta(
    game_state: &GameState,
    depth: u8,
    alpha: i32,
    beta: i32,
    maximizing_color: Color,
) -> i32 {
    if depth == 0 || game_state.game_status.is_game_over() {
        return evaluate_game_state(game_state, maximizing_color);
    }

    if maximizing_color == game_state.active_color {
        let mut max_eval = i32::MIN;
        for move_ in unchecked_chess_move_search(game_state, None) {
            let mut new_game_state = game_state.clone();
            if let Ok(_) = try_handle_chess_move_and_apply(&mut new_game_state, &move_, None) {
                let eval = minimax_with_alpha_beta(
                    &new_game_state,
                    depth - 1,
                    alpha,
                    beta,
                    maximizing_color,
                );
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
        for move_ in unchecked_chess_move_search(game_state, None) {
            let mut new_game_state = game_state.clone();
            if let Ok(_) = try_handle_chess_move_and_apply(&mut new_game_state, &move_, None) {
                let eval = minimax_with_alpha_beta(
                    &new_game_state,
                    depth - 1,
                    alpha,
                    beta,
                    maximizing_color,
                );
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

pub fn find_best_move(game_state: &GameState, depth: u8) -> Result<ChessMove, &'static str> {
    let mut best_move = None;
    let mut best_eval = i32::MIN;

    let maximizing_player_color = game_state.active_color;
    for move_ in unchecked_chess_move_search(game_state, None) {
        /*if state.history.move_history.len() >= 132 {
            println!("{:?}", move_);
        }*/
        let mut new_game_state = game_state.clone();
        let move_result = try_handle_chess_move_and_apply(&mut new_game_state, &move_, None);
        if let Ok(_) = move_result {
            let eval = minimax_with_alpha_beta(
                &new_game_state,
                depth - 1,
                i32::MIN,
                i32::MAX,
                maximizing_player_color,
            );

            if eval > best_eval {
                best_eval = eval;
                best_move = Some(move_);
            } else {
                // force to pick at least one legal chess_move
                best_move = best_move.or(Some(move_));
            }
            /* if state.history.move_history.len() >= 132 {
                println!("ok - {best_eval}");
            }*/
        } /* else if let Err(err) = move_result {
              if state.history.move_history.len() >= 132 {
                  println!("err - {best_eval} - {err:?}");
              }
          }*/
    }

    let Some(best_move) = best_move else {
        return Err("No legal moves available");
    };

    Ok(best_move)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;
    use crate::board::board_position::BoardPosition;
    use crate::board::position::*;
    use crate::chess_move::chess_move::ChessMoveType;
    use crate::notation::fen::{FEN_STARTING_POS, deserialize};
    use crate::piece::promotion_piece::PromotionPiece;
    use rstest::rstest;

    #[rstest]
    // TODO: requires depth of 4 which takes 30s
    #[case(FEN_STARTING_POS, E2, E4, None, Some(4), Some(Duration::from_secs(34)))]
    #[case("8/8/1R5p/1P2pkp1/7P/5KP1/1r6/8 w - - 0 1", G3, G4, None, None, Some(Duration::from_millis(31)))]
    #[case(
        "8/1P2R3/k7/8/1Q6/8/8/7K w - - 0 1",
        B7,
        B8,
        Some(PromotionPiece::Knight),
        None,
        Some(Duration::from_millis(32))
    )]
    fn test_find_best_move_first_move(
        #[case] fen_str: &'static str,
        #[case] expected_from: BoardPosition,
        #[case] expected_to: BoardPosition,
        #[case] expected_promotion: Option<PromotionPiece>,
        #[case] depth: Option<u8>,
        #[case] max_duration: Option<Duration>,
    ) -> Result<(), &'static str> {
        let game_state = deserialize(fen_str).expect("bad fen string!");
        let depth = depth.unwrap_or(2);
        let start = std::time::Instant::now();
        let best_move = find_best_move(&game_state, depth)?;
        let duration = start.elapsed();
        println!("Time taken: {:?}", duration);
        println!("{best_move} depth: {depth}");
        assert_eq!(expected_from, best_move.from);
        assert_eq!(expected_to, best_move.to);
        match best_move.move_type {
            ChessMoveType::Promotion(promotion) => {
                assert_eq!(expected_promotion, Some(promotion))
            }
            _ => assert_eq!(expected_promotion, None),
        }
        if let Some(max_duration) = max_duration {
            assert!(duration < max_duration);
        }
        Ok(())
    }
}
