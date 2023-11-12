use crate::color::Color;
use crate::direction::castle_side::CastleSide;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum CastleRightsStringParseError {
    #[error("Invalid castle rights string, expects to be any combination of K | q | Q | q | KQ | kq, received: {0}")]
    InvalidCastleRightsString(String),
}

#[derive(Clone, Copy, Debug)]
pub enum CastleRights {
    KingSideOnly,
    QueenSideOnly,
    Both,
}

impl CastleRights {
    pub const fn from_castle_side(castle_side: CastleSide) -> Self {
        match castle_side {
            CastleSide::King => CastleRights::KingSideOnly,
            CastleSide::Queen => CastleRights::QueenSideOnly,
        }
    }
    pub const fn has(&self, check_castle_rights: CastleRights) -> bool {
        match (self, check_castle_rights) {
            (Self::Both, _) => true,
            (Self::KingSideOnly, Self::KingSideOnly) => true,
            (Self::QueenSideOnly, Self::QueenSideOnly) => true,
            (_, _) => false,
        }
    }
    pub const fn without(&self, without_castle_rights: CastleRights) -> Option<Self> {
        Some(match (without_castle_rights, self) {
            (Self::KingSideOnly, Self::Both | Self::QueenSideOnly) => CastleRights::QueenSideOnly,
            (Self::QueenSideOnly, Self::Both | Self::KingSideOnly) => CastleRights::KingSideOnly,
            (Self::KingSideOnly, Self::KingSideOnly) => return None,
            (Self::QueenSideOnly, Self::QueenSideOnly) => return None,
            (Self::Both, _) => return None,
        })
    }
    pub const fn with(&self, with_castle_rights: CastleRights) -> Self {
        match (self, with_castle_rights) {
            (Self::KingSideOnly, Self::QueenSideOnly)
            | (Self::QueenSideOnly, Self::KingSideOnly) => CastleRights::Both,
            (Self::Both, _) | (_, Self::Both) => CastleRights::Both,
            (_, right) => right,
        }
    }
    pub const fn with_option(
        source: Option<CastleRights>,
        with_castle_rights: CastleRights,
    ) -> Option<Self> {
        Some(match source {
            None => with_castle_rights,
            Some(cr) => cr.with(with_castle_rights),
        })
    }
    pub const fn as_str(&self, color: Color) -> &'static str {
        match color {
            Color::White => match self {
                Self::KingSideOnly => "K",
                Self::QueenSideOnly => "Q",
                Self::Both => "KQ",
            },
            Color::Black => match self {
                Self::KingSideOnly => "k",
                Self::QueenSideOnly => "q",
                Self::Both => "kq",
            },
        }
    }
    pub fn from_str(s: &str) -> Result<Self, CastleRightsStringParseError> {
        Ok(match s {
            "K" | "k" => Self::KingSideOnly,
            "Q" | "q" => Self::QueenSideOnly,
            "KQ" | "kq" => Self::Both,
            _ => return Err(CastleRightsStringParseError::InvalidCastleRightsString(s.to_string()))
        })
    }
}
