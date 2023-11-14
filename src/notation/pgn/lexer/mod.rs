use token::Token;
use token_context::TokenContext;
use crate::notation::pgn::lexer::token_with_context::TokenWithContext;

mod token;
mod token_context;
mod token_with_context;

#[derive(Debug, Default)]
pub struct LexerState<'a> {
    context: TokenContext<'a>,
    tokens: Vec<TokenWithContext<'a>>,
}

impl<'a> LexerState<'a> {
    pub(crate) fn new(data: &'a str) -> Self {
        Self {
            context: TokenContext::new(data),
            ..Default::default()
        }
    }

    pub(crate) fn push_token(&mut self, token: Token) {
        self.tokens.push(self.context.wrap_token(token))
    }
}

pub struct Lexer<'a> {
    state: LexerState<'a>,
}

impl<'a> Lexer<'a> {
    fn init(data: &'a str) -> Self {
        Self {
            state: LexerState::new(data),
        }
    }
    pub fn handle_char(&mut self, &current_char: &char) {
        match self.state.tokens.last_mut() {
            None => {
                match current_char {
                    '[' => self.state.push_token(Token::TagPairStart(current_char)),
                    crate::utils::char::SPACE | crate::utils::char::NEW_LINE => { /* skip */},
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => self.state.push_token(Token::TurnBegin(format!("{current_char}"))),
                    _ => self.state.push_token(Token::Unknown(format!("{current_char}"))),
                };
            }
            Some(ref mut token_with_context) => {
                let TokenWithContext(token, _) = token_with_context;
                match token {
                    Token::TagPairStart(_) => {
                        self.state.push_token(Token::TagPairName(format!("{current_char}")))
                    },
                    Token::TagPairName(ref mut str) => {
                        match current_char {
                            crate::utils::char::NEW_LINE => self.state.push_token(Token::NewLine),
                            crate::utils::char::SPACE => self.state.push_token(Token::TagPairValue(String::new())),
                            ']' => self.state.push_token(Token::TagPairEnd(current_char)),
                            _ => str.push(current_char),
                        }
                    },
                    Token::TagPairValue(str) => {
                        match current_char {
                            ']' => self.state.push_token(Token::TagPairEnd(current_char)),
                            _ => str.push(current_char),
                        }
                    },
                    Token::TagPairEnd(_) => {},
                    Token::TurnBegin(str) => {},
                    Token::PieceMoving(char) => {},
                    Token::MovingFrom(char) => {},
                    Token::CaptureIndicator => {}
                    Token::MovingTo(str) => {},
                    Token::CheckIndicator(char) => {},
                    Token::CheckMateIndicator(char) => {},
                    Token::PromotionStart(char) => {},
                    Token::Promotion(char) => {},
                    Token::PromotionEnd(char) => {},
                    Token::AnnotationStart(char) => {},
                    Token::Annotation(str) => {},
                    Token::AnnotationEnd(char) => {},
                    Token::MoveQuality(str) => {},
                    Token::Nag(str) => {},
                    Token::TurnContinuation(str) => {},
                    Token::GameTermination(str) => {},
                    Token::Unknown(ref mut str) => {
                        str.push(current_char);
                    },
                    Token::NewLine => {},
                }
            }
        }
    }

    pub fn lex(data: &'a str) -> Vec<TokenWithContext<'a>> {
        let mut lexer = Lexer::init(data);
        for (line_ix, line) in data.lines().enumerate() {
            for (char_ix, char) in line.chars().enumerate() {
                lexer.state.context.update(line_ix, char_ix);
                lexer.handle_char(&char);
            }
            lexer.state.context.update(line_ix, line.len());
            lexer.handle_char(&crate::utils::char::NEW_LINE);
        }
        lexer.state.tokens
    }
}

