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
        Some(match self.index {
            0 => (Color::White, &self.data.white),
            1 => (Color::Black, &self.data.black),
            _ => return None,
        })
    }
}

impl<'a> Iterator for CustomStructIteratorMut<'a, FullColorPieceBitBoard> {
    type Item = (Color, &'a mut FullPieceBitBoard);

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.index {
            0 => (Color::White, &mut self.data.white),
            1 => (Color::Black, &mut self.data.black),
            _ => return None,
        })
    }
}
