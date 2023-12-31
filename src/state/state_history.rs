use crate::bit_board::full_color_piece_bit_board::FullColorPieceBitBoard;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum StateHistoryContainer {
    New(FullColorPieceBitBoard),
    Hash(HashMap<FullColorPieceBitBoard, u8>),
}

impl StateHistoryContainer {
    pub const fn new(full_color_piece_bit_board: FullColorPieceBitBoard) -> Self {
        Self::New(full_color_piece_bit_board)
    }
    pub fn upgrade(&mut self) {
        match self {
            StateHistoryContainer::New(first_entry) => {
                let mut map = HashMap::new();
                map.insert(first_entry.to_owned(), 1);
                *self = Self::Hash(map)
            }
            StateHistoryContainer::Hash(_) => {}
        }
    }
    pub fn increment(&mut self, full_color_piece_bit_board: FullColorPieceBitBoard) -> u8 {
        match self {
            StateHistoryContainer::New(_) => {
                self.upgrade();
                self.increment(full_color_piece_bit_board)
            }
            StateHistoryContainer::Hash(map) => {
                let entry = map.entry(full_color_piece_bit_board).or_insert(0);
                *entry += 1;
                *entry
            }
        }
    }
    pub fn clear(&mut self) {
        match self {
            StateHistoryContainer::New(_) => {
                self.upgrade();
                self.clear()
            }
            StateHistoryContainer::Hash(map) => {
                map.clear();
            }
        }
    }
}
