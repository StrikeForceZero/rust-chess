use thiserror::Error;
use crate::board::Board;
use crate::board_position::{BoardPosition, BoardPositionStrParseError};
use crate::board_rank::{BoardRank, BoardRankError};
use itertools::Itertools;
use crate::board_file::{BoardFile, BoardFileError};
use crate::castle_rights::CastleRightsStringParseError;
use crate::chess_piece::ChessPiece;
use crate::color::Color;
use crate::color_castle_rights::ColorCastleRights;
use crate::game_state::GameState;

#[repr(transparent)]
#[derive(Clone)]
pub struct Fen(String);

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
}


const fn get_active_color_from_str(active_color_str: &str) -> Result<ActiveColor, FenParsingError> {
    if active_color_str.len() != 1 {
        return Err(FenParsingError::InvalidActiveColorString(active_color_str.to_string()));
    }
    let active_color_chars = active_color_str.chars().take(1).collect::<Vec<_>>();
    let &active_color_char = active_color_chars.get(0).unwrap();
    ActiveColor::from_char(active_color_char)
}

const fn get_en_passant_pos_from_str(en_passant_str: &str) -> Result<Option<BoardPosition>, BoardPositionStrParseError> {
    if en_passant_str.len() == 0 || en_passant_str == "-" {
        return Ok(None);
    }
    Ok(Some(match BoardPosition::from_str(en_passant_str) {
        Ok(pos) => pos,
        Err(err) => return Err(err),
    }))
}

const BOARD_TERMINATOR: &str = "/";
pub fn deserialize(fen_str: &str) -> Result<GameState, FenParsingError> {
    let mut game_state = GameState::new();
    let parts = fen_str.split_whitespace().collect::<Vec<_>>();
    let (squares_str, active_color_str, castle_rights_str, en_passant_str, half_move_clock_str, full_move_clock_str) = (parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]);
    game_state.active_color = match get_active_color_from_str(active_color_str) {
        Ok(color) => color.as_color(),
        Err(e) => return Err(e),
    };
    game_state.castle_rights = match ColorCastleRights::from_str(castle_rights_str) {
        Ok(castle_rights) => castle_rights,
        Err(e) => return Err(FenParsingError::InvalidCastleRights(e)),
    };
    game_state.move_clock.half_move = half_move_clock_str.parse::<u16>().expect("invalid half_move_clock_str");
    game_state.move_clock.full_move = full_move_clock_str.parse::<u16>().expect("invalid full_move_clock_str");
    game_state.en_passant_target_pos = match get_en_passant_pos_from_str(en_passant_str) {
        Ok(pos) => pos,
        Err(e) => return Err(FenParsingError::InvalidEnPassant(e))
    };
    let rows = squares_str.split_terminator(BOARD_TERMINATOR).collect::<Vec<_>>();
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
                            return Err(FenParsingError::InvalidBoardString(format!("invalid fen string - given: {blanks}, overflow: {remaining}")));
                        };
                        file = next_file;
                    }
                }
                continue;
            }
            match ChessPiece::from_char(char) {
                Ok(piece) => game_state.board.set(BoardPosition(file, rank), Some(piece)),
                Err(err) => panic!("{err}"),
            }
        }
    }
    game_state.history.fen.push(Fen(fen_str.to_string()));
    Ok(game_state)
}

#[derive(Copy, Clone)]
enum ActiveColor {
    White,
    Black,
}

impl ActiveColor {
    pub fn as_char(&self) -> char {
        match self {
            ActiveColor::White => 'w',
            ActiveColor::Black => 'b',
        }
    }
    pub fn from_char(c: char) -> Result<ActiveColor, FenParsingError> {
        Ok(match c {
            'w' => ActiveColor::White,
            'b' => ActiveColor::Black,
            _ => return Err(FenParsingError::InvalidActiveColorChar(c)),
        })
    }
    pub fn from_color(color: Color) -> ActiveColor {
        match color {
            Color::White => ActiveColor::White,
            Color::Black => ActiveColor::Black,
        }
    }
    pub fn as_color(&self) -> Color {
        match self {
            ActiveColor::White => Color::White,
            ActiveColor::Black => Color::Black,
        }
    }
}

struct FEN {
    squares: [[Option<ChessPiece>; 8]; 8],
    active_color: ActiveColor,
    castle: ColorCastleRights,
    en_passant: Option<BoardPosition>,
    half_move_clock: u16,
    full_move_clock: u16,
}

const EMPTY: &str = "";
const SPACE: char = ' ';
const DASH: char = '-';
impl FEN {
    pub fn to_string(&self) -> String {
        let board_str = self.squares.iter()
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
            if self.en_passant.is_some() { self.en_passant.unwrap().to_string().to_lowercase() } else { DASH.to_string() },
            self.half_move_clock.to_string(),
            self.full_move_clock.to_string(),
        ]
            .iter()
            .filter(|x| !x.is_empty())
            .join(SPACE.to_string().as_str());

        return str;
    }
}

pub fn serialize(game_state: GameState) -> String {
    let mut fen = FEN {
        squares: [[None; 8]; 8],
        active_color: ActiveColor::from_color(game_state.active_color),
        castle:  game_state.castle_rights.clone(),
        en_passant: game_state.en_passant_target_pos,
        half_move_clock: game_state.move_clock.half_move,
        full_move_clock: game_state.move_clock.full_move,
    };
    for (row_ix, &row) in game_state.board.as_slice().iter().rev().enumerate() {
        for (col_ix, &col) in row.iter().enumerate() {
            fen.squares[row_ix][col_ix] = *col;
        }
    }
    fen.to_string()
}

pub const fn serialize_without_clock_and_active_color(game_state: GameState) -> String {
    let serialized = serialize(game_state);
    let mut parts = serialized.split_whitespace().collect::<Vec<_>>();
    let (squares_str, _active_color_str, castle_rights_str, en_passant_str, _half_move_clock_str, _full_move_clock_str) = (parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]);
    [squares_str, castle_rights_str, en_passant_str].join(SPACE.to_string().as_str())
}
