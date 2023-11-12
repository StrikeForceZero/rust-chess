use crate::bit_board::bit_board_const::BitBoardConst;
use crate::piece::piece::Piece;
use crate::utils::custom_struct_iterator::CustomStructIterator;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FullPieceBitBoard {
    pub pawn: BitBoardConst,
    pub knight: BitBoardConst,
    pub bishop: BitBoardConst,
    pub rook: BitBoardConst,
    pub queen: BitBoardConst,
    pub king: BitBoardConst,
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
    type Item = (Piece, &'a BitBoardConst);

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
