use std::fmt::{Display, Formatter};
use crate::notation::pgn::tag_pairs::PgnTagPairParseError;

#[derive(Debug, PartialEq)]
pub enum PgnTagPairResult {
    WhiteWon,
    BlackWon,
    Draw,
    GameInProgress,
}

impl PgnTagPairResult {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::WhiteWon => "1-0",
            Self::BlackWon => "0-1",
            Self::Draw => "1/2-1/2",
            Self::GameInProgress => "*",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, PgnTagPairParseError> {
        Ok(match s {
            "1-0" => Self::WhiteWon,
            "0-1" => Self::BlackWon,
            "1/2-1/2" => Self::Draw,
            "*" => Self::GameInProgress,
            _ => return Err(Self::create_parsing_error(s))
        })
    }
}

// for easier copy paste
type ThisPgnTagPair = PgnTagPairResult;
const NAME: &str = "Result";
super::impl_named_tag_pair_for!(ThisPgnTagPair, NAME);

impl Display for PgnTagPairResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
