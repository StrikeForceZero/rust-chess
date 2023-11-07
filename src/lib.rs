mod bit_board;
mod color;
mod piece;
mod piece_bit_board;
mod color_piece_bit_board;
mod colored_piece;
mod fen;
mod board_rank;
mod board_file;
mod board_position;

#[cfg(test)]
mod tests {
    use crate::bit_board::BitBoard;
    use crate::board_file::BoardFile;
    use crate::board_position::BoardPosition;
    use crate::board_rank::BoardRank;
    use crate::color::Color;
    use crate::color_piece_bit_board::ColorPieceBitBoard;
    use crate::piece::Piece;
    use crate::piece_bit_board::PieceBitBoard;
    use super::*;
    #[test]
    fn test() {
        let bit_boards = vec![
            ColorPieceBitBoard::from(Color::White, PieceBitBoard::from(Piece::Pawn, BitBoard::from_value(bit_board::PAWN << BoardRank::Two.as_shift_offset()))),
            ColorPieceBitBoard::from(Color::White, PieceBitBoard::from(Piece::Rook, BitBoard::from_value(bit_board::ROOK))),
            ColorPieceBitBoard::from(Color::White, PieceBitBoard::from(Piece::Knight, BitBoard::from_value(bit_board::KNIGHT))),
            ColorPieceBitBoard::from(Color::White, PieceBitBoard::from(Piece::Bishop, BitBoard::from_value(bit_board::BISHOP))),
            ColorPieceBitBoard::from(Color::White, PieceBitBoard::from(Piece::Queen, BitBoard::from_value(bit_board::QUEEN))),
            ColorPieceBitBoard::from(Color::White, PieceBitBoard::from(Piece::King, BitBoard::from_value(bit_board::KING))),

            ColorPieceBitBoard::from(Color::Black, PieceBitBoard::from(Piece::Pawn, BitBoard::from_value(bit_board::PAWN << BoardRank::Seven.as_shift_offset()))),
            ColorPieceBitBoard::from(Color::Black, PieceBitBoard::from(Piece::Rook, BitBoard::from_value(bit_board::ROOK << BoardRank::Eight.as_shift_offset()))),
            ColorPieceBitBoard::from(Color::Black, PieceBitBoard::from(Piece::Knight, BitBoard::from_value(bit_board::KNIGHT << BoardRank::Eight.as_shift_offset()))),
            ColorPieceBitBoard::from(Color::Black, PieceBitBoard::from(Piece::Bishop, BitBoard::from_value(bit_board::BISHOP << BoardRank::Eight.as_shift_offset()))),
            ColorPieceBitBoard::from(Color::Black, PieceBitBoard::from(Piece::Queen, BitBoard::from_value(bit_board::QUEEN << BoardRank::Eight.as_shift_offset()))),
            ColorPieceBitBoard::from(Color::Black, PieceBitBoard::from(Piece::King, BitBoard::from_value(bit_board::KING << BoardRank::Eight.as_shift_offset()))),
        ];
        let mut b = BitBoard::from_value(0);
        b.fill_diag_from_pos(BoardPosition::from(BoardFile::A, BoardRank::One));
        println!("{:064b}", b.bitmap().as_value());
        println!("\n{}", b.as_multiline_str());
    }
}
