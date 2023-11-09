use crate::board::Board;
use crate::board_position::BoardPosition;
use crate::chess_piece::ChessPiece;
use crate::direction::Direction;

pub struct BoardScanner<'a> {
    pub board: &'a Board,
    pub starting_pos: BoardPosition,
    pub last_pos: Option<BoardPosition>,
    pub direction: Direction,
}

impl<'a> BoardScanner<'a> {
    pub const fn from_pos(board: &'a Board, starting_pos: BoardPosition, direction: Direction) -> Self {
        BoardScanner {
            board,
            starting_pos,
            last_pos: Some(starting_pos),
            direction,
        }
    }
}

impl<'a> Iterator for BoardScanner<'a> {
    type Item = (BoardPosition, &'a Option<ChessPiece>);

    fn next(&mut self) -> Option<Self::Item> {
        let Some(last_pos) = self.last_pos else {
            return None;
        };
        self.last_pos = self.direction.get_next_pos(last_pos);
        let Some(next_pos) = self.last_pos else {
            return None;
        };
        Some((next_pos, self.board.get(next_pos)))
    }
}
