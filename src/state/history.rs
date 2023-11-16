use crate::board::board::Board;
use crate::state::move_history_entry::MoveHistoryEntry;
use crate::state::state_history::StateHistoryContainer;

#[derive(Clone, Debug)]
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
            state_history: Some(StateHistoryContainer::New(
                Board::new().as_bit_boards_const(),
            )),
        }
    }
}

// TODO: might not want starting position for default but it might be required by bevy?
impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}
