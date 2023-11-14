use token::Token;
use token_context::TokenContext;

mod token;
mod token_context;

#[derive(Debug, Default)]
pub struct LexerState {
    tokens: Vec<Token>,
}

pub struct Lexer {
    state: LexerState,
}

impl Lexer {
    pub fn handle_char(&mut self, context: &TokenContext, char: &char) {
        match self.state.tokens.last() {
            None => {}
            Some(token) => {
                match token {
                    Token::TagPairStart(char) => {},
                    Token::TagPairName(str) => {},
                    Token::TagPairValue(str) => {},
                    Token::TagPairEnd(char) => {},
                    Token::TurnBegin(str) => {},
                    Token::PieceMoving(char) => {},
                    Token::MovingFrom(char) => {},
                    Token::Capture => {},
                    Token::MovingTo(str) => {},
                    Token::CheckOrMateIndicator(char) => {},
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
                    Token::Unknown(str) => {},
                    Token::NewLine => {},
                }
            }
        }
    }

    pub fn lex(data: &str) -> Vec<Token> {
        let mut lexer = Self {
            state: LexerState::default(),
        };
        let mut context = TokenContext::new(data);
        for (line_ix, line) in data.lines().enumerate() {
            for (char_ix, char) in line.chars().enumerate() {
                context.update(line_ix, char_ix);
                lexer.handle_char(&context, &char);
            }
        }
        lexer.state.tokens
    }
}

