use bevy::prelude::*;
use crate::piece::chess_piece::ChessPiece;
use crate::piece::piece::Piece;

#[derive(Component, Copy, Clone, Hash, Eq, PartialEq, Debug, Reflect)]
pub enum PlacedPiece {
    Empty,
    Black(Piece),
    White(Piece),
}

impl PlacedPiece {
    pub fn from_chess_piece(piece: &Option<ChessPiece>) -> PlacedPiece {
        use PlacedPiece::*;
        let Some(piece) = piece else {
            return Empty;
        };
        let color = match piece.as_color() {
            crate::color::Color::White => White,
            crate::color::Color::Black => Black,
        };
        let piece = piece.as_piece();
        color(piece)
    }
}
