use thiserror::Error;
use tracing::instrument;
use crate::notation::pgn::simple_parser::SimpleParserContext;
use crate::notation::pgn::util::LineWordPosTuple;

#[derive(Debug, Clone, Error, PartialEq)]
pub enum PgnParsingError {
    #[error("Invalid PGN - {1:?} @{2}:{3}:{0:?}")]
    InvalidPgn(String, String, usize, usize),
    #[error("Invalid PGN - {0}")]
    InvalidTagPair(String),
}
