use crate::board::board::Board;
use crate::board::board_position::BoardPosition;
use crate::color::Color;
use crate::state::cache::Cache;
use crate::state::color_castle_rights::ColorCastleRights;
use crate::state::game_status::GameStatus;
use crate::state::history::History;
use crate::state::move_counter::MoveCounter;

#[derive(Clone, Debug)]
pub struct GameState {
    pub board: Board,
    pub history: History,
    pub move_counter: MoveCounter,
    pub en_passant_target_pos: Option<BoardPosition>,
    pub castle_rights: ColorCastleRights,
    pub active_color: Color,
    pub game_status: GameStatus,
    pub cache: Option<Cache>,
}

impl GameState {
    pub const fn empty() -> Self {
        Self {
            board: Board::empty(),
            history: History::empty(),
            move_counter: MoveCounter::new(),
            en_passant_target_pos: None,
            castle_rights: ColorCastleRights::empty(),
            active_color: Color::White,
            game_status: GameStatus::New,
            cache: None,
        }
    }

    pub const fn new() -> Self {
        Self {
            board: Board::new(),
            history: History::new(),
            move_counter: MoveCounter::new(),
            en_passant_target_pos: None,
            castle_rights: ColorCastleRights::new(),
            active_color: Color::White,
            game_status: GameStatus::New,
            cache: None,
        }
    }
}
