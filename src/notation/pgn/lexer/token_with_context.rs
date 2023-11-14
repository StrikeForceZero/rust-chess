use crate::notation::pgn::lexer::token::Token;
use crate::notation::pgn::lexer::token_context::TokenContext;

#[derive(Debug)]
pub struct TokenWithContext<'a>(pub Token, pub TokenContext<'a>);
