use crate::board::Board;
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
    pub castle_rights: ColorCastleRights,
    pub active_color: Color,
    pub game_status: GameStatus,
}
