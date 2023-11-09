use crate::color::Color;

#[derive(Copy, Clone)]
pub enum GameStatus {
    New,
    InProgress,
    Check(Color),
    CheckMate(Color),
    Stalemate,
    Draw,
}
