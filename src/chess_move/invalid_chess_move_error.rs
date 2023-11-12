use crate::board::board_position::BoardPosition;
use crate::color::Color;
use crate::piece::chess_piece::ChessPiece;
use crate::state::game_status::GameStatus;
use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq)]
pub enum InvalidChessMoveError {
    #[error("Game is over: {0:?}")]
    GameOver(GameStatus),
    #[error("Invalid Move: No Piece at origin {0}")]
    NoPieceAtOrigin(BoardPosition),
    #[error("Invalid Move: {0:?}'s turn")]
    NotCurrentTurn(Color),
    #[error("Invalid Move: Expected capture: {0:?} but got {1:?}")]
    UnexpectedCapture(Option<ChessPiece>, Option<ChessPiece>),
    #[error("Invalid Move: can't castle when in check")]
    CastleWhileInCheck,
    #[error("Invalid Move: can't chess_move into check")]
    MoveIntoCheck,
    #[error("Invalid Move: still in check")]
    StillInCheck,
    #[error("Invalid Move: {0} -> {1}")]
    InvalidMove(BoardPosition, BoardPosition),
}
