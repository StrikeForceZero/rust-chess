use thiserror::Error;
use crate::board_position::BoardPosition;
use crate::chess_piece::ChessPiece;
use crate::color::Color;
use crate::game_status::GameStatus;

#[derive(Error, Debug, Clone, Copy, PartialEq)]
pub enum  InvalidMoveError {
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
    #[error("Invalid Move: can't move into check")]
    MoveIntoCheck,
    #[error("Invalid Move: still in check")]
    StillInCheck,
    #[error("Invalid Move: {0} -> {1}")]
    InvalidMove(BoardPosition, BoardPosition),
}
