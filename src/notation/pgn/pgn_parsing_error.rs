use thiserror::Error;
use tracing::instrument;
use crate::notation::pgn::simple_parser::SimpleParserContext;
use crate::notation::pgn::tag_pairs::PgnTagPairParseError;
use crate::notation::pgn::util::LineWordPosTuple;

#[derive(Debug, Clone, Error, PartialEq)]
pub enum PgnParsingError {
    #[error("Invalid PGN - {1:?} @{2}:{3}:{0:?}")]
    InvalidPgn(String, String, usize, usize),
    #[error("Invalid PGN - {0}")]
    InvalidTagPair(String),
    #[error("Invalid PGN - roster missing required field {0}")]
    RosterMissingRequiredField(&'static str),
}



impl From<PgnTagPairParseError> for PgnParsingError {
    fn from(value: PgnTagPairParseError) -> Self {
        let message = match value {
            PgnTagPairParseError::FailedParsingTagPair(name, data) => format!("Failed parsing tag {name} ({data})"),
            PgnTagPairParseError::InvalidTagString(data) => data,
            PgnTagPairParseError::UnknownTagPair(data) => data,
        };
        Self::InvalidTagPair(message)
    }
}
