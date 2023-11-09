use crate::board::Board;
use crate::board_position::BoardPosition;
use crate::color::Color;
use crate::color_castle_rights::ColorCastleRights;
use crate::game_status::GameStatus;
use crate::history::History;
use crate::move_clock::MoveClock;

#[derive(Clone)]
pub struct GameState {
    pub board: Board,
    pub history: History,
    pub move_clock: MoveClock,
    pub en_passant_target_pos: Option<BoardPosition>,
    pub castle_rights: ColorCastleRights,
    pub active_color: Color,
    pub game_status: GameStatus,
}

impl GameState {
    pub const fn new() -> Self {
        Self {
            board: Board::new(),
            history: History::new(),
            move_clock: MoveClock::new(),
            en_passant_target_pos: None,
            castle_rights: ColorCastleRights::empty(),
            active_color: Color::White,
            game_status: GameStatus::New,
        }
    }
}
