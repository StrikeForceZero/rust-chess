use crate::color::Color;
use crate::notation::pgn::pgn_move_builder::PgnMoveBuilder;
use crate::notation::pgn::pgn_move_detail_builder::PgnMoveDetailBuilder;
use crate::notation::pgn::pgn_turn_data::PgnTurnData;

#[derive(Debug, Default)]
pub struct PgnTurnBuilder {
    pub turn_number: usize,
    pub white: Option<PgnMoveBuilder>,
    pub black: Option<PgnMoveBuilder>,
    pub comment: Option<String>,
}

impl PgnTurnBuilder {
    pub fn new(turn_number: usize) -> Self {
        Self {
            turn_number,
            ..Default::default()
        }
    }
    pub fn get_or_insert(&mut self, color: Color) -> &mut PgnMoveBuilder {
        match color {
            Color::White => self.white.get_or_insert(PgnMoveBuilder::default()),
            Color::Black => self.black.get_or_insert(PgnMoveBuilder::default()),
        }
    }
    pub fn set_comment(&mut self, comment: String) -> &mut Self {
        self.comment = Some(comment);
        self
    }
    pub fn build(&mut self) -> Result<PgnTurnData, String> {
        let Some(white_move_builder) = self.white.take() else {
            return Err("white move data required to build!".into());
        };

        let mut black = None;
        if let Some(black_move_builder) = self.black.take() {
            black = Some(black_move_builder.build()?);
        };

        Ok(PgnTurnData {
            turn_number: self.turn_number,
            white: white_move_builder.build()?,
            black,
            comment: self.comment.take(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use rstest::rstest;
    use crate::board::board::Board;
    use crate::board::board_position::BoardPosition;
    use crate::notation::fen::deserialize;
    use crate::board::position::*;
    use crate::notation::pgn::pgn_move_detail::PgnCheckFlag;
    use crate::piece::chess_piece::ChessPiece;
    use crate::state::game_state::GameState;
    use crate::state::game_status::{is_check, is_check_mate, GameStatus};
    use super::*;

    fn static_str_option_to_string_option(a: Option<&'static str>) -> Option<String> {
        let Some(s) = a else {
            return None;
        };
        Some(s.to_string())
    }

    fn perform_move(game_state: &mut GameState, from_pos: BoardPosition, to_pos: BoardPosition) -> (ChessPiece, Option<ChessPiece>) {
        let chess_piece = game_state.board.get_mut(from_pos).take().expect(&format!("no piece at pos: {from_pos}"));
        let captured_piece = game_state.board.replace(to_pos, Some(chess_piece));
        game_state.active_color = chess_piece.as_color().as_inverse();
        if is_check(&game_state) {
            game_state.game_status = GameStatus::Check(game_state.active_color);
        }
        if is_check_mate(&game_state) {
            game_state.game_status = GameStatus::CheckMate(game_state.active_color);
        }
        (chess_piece, captured_piece)
    }

    fn perform_move_and_update_turn_builder(
        turn_builder: &mut PgnTurnBuilder,
        game_state: &mut GameState,
        from_to_tuple: (BoardPosition, BoardPosition),
        comment: Option<&'static str>,
    ) {
        let color = game_state.active_color;
        let (from, to) = from_to_tuple;
        // TODO: use move handler
        let (chess_piece, captured_piece) = perform_move(game_state, from, to);
        turn_builder.get_or_insert(color).get_move_detail_mut().refresh(chess_piece, &game_state.board, from, to);
        turn_builder.get_or_insert(color).get_move_detail_mut().comment = static_str_option_to_string_option(comment);
        match game_state.game_status {
            GameStatus::Check(_) => {
                turn_builder.get_or_insert(color).get_move_detail_mut().set_check_flag(PgnCheckFlag::Check);
            },
            GameStatus::CheckMate(_) => {
                turn_builder.get_or_insert(color).get_move_detail_mut().set_check_flag(PgnCheckFlag::Mate);
            },
            _ => {},
        };
        turn_builder.get_or_insert(color).get_move_detail_mut().set_is_capture(captured_piece.is_some());
    }

    #[rstest]
    #[case(
        "B3R3/7p/2B2PP1/8/8/N1N4R/5k2/K6Q w - - 0 1",
        (A3, B5),
        Some("white comment"),
        Some((H7, G6)),
        Some("black comment"),
        Some("line comment"),
        "1. Nab5 {white comment} 1... hxg6 {black comment} ;line comment\n"
    )]
    #[case(
        "B3R3/7p/2B2PP1/8/8/N1N4R/5k2/K6Q w - - 0 1",
        (A3, B5),
        None,
        Some((H7, G6)),
        None,
        None,
        "1. Nab5 hxg6"
    )]
    #[case(
        "B7/7p/2B2PP1/8/8/N1N1R2R/5k2/K6Q w - - 0 1",
        (H1, E1),
        Some("white comment"),
        None,
        Some("black comment"),
        Some("line comment"),
        "1. Qe1# {white comment} ;line comment\n"
    )]
    fn e2e_turn_to_text(
        #[case] fen_str: &'static str,
        #[case] a: (BoardPosition, BoardPosition),
        #[case] a_comment: Option<&'static str>,
        #[case] b: Option<(BoardPosition, BoardPosition)>,
        #[case] b_comment: Option<&'static str>,
        #[case] comment: Option<&'static str>,
        #[case] expected: &'static str,
    ) -> Result<(), Box<dyn Error>> {
        let mut game_state = deserialize(fen_str)?;

        let mut turn_builder = PgnTurnBuilder::new(1);

        perform_move_and_update_turn_builder(&mut turn_builder, &mut game_state, a, a_comment);
        if let Some(b) = b {
            perform_move_and_update_turn_builder(&mut turn_builder, &mut game_state, b, b_comment);
        }

        turn_builder.comment = static_str_option_to_string_option(comment);

        assert_eq!(expected, turn_builder.build()?.to_string());

        Ok(())
    }
}
