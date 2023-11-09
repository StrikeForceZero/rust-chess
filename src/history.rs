use std::collections::HashMap;
use crate::fen::{Fen, FEN_STARTING_POS};
use crate::full_color_piece_bit_board::FullColorPieceBitBoard;
use crate::move_history_entry::MoveHistoryEntry;

#[derive(Clone)]
pub struct History {
    pub move_history: Vec<MoveHistoryEntry>,
    // most moves in history 269
    pub state_history: HashMap<FullColorPieceBitBoard, u8>,
}

impl History {
    pub const fn empty() -> Self {
        Self {
            move_history: Vec::new(),
            state_history: HashMap::new(),
        }
    }
    pub const fn new() -> Self {
        Self {
            move_history: Vec::new(),
            state_history: HashMap::new(),
        }
    }
}
