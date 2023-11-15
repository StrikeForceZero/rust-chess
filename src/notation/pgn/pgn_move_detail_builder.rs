use crate::bit_board::bit_board::BitBoard;
use crate::board::board::Board;
use crate::board::board_file::BoardFile;
use crate::board::board_position::BoardPosition;
use crate::board::board_rank::BoardRank;
use crate::notation::pgn::pgn_move_detail::{PgnCheckFlag, PgnMoveDetail};
use crate::piece::chess_piece::ChessPiece;
use crate::piece::piece::Piece;
use crate::piece::promotion_piece::PromotionPiece;

// TODO: add support for promotions
//  FIDE Examples: d8Q, f8N, b1B, g1R
//  SAN Examples: d8=Q, f8=N, b1=B, g1=R
//  Alt Examples: d8(Q), f8/N
#[derive(Debug, Default)]
pub struct PgnMoveDetailBuilder {
    pub chess_piece: Option<ChessPiece>,
    pub from_board_file: Option<BoardFile>,
    pub from_board_rank: Option<BoardRank>,
    pub to_pos: Option<BoardPosition>,
    pub is_capture: Option<bool>,
    pub check_flag: Option<PgnCheckFlag>,
    pub comment: Option<String>,
    pub promotion: Option<PromotionPiece>,
}

impl PgnMoveDetailBuilder {
    pub fn create(
        chess_piece: ChessPiece,
        board: &Board,
        from_pos: BoardPosition,
        to_pos: BoardPosition,
    ) -> Self {
        let mut entry = Self {
            chess_piece: Some(chess_piece),
            to_pos: Some(to_pos),
            ..Default::default()
        };
        match resolve_from_discriminator_dynamic(board, chess_piece, from_pos, to_pos) {
            None => {}
            Some(FromDiscriminator::File(file)) => {
                entry.from_board_file = Some(file);
            },
            Some(FromDiscriminator::Rank(rank)) => {
                entry.from_board_rank = Some(rank);
            },
        }
        entry
    }
    pub fn refresh(
        &mut self,
        chess_piece: ChessPiece,
        board: &Board,
        from_pos: BoardPosition,
        to_pos: BoardPosition,
    ) {
        self.chess_piece = Some(chess_piece);
        self.from_board_file = None;
        self.from_board_rank = None;
        self.to_pos = Some(to_pos);
        self.is_capture = None;
        self.check_flag = None;
        self.promotion = None;
        match resolve_from_discriminator_dynamic(board, chess_piece, from_pos, to_pos) {
            None => {}
            Some(FromDiscriminator::File(file)) => {
                self.from_board_file = Some(file);
            },
            Some(FromDiscriminator::Rank(rank)) => {
                self.from_board_rank = Some(rank);
            },
        }
    }
    pub fn has_from(&self) -> bool {
        self.from_board_file.is_some() || self.from_board_rank.is_some()
    }
    pub fn set_is_capture(&mut self, value: bool) -> &mut Self {
        self.is_capture = Some(value);
        self
    }
    pub fn set_check_flag(&mut self, value: PgnCheckFlag) -> &mut Self {
        self.check_flag = Some(value);
        self
    }
    pub fn set_promotion(&mut self, value: PromotionPiece) -> &mut Self {
        self.promotion = Some(value);
        self
    }
    pub fn build(self) -> Result<PgnMoveDetail, &'static str> {
        let Some(chess_piece) = self.chess_piece else {
            return Err("missing chess_piece!")
        };
        let Some(to_pos) = self.to_pos else {
            return Err("missing to_pos!")
        };
        Ok(PgnMoveDetail {
            chess_piece,
            from_board_file: self.from_board_file,
            from_board_rank: self.from_board_rank,
            to_pos,
            is_capture: self.is_capture.unwrap_or(false),
            check_flag: self.check_flag,
            comment: self.comment,
            promotion: self.promotion,
        })
    }
}

pub enum FromDiscriminator {
    File(BoardFile),
    Rank(BoardRank),
}

fn resolve_from_discriminator<F>(
    bit_board: BitBoard,
    from_pos: BoardPosition,
    to_pos: BoardPosition,
    lookup_fn: F,
) -> Option<FromDiscriminator>
    where F: Fn(BoardPosition) -> u64
{
    let mut potential_from_positions =  BitBoard::from_value(lookup_fn(to_pos));
    // remove the original piece
    potential_from_positions.set_pos(from_pos, false);
    // for bishops and rooks
    for (pos, value) in bit_board.as_iter() {
        // skip unset bits or same position as the original piece
        if !value || pos == from_pos || pos == to_pos {
            continue;
        }
        let other_moves = lookup_fn(pos);
        let same_moves = potential_from_positions.bitmap().as_value() & other_moves;
        if same_moves > 0 {
            // TODO: this probably breaks if more than 2 pieces of the same type is one the board
            // if same file, then return rank
            if *pos.file() == *from_pos.file() {
                println!("same file");
                return Some(FromDiscriminator::Rank(*from_pos.rank()))
            }
            // if same rank then return file
            if *pos.rank() == *from_pos.rank() {
                println!("same rank");
                return Some(FromDiscriminator::File(*from_pos.file()))
            }
            println!("default");
            // default to file
            return Some(FromDiscriminator::File(*from_pos.file()))
        }
    }
    // for knights
    for (pos, value) in bit_board.as_iter() {
        // skip unset bits or same position as the original piece
        if !value || pos == from_pos {
            continue;
        }
        let other_moves = lookup_fn(pos);
        let same_moves = potential_from_positions.bitmap().as_value() & other_moves;
        if same_moves > 0 {
            // TODO: this probably breaks if more than 2 pieces of the same type is one the board
            // if same file, then return rank
            if *pos.file() == *from_pos.file() {
                println!("same file");
                return Some(FromDiscriminator::Rank(*from_pos.rank()))
            }
            // if same rank then return file
            if *pos.rank() == *from_pos.rank() {
                println!("same rank");
                return Some(FromDiscriminator::File(*from_pos.file()))
            }
            println!("default");
            // default to file
            return Some(FromDiscriminator::File(*from_pos.file()))
        }
    }
    None
}

fn resolve_from_discriminator_dynamic(
    board: &Board,
    chess_piece: ChessPiece,
    from_pos: BoardPosition,
    to_pos: BoardPosition,
) -> Option<FromDiscriminator> {
    let bit_boards = board.as_bit_boards_const();
    let bit_board_for_color = bit_boards.for_color(chess_piece.as_color());
    match chess_piece.as_piece() {
        Piece::Pawn => {
            if *from_pos.file() == *to_pos.file() {
                None
            } else {
                Some(FromDiscriminator::File(*from_pos.file()))
            }
        },
        Piece::King | Piece::Queen => None,
        Piece::Knight => {
            resolve_from_discriminator(
                bit_board_for_color.knight.as_bitboard(),
                from_pos,
                to_pos,
                BitBoard::lookup_knight,
            )
        }
        Piece::Bishop => {
            resolve_from_discriminator(
                bit_board_for_color.bishop.as_bitboard(),
                from_pos,
                to_pos,
                BitBoard::lookup_bishop,
            )
        }
        Piece::Rook => {
            resolve_from_discriminator(
                bit_board_for_color.rook.as_bitboard(),
                from_pos,
                to_pos,
                BitBoard::lookup_rook,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use rstest::rstest;
    use crate::board::board_position::BoardPosition;
    use crate::notation::fen::deserialize;
    use crate::board::position::*;
    use crate::state::game_status::{GameStatus, is_check, is_check_mate};
    use super::*;

    #[rstest]
    #[case(
        "B3R3/7p/2B1RPP1/8/8/N1N1R2R/5k2/K6Q w - - 0 1",
        A3, B5,
        "Nab5"
    )]
    #[case(
        "B7/7p/2B2PP1/8/8/N1N1R2R/5k2/K6Q w - - 0 1",
        E3, G3,
        "Reg3"
    )]
    #[case(
        "B3R3/7p/2B2PP1/8/8/N1N4R/5k2/K6Q w - - 0 1",
        E8, E3,
        "Ree3"
    )]
    #[case(
        "B3R3/7p/2B1RPP1/8/8/N1N1R2R/5k2/K6Q w - - 0 1",
        A8, B7,
        "Bab7"
    )]
    #[case(
        "B3R3/7p/2B1RPP1/8/8/N1N5/5k2/K6Q w - - 0 1",
        E6, E7,
        "R6e7"
    )]
    #[case(
        "B3R3/7p/2B1RPP1/8/8/N1N1R2R/5k2/K6Q w - - 0 1",
        F6, F7,
        "f7"
    )]
    #[case(
        "B3R3/7p/2B1RPP1/8/8/N1N1R2R/5k2/K6Q w - - 0 1",
        G6, H7,
        "gxh7"
    )]
    #[case(
        "B3R3/7p/2B1RPP1/8/8/N1N1R2R/5k2/K6Q w - - 0 1",
        A1, B1,
        "Kb1"
    )]
    #[case(
        "B3R3/7p/2B1RPP1/8/8/N1N1R2R/5k2/K6Q w - - 0 1",
        H1, F1,
        "Qf1+"
    )]
    #[case(
        "B3R3/7p/2B1RPP1/8/8/N1N1R2R/5k2/K6Q w - - 0 1",
        H1, G2,
        "Qg2#"
    )]
    fn e2e_detail_to_text(
        #[case] fen_str: &'static str,
        #[case] from: BoardPosition,
        #[case] to: BoardPosition,
        #[case] expected: &'static str,
    ) -> Result<(), Box<dyn Error>> {
        let mut game_state = deserialize(fen_str)?;

        // TODO: use move handler
        let chess_piece = game_state.board.get_mut(from).take().expect(&format!("no piece at pos: {from}"));
        let captured_piece = game_state.board.replace(to, Some(chess_piece));
        game_state.active_color = chess_piece.as_color().as_inverse();
        if is_check(&mut game_state) {
            game_state.game_status = GameStatus::Check(game_state.active_color);
        }
        if is_check_mate(&mut game_state) {
            game_state.game_status = GameStatus::CheckMate(game_state.active_color);
        }

        let mut move_builder = PgnMoveDetailBuilder::create(
                chess_piece,
                &game_state.board,
                from,
            to,
            )
        ;
        move_builder.set_is_capture(captured_piece.is_some());
        let move_entry = if game_state.game_status.is_check_or_mate() {
            match game_state.game_status {
                GameStatus::Check(_) => {
                    move_builder.set_check_flag(PgnCheckFlag::Check);
                },
                GameStatus::CheckMate(_) => {
                    move_builder.set_check_flag(PgnCheckFlag::Mate);
                },
                _ => {},
            };
            move_builder.build()?
        } else {
            move_builder.build()?
        };
        assert_eq!(expected, move_entry.to_string());
        Ok(())
    }
}
