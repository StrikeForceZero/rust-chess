use crate::color::Color;
use crate::full_piece_bit_board::FullPieceBitBoard;
use crate::utils::{CustomStructIterator, CustomStructIteratorMut};

pub struct FullColorPieceBitBoard {
    pub white: FullPieceBitBoard,
    pub black: FullPieceBitBoard,
}

impl FullColorPieceBitBoard {
    pub const fn as_iter(&self) -> CustomStructIterator<Self> {
        CustomStructIterator {
            data: self,
            index: 0,
        }
    }
}

impl<'a> Iterator for CustomStructIterator<'a, FullColorPieceBitBoard> {
    type Item = (Color, &'a FullPieceBitBoard);

    fn next(&mut self) -> Option<Self::Item> {
        let res = Some(match self.index {
            0 => (Color::White, &self.data.white),
            1 => (Color::Black, &self.data.black),
            _ => return None,
        });
        self.index += 1;
        res
    }
}
