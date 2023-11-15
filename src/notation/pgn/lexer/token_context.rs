use itertools::Itertools;
use crate::notation::pgn::lexer::token::Token;
use crate::notation::pgn::lexer::token_with_context::TokenWithContext;
use crate::notation::pgn::pgn_parsing_error::PgnParsingError;

// Cursor, or CursorContext might have been a better name
// but since we aren't actually using it when iterating over the data
// and its more of like a source map for a token, it's probably best left as TokenContext
#[derive(Debug, Default, Clone)]
pub struct TokenContext<'a> {
    data: &'a str,
    line_ix: usize,
    col_ix: usize,
}

impl<'a> TokenContext<'a> {
    pub(crate) fn new(data: &'a str) -> Self {
        Self {
            data,
            ..Default::default()
        }
    }
    pub(crate) fn update(&mut self, line_ix: usize, col_ix: usize) {
        self.line_ix = line_ix;
        self.col_ix = col_ix;
    }

    pub(crate) fn wrap_token(&self, token: Token) -> TokenWithContext<'a> {
        TokenWithContext(token, self.clone())
    }

    pub(crate) fn create_error(&self) -> PgnParsingError {
        let lines = self.data.lines().collect_vec();
        let line = lines.get(self.line_ix).expect("invalid line_ix");
        let section = &line[0..=self.col_ix];
        PgnParsingError::InvalidPgn(self.data.to_string(), section.to_string(), self.line_ix, self.col_ix)
    }
}
