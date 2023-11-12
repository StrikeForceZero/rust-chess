use thiserror::Error;
use crate::board::board_file::{BoardFile, BoardFileError};
use crate::board::board_position::{BoardPosition, BoardPositionStrParseError};
use crate::board::board_rank::{BoardRank, BoardRankError};
use crate::notation::fen::{ActiveColor, BOARD_TERMINATOR};
use crate::notation::fen::fen_parts::FenParts;
use crate::piece::chess_piece::ChessPiece;
use crate::state::castle_rights::CastleRightsStringParseError;
use crate::state::color_castle_rights::ColorCastleRights;
use crate::state::game_state::GameState;
use crate::state::game_status::{GameStatus, is_check, is_check_mate};
use crate::state::state_history::StateHistoryContainer;

#[derive(Error, Debug, Clone)]
pub enum FenParsingError {
    #[error("Invalid active color for Fen, active color expects to be W | B, received: {0}")]
    InvalidActiveColorString(String),
    #[error("Invalid active color for Fen, active color expects to be W | B, received: {0}")]
    InvalidActiveColorChar(char),
    #[error("Invalid castle rights: {0}")]
    InvalidCastleRights(CastleRightsStringParseError),
    #[error("Invalid en passant: {0}")]
    InvalidEnPassant(BoardPositionStrParseError),
    #[error("Invalid board string: {0}")]
    InvalidBoardString(String),
    #[error("Invalid board string: {0}")]
    InvalidBoardStringBoardFileParseError(BoardFileError),
    #[error("Invalid board string: {0}")]
    InvalidBoardStringBoardRankParseError(BoardRankError),
    #[error("Invalid fen string: {0}")]
    InvalidFenString(String),
}

pub fn get_parts(fen_str: &str) -> Result<FenParts, FenParsingError> {
    let parts = fen_str.split_whitespace().collect::<Vec<_>>();
    if parts.len() != 6 {
        return Err(FenParsingError::InvalidFenString(fen_str.to_string()))
    }
    let (
        squares_str,
        active_color_str,
        castle_rights_str,
        en_passant_str,
        half_move_clock_str,
        full_move_num_str,
    ) = (parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]);
    Ok(FenParts {
        squares_str,
        active_color_str,
        castle_rights_str,
        en_passant_str,
        half_move_clock_str,
        full_move_num_str,
    })
}

fn get_active_color_from_str(active_color_str: &str) -> Result<ActiveColor, FenParsingError> {
    if active_color_str.len() != 1 {
        return Err(FenParsingError::InvalidActiveColorString(
            active_color_str.to_string(),
        ));
    }
    let active_color_chars = active_color_str.chars().take(1).collect::<Vec<_>>();
    let &active_color_char = active_color_chars.get(0).unwrap();
    ActiveColor::from_char(active_color_char)
}

fn get_en_passant_pos_from_str(
    en_passant_str: &str,
) -> Result<Option<BoardPosition>, BoardPositionStrParseError> {
    if en_passant_str.len() == 0 || en_passant_str == "-" {
        return Ok(None);
    }
    Ok(Some(match BoardPosition::from_str(en_passant_str) {
        Ok(pos) => pos,
        Err(err) => return Err(err),
    }))
}

pub fn deserialize(fen_str: &str) -> Result<GameState, FenParsingError> {
    let mut game_state = GameState::empty();
    let parts = get_parts(fen_str)?;
    game_state.active_color = match get_active_color_from_str(parts.active_color_str) {
        Ok(color) => color.as_color(),
        Err(e) => return Err(e),
    };
    game_state.castle_rights = match ColorCastleRights::from_str(parts.castle_rights_str) {
        Ok(castle_rights) => castle_rights,
        Err(e) => return Err(FenParsingError::InvalidCastleRights(e)),
    };
    game_state.move_counter.half_move = parts.half_move_clock_str
        .parse::<u16>()
        .expect("invalid half_move_clock_str");
    game_state.move_counter.full_move = parts.full_move_num_str
        .parse::<u16>()
        .expect("invalid full_move_clock_str");
    game_state.en_passant_target_pos = match get_en_passant_pos_from_str(parts.en_passant_str) {
        Ok(pos) => pos,
        Err(e) => return Err(FenParsingError::InvalidEnPassant(e)),
    };
    let rows = parts.squares_str
        .split_terminator(BOARD_TERMINATOR)
        .collect::<Vec<_>>();
    // flip so white is on bottom
    let rows = rows.iter().rev();
    for (row_ix, row) in rows.enumerate() {
        let rank_u8 = u8::try_from(row_ix + 1).expect("row_ix overflow");
        let rank = match BoardRank::from_u8(rank_u8) {
            Ok(rank) => rank,
            Err(e) => return Err(FenParsingError::InvalidBoardStringBoardRankParseError(e)),
        };
        let chars = row.chars();
        let mut offset: u8 = 0;
        for (char_ix, char) in chars.enumerate() {
            let file_u8 = u8::try_from(char_ix + 1).expect("char_ix overflow") + offset;
            let mut file = match BoardFile::from_u8(file_u8) {
                Ok(file) => file,
                Err(e) => return Err(FenParsingError::InvalidBoardStringBoardFileParseError(e)),
            };

            if let Ok(blanks) = format!("{char}").parse::<u8>() {
                offset += blanks - 1;
                let mut remaining = blanks;
                while remaining > 0 {
                    let pos = BoardPosition(file, rank);
                    game_state.board.set(pos, None);
                    remaining -= 1;
                    if remaining > 0 {
                        let Some(next_file) = file.next() else {
                            return Err(FenParsingError::InvalidBoardString(format!(
                                "invalid fen string - given: {blanks}, overflow: {remaining}"
                            )));
                        };
                        file = next_file;
                    }
                }
                continue;
            }
            match ChessPiece::from_char(char) {
                Ok(piece) => game_state.board.set(BoardPosition(file, rank), Some(piece)),
                Err(err) => return Err(FenParsingError::InvalidBoardString(format!("{} {err:?}", parts.squares_str))),
            }
        }
    }
    game_state.history.state_history = Some(StateHistoryContainer::New(
        game_state.board.as_bit_boards_const(),
    ));
    if is_check(&game_state) {
        game_state.game_status = GameStatus::Check(game_state.active_color);
        if is_check_mate(&game_state) {
            game_state.game_status = GameStatus::CheckMate(game_state.active_color);
        }
    }
    Ok(game_state)
}
