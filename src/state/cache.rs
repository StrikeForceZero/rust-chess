use std::cell::Cell;
use crate::bit_board::bit_board::BitBoard;
use crate::board::board_position::BoardPosition;
use crate::board::generic_board::GenericBoard;
use crate::chess_move::chess_move::ChessMove;

#[derive(Debug, Clone)]
pub struct ChessMoveWithEvalScore {
    chess_move: ChessMove,
    eval_score: Option<i32>,
}

impl ChessMoveWithEvalScore {
    pub fn from_chess_move(chess_move: ChessMove, eval_score: Option<i32>) -> Self {
        Self {
            chess_move,
            eval_score,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Cache {
    moves: GenericBoard<Vec<ChessMoveWithEvalScore>>,
    locked: BitBoard,
    blocked_attackers: BitBoard,
    attackers: BitBoard,
}

impl Cache {
    pub fn invalidate_pos(&mut self, pos: BoardPosition) {
        let Some(moves) = self.moves.get(pos) else {
            return;
        };
    }
    pub fn cache_for_pos(&mut self, pos: BoardPosition, chess_moves_with_eval_score: Vec<ChessMoveWithEvalScore>) {
        self.moves.replace(pos, Some(chess_moves_with_eval_score));
    }
}
