use crate::castle_rights::{CastleRights, CastleRightsStringParseError};
use crate::color::Color;

#[derive(Clone)]
pub struct ColorCastleRights {
    pub white: Option<CastleRights>,
    pub black: Option<CastleRights>,
}

impl ColorCastleRights {
    pub const fn empty() -> Self {
        Self {
            white: None,
            black: None,
        }
    }
    pub const fn new() -> Self {
        Self {
            white: Some(CastleRights::Both),
            black: Some(CastleRights::Both),
        }
    }
    pub const fn for_color(&self, color: Color) -> Option<CastleRights> {
        match color {
            Color::White => self.white,
            Color::Black => self.black,
        }
    }
    const fn to_tuple(&self) -> (&Option<CastleRights>, &Option<CastleRights>) {
        (&self.white, &self.black)
    }
    const fn from_tuple(tuple: (Option<CastleRights>, Option<CastleRights>)) -> Self {
        let (white, black) = tuple;
        Self {
            white,
            black,
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self.to_tuple() {
            (None, None) => "-",
            (None, Some(black_cr)) => black_cr.as_str(Color::Black),
            (Some(white_cr), None) => white_cr.as_str(Color::White),
            // TODO: maintaining &'static str is probably not worth it
            (Some(white_cr), Some(black_cr)) => match (white_cr, black_cr) {
                (CastleRights::Both, CastleRights::Both) => "KQkq",
                (CastleRights::KingSideOnly, CastleRights::Both) => "Kkq",
                (CastleRights::QueenSideOnly, CastleRights::Both) => "Qkq",
                (CastleRights::Both, CastleRights::KingSideOnly) => "KQk",
                (CastleRights::Both, CastleRights::QueenSideOnly) => "KQq",
                (CastleRights::KingSideOnly, CastleRights::KingSideOnly) => "Kk",
                (CastleRights::QueenSideOnly, CastleRights::QueenSideOnly) => "Qq",
                (CastleRights::KingSideOnly, CastleRights::QueenSideOnly) => "Kq",
                (CastleRights::QueenSideOnly, CastleRights::KingSideOnly) => "Qk",
            },
        }
    }

    pub fn from_str(s: &str) -> Result<Self, CastleRightsStringParseError> {
        // TODO: being this explicit is probably not worth it
        Ok(Self::from_tuple(match s {
            "-" => return Ok(Self::empty()),
            "KQkq" => (Some(CastleRights::Both), Some(CastleRights::Both)),
            "Kkq" => (Some(CastleRights::KingSideOnly), Some(CastleRights::Both)),
            "Qkq" => (Some(CastleRights::QueenSideOnly), Some(CastleRights::Both)),
            "KQk" => (Some(CastleRights::Both), Some(CastleRights::KingSideOnly)),
            "KQq" => (Some(CastleRights::Both), Some(CastleRights::QueenSideOnly)),
            "Kk" => (Some(CastleRights::KingSideOnly), Some(CastleRights::KingSideOnly)),
            "Qq" => (Some(CastleRights::QueenSideOnly), Some(CastleRights::QueenSideOnly)),
            "Kq" => (Some(CastleRights::KingSideOnly), Some(CastleRights::QueenSideOnly)),
            "Qk" => (Some(CastleRights::QueenSideOnly), Some(CastleRights::KingSideOnly)),
            "KQ" => (Some(CastleRights::Both), None),
            "K" => (Some(CastleRights::KingSideOnly), None),
            "Q" => (Some(CastleRights::QueenSideOnly), None),
            "kq" => (None, Some(CastleRights::Both)),
            "k" => (None, Some(CastleRights::KingSideOnly)),
            "q" => (None, Some(CastleRights::QueenSideOnly)),
            // TODO: InvalidCastleRightsString does not mention "-"
            _ => return Err(CastleRightsStringParseError::InvalidCastleRightsString(s.to_string()))
        }))
    }
}
