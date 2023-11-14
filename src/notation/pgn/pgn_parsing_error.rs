use thiserror::Error;
use tracing::instrument;
use crate::notation::pgn::parser::{LineWordPosTuple, ParsingContext};

#[derive(Debug, Clone, Error, PartialEq)]
pub enum PgnParsingError {
    #[error("Invalid PGN - {1:?} @{2}:{3}:{0:?}")]
    InvalidPgn(String, String, usize, usize),
    #[error("Invalid PGN - {0}")]
    InvalidTagPair(String),
}

impl PgnParsingError {
    #[instrument]
    pub fn create(parsing_context: &ParsingContext) -> Self {
        let LineWordPosTuple(line, word, col) = parsing_context.resolve_line_word_pos_tuple();
        Self::InvalidPgn(line, word, parsing_context.line_ix + 1, col + 1)
    }
}
