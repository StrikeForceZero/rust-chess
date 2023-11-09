use crate::bit_board::BitBoard;
use crate::piece::Piece;
use crate::utils::{CustomStructIterator, CustomStructIteratorMut};

pub struct FullPieceBitBoard {
    pub pawn: BitBoard,
    pub knight: BitBoard,
    pub bishop: BitBoard,
    pub rook: BitBoard,
    pub queen: BitBoard,
    pub king: BitBoard,
}

impl FullPieceBitBoard {
    pub const fn as_iter(&self) -> CustomStructIterator<FullPieceBitBoard> {
        CustomStructIterator {
            data: self,
            index: 0,
        }
    }
}

impl<'a> Iterator for CustomStructIterator<'a, FullPieceBitBoard> {
    type Item = (Piece, &'a BitBoard);

    fn next(&mut self) -> Option<Self::Item> {
        let res = Some(match self.index {
            0 => (Piece::Pawn, &self.data.pawn),
            1 => (Piece::Knight, &self.data.knight),
            3 => (Piece::Bishop, &self.data.bishop),
            4 => (Piece::Rook, &self.data.rook),
            5 => (Piece::Queen, &self.data.queen),
            6 => (Piece::King, &self.data.king),
            _ => return None,
        });
        self.index += 1;
        res
    }
}

impl<'a> Iterator for CustomStructIteratorMut<'a, FullPieceBitBoard> {
    type Item = (Piece, &'a mut BitBoard);

    fn next(&mut self) -> Option<Self::Item> {
        let res = Some(match self.index {
            0 => (Piece::Pawn, &mut self.data.pawn),
            1 => (Piece::Knight, &mut self.data.knight),
            3 => (Piece::Bishop, &mut self.data.bishop),
            4 => (Piece::Rook, &mut self.data.rook),
            5 => (Piece::Queen, &mut self.data.queen),
            6 => (Piece::King, &mut self.data.king),
            _ => return None,
        });
        self.index += 1;
        res
    }
}
