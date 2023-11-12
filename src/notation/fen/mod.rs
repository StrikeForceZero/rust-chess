pub mod serialize;
pub mod deserialize;
mod fen_parts;

pub use serialize::serialize;
pub use deserialize::deserialize;

use std::fmt::{Display, Formatter};
use crate::color::Color;
use crate::notation::fen::deserialize::{FenParsingError, get_parts};
use crate::notation::fen::fen_parts::FenParts;

pub const FEN_STARTING_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const FEN_EMPTY: &str = "8/8/8/8/8/8/8/8 w - - 0 1";

const BOARD_TERMINATOR: &str = "/";

#[derive(Clone, Debug)]
pub enum Fen {
    Static(&'static str),
    Owned(String),
}

impl Fen {
    pub fn get_str(&self) -> &str {
        match self {
            Fen::Static(s) => s,
            Fen::Owned(s) => s.as_ref(),
        }
    }

    pub fn get_parts(&self) -> Result<FenParts, FenParsingError> {
        get_parts(self.get_str())
    }
}

impl PartialEq<Fen> for Fen {
    fn eq(&self, other: &Fen) -> bool {
        self.get_str() == other.get_str()
    }
}

impl PartialEq<String> for Fen {
    fn eq(&self, other: &String) -> bool {
        self.get_str() == other
    }
}

impl PartialEq<str> for Fen {
    fn eq(&self, other: &str) -> bool {
        self.get_str() == other
    }
}

impl Display for Fen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_str())
    }
}

#[derive(Copy, Clone)]
pub(in crate::notation::fen) enum ActiveColor {
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

