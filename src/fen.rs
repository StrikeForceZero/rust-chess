use thiserror::Error;
use crate::board::Board;
use crate::board_position::{BoardPosition, BoardPositionStrParseError};
use crate::board_rank::BoardRank;
use itertools::Itertools;
use crate::chess_piece::ChessPiece;
use crate::color::Color;
use crate::color_castle_rights::ColorCastleRights;

#[repr(transparent)]
pub struct Fen(String);

#[derive(Error, Debug, Clone)]
pub enum FenParsingError {
    #[error("Invalid active color for Fen, active color expects to be W | B, received: {0}")]
    InvalidActiveColorString(String),
    #[error("Invalid active color for Fen, active color expects to be W | B, received: {0}")]
    InvalidActiveColorChar(char),
}


fn get_active_color_from_str(active_color_str: &str) -> Result<ActiveColor, FenParsingError> {
    if active_color_str.len() != 1 {
        return Err(FenParsingError::InvalidActiveColorString(active_color_str.to_string()));
    }
    let active_color_chars = active_color_str.chars().take(1).collect::<Vec<_>>();
    let &active_color_char = active_color_chars.get(0).unwrap();
    ActiveColor::from_char(active_color_char)
}

fn get_en_passant_pos_from_str(en_passant_str: &str) -> Result<Option<BoardPosition>, BoardPositionStrParseError> {
    if en_passant_str.len() == 0 || en_passant_str == "-" {
        return Ok(None);
    }
    Ok(Some(BoardPosition::from_str(en_passant_str)?))
}

#[derive(Error, Debug, Copy, Clone)]
enum ApplyError {

}

const BOARD_TERMINATOR: &str = "/";
pub fn apply(board: &mut Board, fen_str: &str) -> Result<(), ApplyError> {
    let parts = fen_str.split_whitespace().collect::<Vec<_>>();
    let mut board_state = board.get_state_mut();
    // TODO: implement remaining
    let (squares_str, active_color_str, castle_rights_str, en_passant_str, half_move_clock_str, full_move_clock_str) = (parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]);
    board_state.color_to_move = get_active_color_from_str(active_color_str)?.as_color();
    let castle_rights = ColorCastleRights::from_str(castle_rights_str);
    board_state.white_castling_rights = castle_rights.white;
    board_state.black_castling_rights = castle_rights.black;
    board_state.half_move_clock = half_move_clock_str.parse::<u16>().expect("invalid half_move_clock_str");
    board_state.full_move_clock = full_move_clock_str.parse::<u16>().expect("invalid full_move_clock_str");
    board_state.en_passant = get_en_passant_pos_from_str(en_passant_str);
    let rows = squares_str.split_terminator(BOARD_TERMINATOR).collect::<Vec<_>>();
    // flip so white is on bottom
    let rows = rows.iter().rev();
    for (row_ix, row) in rows.enumerate() {
        let rank_u8 = u8::try_from(row_ix).expect("row_ix overflow");
        let rank = BoardRank::from_u8(rank_u8);
        let chars = row.chars();
        let mut offset: u8 = 0;
        for (char_ix, char) in chars.enumerate() {
            let file_u8 = u8::try_from(char_ix).expect("char_ix overflow") + offset;
            let mut file = BoardRank::from_u8(file_u8);

            if let Ok(blanks) = format!("{char}").parse::<u8>() {
                offset += blanks - 1;
                let mut remaining = blanks;
                while remaining > 0 {
                    let pos = BoardPosition(file, rank);
                    board.set(pos, None);
                    remaining -= 1;
                    if remaining > 0 {
                        file = if let Some(next_file) = file.next() { next_file } else {
                            panic!("invalid fen string - given: {blanks}, overflow: {remaining}");
                        };
                    }
                }
                continue;
            }
            match ChessPiece::try_from_fen(&char) {
                Ok(piece) => board.set(BoardPosition(file, rank), Some(piece)),
                Err(err) => panic!("{err}"),
            }
        }
    }
    Ok(())
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
            if self.en_passant.is_some() { self.en_passant.unwrap().to_string() } else { DASH.to_string() },
            self.half_move_clock.to_string(),
            self.full_move_clock.to_string(),
        ]
            .iter()
            .filter(|x| !x.is_empty())
            .join(SPACE.to_string().as_str());

        return str;
    }
}

pub fn serialize(board: &Board) -> String {
    let en_passant = board.get_state().en_passant;
    let mut en_passant_string: Option<String> = None;
    if let Some(pos) = en_passant {
        en_passant_string = Some(pos.to_string().to_lowercase());
    }
    let mut fen = FEN {
        squares: [[None; 8]; 8],
        active_color: ActiveColor::from_color(board.get_state().color_to_move),
        castle:  FenCastle {
            white: board.get_state().white_castling_rights,
            black: board.get_state().black_castling_rights,
        },
        // TODO: this is dumb
        en_passant: en_passant_string,
        half_move_clock: board.get_state().half_move_clock,
        full_move_clock: board.get_state().full_move_clock,
    };
    for (row_ix, &row) in board.as_slice().iter().rev().enumerate() {
        for (col_ix, &col) in row.iter().enumerate() {
            fen.squares[row_ix][col_ix] = *col;
        }
    }
    fen.to_string()
}

pub fn serialize_without_clock_and_active_color(board: &Board) -> String {
    let serialized = serialize(board);
    let mut parts = serialized.split_whitespace().collect::<Vec<_>>();
    let (squares_str, _active_color_str, castle_rights_str, en_passant_str, _half_move_clock_str, _full_move_clock_str) = (parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]);
    [squares_str, castle_rights_str, en_passant_str].join(SPACE.to_string().as_str())
}
