use std::collections::HashMap;
use crate::full_color_piece_bit_board::FullColorPieceBitBoard;

#[derive(Clone)]
pub enum StateHistoryContainer {
    New(FullColorPieceBitBoard),
    Hash(HashMap<FullColorPieceBitBoard, u8>)
}

impl StateHistoryContainer {
    pub const fn new(full_color_piece_bit_board: FullColorPieceBitBoard) -> Self {
        Self::New(full_color_piece_bit_board)
    }
    pub fn upgrade(&mut self)  {
        match self {
            StateHistoryContainer::New(first_entry) => {
                let mut map = HashMap::new();
                map.insert(first_entry.to_owned(), 1);
                *self = Self::Hash(map)
            }
            StateHistoryContainer::Hash(_) => {}
        }
    }
}
