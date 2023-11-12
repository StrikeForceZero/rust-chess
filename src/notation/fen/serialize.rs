use itertools::Itertools;
use crate::board::board_position::BoardPosition;
use crate::notation::fen::{BOARD_TERMINATOR, Fen};
use crate::notation::fen::ActiveColor;
use crate::piece::chess_piece::ChessPiece;
use crate::state::color_castle_rights::ColorCastleRights;
use crate::state::game_state::GameState;

const EMPTY: &str = "";
const SPACE: char = ' ';
const DASH: char = '-';

struct FenData {
    squares: [[Option<ChessPiece>; 8]; 8],
    active_color: ActiveColor,
    castle: ColorCastleRights,
    en_passant: Option<BoardPosition>,
    half_move_clock: u16,
    full_move_num: u16,
}

impl FenData {
    pub fn to_string(&self) -> Fen {
        let board_str = self
            .squares
            .iter()
            .map(|row| {
                let mut count: u8 = 0;
                let mut chunks: Vec<String> = vec![];
                for (ix, square) in row.iter().enumerate() {
                    if let Some(piece) = square {
                        if count > 0 {
                            chunks.push(count.to_string());
                            count = 0;
                        }
                        chunks.push(piece.as_char().to_string());
                    } else {
                        count += 1;
                    }
                    let is_last = ix == row.len() - 1;
                    if count > 0 && is_last {
                        chunks.push(count.to_string());
                    }
                }
                chunks.join(EMPTY)
            })
            .collect::<Vec<String>>()
            .join(BOARD_TERMINATOR);
        let str: String = [
            board_str,
            self.active_color.as_char().to_string(),
            self.castle.as_str().to_string(),
            if self.en_passant.is_some() {
                self.en_passant.unwrap().to_string().to_lowercase()
            } else {
                DASH.to_string()
            },
            self.half_move_clock.to_string(),
            self.full_move_num.to_string(),
        ]
        .iter()
        .filter(|x| !x.is_empty())
        .join(SPACE.to_string().as_str());

        Fen::Owned(str)
    }
}

pub fn serialize(game_state: &GameState) -> Fen {
    let mut fen = FenData {
        squares: [[None; 8]; 8],
        active_color: ActiveColor::from_color(game_state.active_color),
        castle: game_state.castle_rights.clone(),
        en_passant: game_state.en_passant_target_pos,
        half_move_clock: game_state.move_counter.half_move,
        full_move_num: game_state.move_counter.full_move,
    };
    for (row_ix, &row) in game_state.board.as_slice().iter().rev().enumerate() {
        for (col_ix, &col) in row.iter().enumerate() {
            fen.squares[row_ix][col_ix] = *col;
        }
    }
    fen.to_string()
}

pub fn serialize_without_clock_and_active_color(game_state: GameState) -> String {
    let fen = serialize(&game_state);
    let serialized = fen.get_str();
    let parts = serialized.split_whitespace().collect::<Vec<_>>();
    let (
        squares_str,
        _active_color_str,
        castle_rights_str,
        en_passant_str,
        _half_move_clock_str,
        _full_move_num_str,
    ) = (parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]);
    [squares_str, castle_rights_str, en_passant_str].join(SPACE.to_string().as_str())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::notation::fen::*;
    use crate::state::game_state::GameState;

    #[rstest]
    #[case(GameState::new(), FEN_STARTING_POS)]
    #[case(GameState::empty(), FEN_EMPTY)]
    pub fn fen_serialize(#[case] game_state: GameState, #[case] expected: &'static str) {
        assert_eq!(expected, serialize(&game_state).get_str())
    }
}
