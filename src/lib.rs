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
    fn transposition() {
        println!("{}", 7 - (BoardFile::H.as_zero_based_index() as i64 - BoardRank::One.as_zero_based_index() as i64) as i64 - 7);
        println!("B5:{}", 7 - (BoardFile::B.as_zero_based_index() as i64 - BoardRank::Five.as_zero_based_index() as i64) as i64 - 7);
        println!("B2:{}", 7 - (BoardFile::B.as_zero_based_index() as i64 - BoardRank::Two.as_zero_based_index() as i64) as i64 - 7);
        println!("{}", 7 - (BoardFile::H.as_zero_based_index() as i64 - BoardRank::Two.as_zero_based_index() as i64) as i64 - 7);
        println!("{}", 7 - (BoardFile::G.as_zero_based_index() as i64 - BoardRank::One.as_zero_based_index() as i64) as i64 - 7);
        println!("{}", 7 - (BoardFile::A.as_zero_based_index() as i64 - BoardRank::One.as_zero_based_index() as i64) as i64 - 7);
        println!("{}", 7 - (BoardFile::E.as_zero_based_index() as i64 - BoardRank::One.as_zero_based_index() as i64) as i64 - 7);
        println!("{}", 7 - (BoardFile::A.as_zero_based_index() as i64 - BoardRank::Eight.as_zero_based_index() as i64) as i64 - 7);
        println!("{}", 7 - (BoardFile::A.as_zero_based_index() as i64 - BoardRank::One.as_zero_based_index() as i64) as i64 - 7);
        println!("{}", 7 - (BoardFile::A.as_zero_based_index() as i64 - BoardRank::Three.as_zero_based_index() as i64) as i64 - 7);
        println!("{}", 7 - (BoardFile::H.as_zero_based_index() as i64 - BoardRank::Seven.as_zero_based_index() as i64) as i64 - 7);


        println!("---");
        println!("{}", 7 - (BoardFile::H.as_zero_based_index() + BoardRank::One.as_zero_based_index()).abs_diff(7));
        println!("B5:{}", 7 - (BoardFile::B.as_zero_based_index() + BoardRank::Five.as_zero_based_index()).abs_diff(7));
        println!("B2:{}", 7 - (BoardFile::B.as_zero_based_index() + BoardRank::Two.as_zero_based_index()).abs_diff(7));
        println!("{}", 7 - (BoardFile::H.as_zero_based_index() + BoardRank::Two.as_zero_based_index()).abs_diff(7));
        println!("{}", 7 - (BoardFile::G.as_zero_based_index() + BoardRank::One.as_zero_based_index()).abs_diff(7));
        println!("{}", 7 - (BoardFile::A.as_zero_based_index() + BoardRank::One.as_zero_based_index()).abs_diff(7));
        println!("{}", 7 - (BoardFile::E.as_zero_based_index() + BoardRank::One.as_zero_based_index()).abs_diff(7));
        println!("{}", 7 - (BoardFile::A.as_zero_based_index() + BoardRank::Eight.as_zero_based_index()).abs_diff(7));
        println!("{}", 7 - (BoardFile::A.as_zero_based_index() + BoardRank::One.as_zero_based_index()).abs_diff(7));
        println!("{}", 7 - (BoardFile::A.as_zero_based_index() + BoardRank::Three.as_zero_based_index()).abs_diff(7));
        println!("{}", 7 - (BoardFile::H.as_zero_based_index() + BoardRank::Seven.as_zero_based_index()).abs_diff(7));

        println!("{}", BitBoard::from_value(bit_board::FULL_DIAG_RIGHT >> 8 * 7).as_multiline_str()); // h1
        println!("{}", BitBoard::from_value(bit_board::FULL_DIAG_RIGHT >> 8 * 6).as_multiline_str()); // g1 - h2
        println!("{}", BitBoard::from_value(bit_board::FULL_DIAG_RIGHT >> 8 * 5).as_multiline_str()); // f1 - h3
        println!("{}", BitBoard::from_value(bit_board::FULL_DIAG_RIGHT >> 8 * 4).as_multiline_str()); // e1 - h4
        println!("{}", BitBoard::from_value(bit_board::FULL_DIAG_RIGHT >> 8 * 3).as_multiline_str()); // d1 - h5
        println!("{}", BitBoard::from_value(bit_board::FULL_DIAG_RIGHT >> 8 * 2).as_multiline_str()); // c1 - h6
        println!("{}", BitBoard::from_value(bit_board::FULL_DIAG_RIGHT >> 8 * 1).as_multiline_str()); // b1 - h7
        println!("{}", BitBoard::from_value(bit_board::FULL_DIAG_RIGHT).as_multiline_str());          // a1 - h8
        println!("{}", BitBoard::from_value(bit_board::FULL_DIAG_RIGHT << 8 * 1).as_multiline_str()); // a2 - g8
        println!("{}", BitBoard::from_value(bit_board::FULL_DIAG_RIGHT << 8 * 2).as_multiline_str()); // a3 - f8
        println!("{}", BitBoard::from_value(bit_board::FULL_DIAG_RIGHT << 8 * 3).as_multiline_str()); // a4 - e8
        println!("{}", BitBoard::from_value(bit_board::FULL_DIAG_RIGHT << 8 * 4).as_multiline_str()); // a5 - d8
        println!("{}", BitBoard::from_value(bit_board::FULL_DIAG_RIGHT << 8 * 5).as_multiline_str()); // a6 - c8
        println!("{}", BitBoard::from_value(bit_board::FULL_DIAG_RIGHT << 8 * 6).as_multiline_str()); // a7 - b8
        println!("{}", BitBoard::from_value(bit_board::FULL_DIAG_RIGHT << 8 * 7).as_multiline_str()); // a8
    }
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
        let mut a = BitBoard::from_value(0);
        let mut b = BitBoard::from_value(0);
        let mut c = BitBoard::from_value(0);
        let mut d = BitBoard::from_value(0);
        let mut e = BitBoard::from_value(0);
        let mut f = BitBoard::from_value(0);
        let mut g = BitBoard::from_value(0);
        let mut h = BitBoard::from_value(0);
        let mut i = BitBoard::from_value(0);

        a.fill_diag_from_pos(BoardPosition::from(BoardFile::A, BoardRank::One));
        b.fill_diag_from_pos(BoardPosition::from(BoardFile::A, BoardRank::Two));
        c.fill_diag_from_pos(BoardPosition::from(BoardFile::B, BoardRank::Two));
        d.fill_diag_from_pos(BoardPosition::from(BoardFile::H, BoardRank::Five));
        e.fill_diag_from_pos(BoardPosition::from(BoardFile::B, BoardRank::Five));
        f.fill_diag_from_pos(BoardPosition::from(BoardFile::E, BoardRank::Four));
        g.fill_diag_from_pos(BoardPosition::from(BoardFile::H, BoardRank::One));
        h.fill_diag_from_pos(BoardPosition::from(BoardFile::H, BoardRank::Eight));

        let boards = vec![
            (BoardPosition::from(BoardFile::A, BoardRank::One), a),
            (BoardPosition::from(BoardFile::A, BoardRank::Two), b),
            (BoardPosition::from(BoardFile::B, BoardRank::Two), c),
            (BoardPosition::from(BoardFile::H, BoardRank::Five), d),
            (BoardPosition::from(BoardFile::B, BoardRank::Five), e),
            (BoardPosition::from(BoardFile::E, BoardRank::Four), f),
            (BoardPosition::from(BoardFile::H, BoardRank::One), g),
            (BoardPosition::from(BoardFile::H, BoardRank::Eight), h),
        ];
        for (pos, board) in boards {
            println!("{pos}:\n{}",board.as_multiline_str());
        }
    }
}
