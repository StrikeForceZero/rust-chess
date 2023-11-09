use std::collections::HashMap;
use crate::board::Board;
use crate::move_history_entry::MoveHistoryEntry;
use crate::state_history::StateHistoryContainer;

#[derive(Clone)]
pub struct History {
    pub move_history: Vec<MoveHistoryEntry>,
    // most moves in history 269
    pub state_history: Option<StateHistoryContainer>,
}

impl History {
    pub const fn empty() -> Self {
        Self {
            move_history: Vec::new(),
            state_history: None,
        }
    }
    pub const fn new() -> Self {
        Self {
            move_history: Vec::new(),
            state_history: Some(StateHistoryContainer::New(Board::new().as_bit_boards_const())),
        }
    }
}
