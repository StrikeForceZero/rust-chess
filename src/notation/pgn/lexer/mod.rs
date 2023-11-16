use tracing::{instrument, trace};
use token_context::TokenContext;
use crate::notation::pgn::lexer::token::{WhiteSpace, Token, After, ws_new_line, ws_space};
use crate::notation::pgn::lexer::token_with_context::TokenWithContext;
use crate::utils::char;

pub mod token;
pub mod token_context;
pub mod token_with_context;

#[macro_export]
macro_rules! char_match {
    (rank) => { '1'..='8' };
    (file) => { 'a'..='h' };
    (promotion) => { 'Q' | 'B' | 'N' | 'R' };
    (from_piece) => { 'K' | 'Q' | 'B' | 'N' | 'R' };
}



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

#[derive(Debug)]
pub struct Lexer<'a> {
    state: LexerState<'a>,
}

impl<'a> Lexer<'a> {
    fn init(data: &'a str) -> Self {
        Self {
            state: LexerState::new(data),
        }
    }

    fn log_last(&self) {
        trace!("last: {:?}", self.state.tokens.last());
    }

    fn handle_char_after_newline(&mut self, &current_char: &char) {
        trace!("handle_char_after_newline - current_char: {current_char}");
        if current_char.is_ascii_digit() {
            self.state.push_token(Token::TurnBegin(String::from(current_char)));
        } else {
            match current_char {
                '[' => self.state.push_token(Token::TagPairStart(current_char)),
                char::NEW_LINE => self.state.push_token(ws_new_line(current_char, After::NewLine)),
                char::SPACE => self.state.push_token(ws_space(current_char, After::NewLine)),
                '0' | '1' | '2' | '/' | '-' | '*' => {
                    self.state.push_token(Token::MaybeTurnBeginOrContinuationOrMovingFromOrGameTermination(String::from(current_char)));
                }
                _ => self.state.push_token(Token::Unknown(String::from(current_char))),
            };
        }
        self.log_last();
    }

    fn handle_char_after_move(&mut self, &current_char: &char) {
        trace!("handle_char_after_move - current_char: {current_char}");
        match current_char {
            char::SPACE => {
                /* skip */
                trace!("skipping ' '");
            },
            '$' => {
                self.state.push_token(Token::Nag(String::from(current_char)));
            },
            '!' | '?' => {
                self.state.push_token(Token::MoveQuality(String::from(current_char)));
            }
            '(' => {
                // move variation
                todo!("implement")
            },
            '{' | ';' => {
                self.state.push_token(Token::AnnotationStart(current_char));
            }
            char_match!(from_piece) => {
                self.state.push_token(Token::PieceMoving(current_char));
            },
            '*' => {
                self.state.push_token(Token::GameTermination(String::from(current_char)));
            }
            '0' | '1' | '2' => {
                self.state.push_token(Token::MaybeTurnBeginOrContinuationOrMovingFromOrGameTermination(String::from(current_char)));
            },
            char_match!(file) | char_match!(rank) => {
                self.state.push_token(Token::MovingFrom(current_char))
            },
            _ => self.state.push_token(Token::Unknown(String::from(current_char))),
        }
        self.log_last();
    }

    fn handle_char_after_nag(&mut self, &current_char: &char) {
        trace!("handle_char_after_nag - current_char: {current_char}");
        if current_char.is_ascii_digit() {
            self.state.push_token(Token::MaybeTurnBeginOrContinuationOrMovingFromOrGameTermination(String::from(current_char)));
        } else {
            match current_char {
                char::SPACE => {
                    /* skip */
                    trace!("skipping ' '");
                },
                char::NEW_LINE => {
                    self.state.push_token(ws_new_line(current_char, After::Nag));
                }
                char_match!(promotion) => {
                    self.state.push_token(Token::PieceMoving(current_char));
                },
                char_match!(file) => {
                    self.state.push_token(Token::MovingFrom(current_char))
                },
                '*' => {
                    self.state.push_token(Token::GameTermination(String::from(current_char)));
                }
                '0' | '1' | '2' => {
                    self.state.push_token(Token::MaybeTurnBeginOrContinuationOrMovingFromOrGameTermination(String::from(current_char)));
                },
                char_match!(rank) => {
                    self.state.push_token(Token::MaybeTurnBeginOrContinuationOrMovingFromOrGameTermination(String::from(current_char)));
                },
                ';' => {
                    self.state.push_token(Token::Annotation(String::from(current_char)));
                }
                _ => {
                    self.state.push_token(Token::Unknown(String::from(current_char)))
                }
            }
        }
        self.log_last();
    }

    pub fn handle_char(&mut self, &current_char: &char) {
        trace!("handle_char - current_char: {current_char}");
        match self.state.tokens.last_mut() {
            None => {
                self.handle_char_after_newline(&current_char);
            }
            Some(ref mut token_with_context) => {
                let TokenWithContext(token, _) = token_with_context;
                trace!("matching last: {token:?}");
                match token {
                    Token::TagPairStart(_) => {
                        if current_char.is_ascii_alphabetic() {
                            self.state.push_token(Token::TagPairName(String::from(current_char)))
                        } else {
                            self.state.push_token(Token::Unknown(String::from(current_char)));
                        }
                    },
                    Token::TagPairName(ref mut str) => {
                        match current_char {
                            char::NEW_LINE => self.state.push_token(ws_new_line(current_char, After::TagPairName)),
                            char::SPACE => self.state.push_token(ws_space(current_char, After::TagPairName)),
                            ']' => self.state.push_token(Token::TagPairEnd(current_char)),
                            _ => str.push(current_char),
                        }
                    },
                    Token::TagPairValue(str_option) => {
                        match current_char {
                            ']' => self.state.push_token(Token::TagPairEnd(current_char)),
                            _ => str_option.push(current_char),
                        }
                    },
                    Token::TagPairEnd(_) => {
                        match current_char {
                            char::NEW_LINE => self.state.push_token(ws_new_line(current_char, After::TagPairEnd)),
                            char::SPACE => self.state.push_token(ws_space(current_char, After::TagPairEnd)),
                            _ => self.state.push_token(Token::Unknown(String::from(current_char))),
                        }
                    },
                    Token::TurnBegin(str) => {
                        // TODO: this might be too much validation
                        if current_char.is_ascii_digit() && str.chars().last().unwrap_or_default().is_ascii_digit() {
                            str.push(current_char)
                        } else {
                            match current_char {
                                '.' => {
                                    if str.chars().last().unwrap_or_default() == '.' {
                                        self.state.push_token(Token::Unknown(String::from(current_char)))
                                    } else {
                                        str.push(current_char);
                                    }
                                },
                                char::SPACE => self.state.push_token(ws_space(current_char, After::TurnBegin)),
                                _ => self.state.push_token(Token::Unknown(String::from(current_char)))
                            }
                        }
                    },
                    Token::PieceMoving(char) => {
                        match current_char {
                            char_match!(file) | char_match!(rank) => {
                                self.state.push_token(Token::MovingFrom(current_char))
                            },
                            _ => {
                                self.state.push_token(Token::Unknown(String::from(current_char)));
                            },
                        }
                    },
                    Token::MovingFrom(char) => {
                        match current_char {
                            char_match!(file) | char_match!(rank) => {
                                self.state.push_token(Token::MovingTo(String::from(current_char)))
                            },
                            'x' => {
                                self.state.push_token(Token::CaptureIndicator(current_char))
                            },
                            _ => {
                                self.state.push_token(Token::Unknown(String::from(current_char)));
                            }
                        }
                    },
                    Token::CaptureIndicator(char) => {
                        match current_char {
                            char_match!(file) | char_match!(rank) => {
                                self.state.push_token(Token::MovingTo(String::from(current_char)))
                            },
                            _ => {
                                self.state.push_token(Token::Unknown(String::from(current_char)));
                            }
                        }
                    }
                    Token::MovingTo(str) => {
                        match current_char {
                            char_match!(file) | char_match!(rank) => {
                                if str.len() <= 2 {
                                    self.state.push_token(Token::MovingTo(String::from(current_char)))
                                } else {
                                    self.state.push_token(Token::Unknown(String::from(current_char)));
                                }
                            },
                            char_match!(promotion) => {
                                self.state.push_token(Token::Promotion(current_char));
                            }
                            '=' | '(' | '/' => {
                                self.state.push_token(Token::PromotionStart(current_char));
                            }
                            '+' => {
                                self.state.push_token(Token::CheckIndicator(current_char))
                            }
                            '#' => {
                                self.state.push_token(Token::CheckMateIndicator(current_char))
                            }
                            char::SPACE => {
                                self.state.push_token(ws_space(current_char, After::MovingTo))
                            }
                            char::NEW_LINE => {
                                self.state.push_token(ws_new_line(current_char, After::MovingTo));
                            },
                            _ => {
                                self.state.push_token(Token::Unknown(String::from(current_char)));
                            }
                        }
                    },
                    Token::PromotionStart(char) => {
                        match current_char {
                            char_match!(promotion) => {
                                self.state.push_token(Token::Promotion(current_char));
                            }
                            _ => {
                                self.state.push_token(Token::Unknown(String::from(current_char)));
                            }
                        }
                    },
                    Token::Promotion(char) => {
                        match current_char {
                            ')' => {
                                self.state.push_token(Token::PromotionEnd(current_char));
                            }
                            '+' => {
                                self.state.push_token(Token::CheckIndicator(current_char))
                            }
                            '#' => {
                                self.state.push_token(Token::CheckMateIndicator(current_char))
                            }
                            char::SPACE => {
                                self.state.push_token(ws_space(current_char, After::Promotion));
                            }
                            char::NEW_LINE => {
                                self.state.push_token(ws_new_line(current_char, After::Promotion));
                            },
                            _ => {
                                self.state.push_token(Token::Unknown(String::from(current_char)));
                            }
                        }
                    },
                    Token::PromotionEnd(char) => {
                        match current_char {
                            '+' => {
                                self.state.push_token(Token::CheckIndicator(current_char))
                            }
                            '#' => {
                                self.state.push_token(Token::CheckMateIndicator(current_char))
                            }
                            char::SPACE => {
                                self.state.push_token(ws_space(current_char, After::PromotionEnd));
                            },
                            char::NEW_LINE => {
                                self.state.push_token(ws_new_line(current_char, After::PromotionEnd));
                            },
                            _ => {
                                self.state.push_token(Token::Unknown(String::from(current_char)));
                            }
                        }
                    },
                    Token::MoveQuality(str) => {
                        match current_char {
                            '!' | '?' => {
                                str.push(current_char);
                            },
                            char::SPACE => {
                                self.state.push_token(ws_space(current_char, After::MoveQuality));
                            }
                            char::NEW_LINE => {
                                self.state.push_token(ws_new_line(current_char, After::MoveQuality));
                            },
                            _ => {
                                self.state.push_token(Token::Unknown(String::from(current_char)));
                            }
                        }
                    },
                    Token::Nag(str) => {
                        if current_char.is_ascii_digit() {
                            str.push(current_char)
                        } else {
                            match current_char {
                                char::SPACE => {
                                    self.state.push_token(ws_space(current_char, After::Nag));
                                },
                                char::NEW_LINE => {
                                    self.state.push_token(ws_new_line(current_char, After::Nag));
                                },
                                _ => {
                                    self.state.push_token(Token::Unknown(String::from(current_char)));
                                }
                            }
                        }
                    },
                    Token::CheckIndicator(char) => {
                        match current_char {
                            char::SPACE => {
                                self.state.push_token(ws_space(current_char, After::CheckIndicator));
                            }
                            char::NEW_LINE => {
                                self.state.push_token(ws_new_line(current_char, After::CheckIndicator));
                            },
                            _ => {
                                self.state.push_token(Token::Unknown(String::from(current_char)));
                            }
                        }
                    },
                    Token::CheckMateIndicator(char) => {
                        match current_char {
                            char::SPACE => {
                                self.state.push_token(ws_space(current_char, After::CheckMateIndicator));
                            }
                            char::NEW_LINE => {
                                self.state.push_token(ws_new_line(current_char, After::CheckMateIndicator));
                            },
                            _ => {
                                self.state.push_token(Token::Unknown(String::from(current_char)));
                            }
                        }
                    },
                    Token::AnnotationStart(char) => {
                        match current_char {
                            _ => {
                                self.state.push_token(Token::Annotation(String::from(current_char)));
                            }
                        }
                    },
                    Token::Annotation(str) => {
                        match current_char {
                            char::NEW_LINE => {
                                self.state.push_token(ws_new_line(current_char, After::Annotation));
                            }
                            '}' => {
                                self.state.push_token(Token::AnnotationEnd(current_char));
                            }
                            _ => {
                                str.push(current_char);
                            }
                        }
                    },
                    Token::AnnotationEnd(char) => {
                        match current_char {
                            char::NEW_LINE => {
                                self.state.push_token(ws_new_line(current_char, After::AnnotationEnd));
                            }
                            char::SPACE => {
                                self.state.push_token(ws_space(current_char, After::AnnotationEnd));
                            }
                            ')' => {
                                // variation end
                                todo!("implement")
                            }
                            _ => {
                                self.state.push_token(Token::Unknown(String::from(current_char)));
                            }
                        }
                    },
                    Token::TurnContinuation(str) => {
                        // TODO: this might be too much validation
                        if current_char.is_ascii_digit() && str.chars().last().unwrap_or_default().is_ascii_digit() {
                            str.push(current_char)
                        } else {
                            match current_char {
                                '.' => {
                                    if str.ends_with("...") {
                                        self.state.push_token(Token::Unknown(String::from(current_char)))
                                    } else {
                                        str.push(current_char);
                                    }
                                },
                                char::SPACE => self.state.push_token(ws_space(current_char, After::TurnContinuation)),
                                _ => self.state.push_token(Token::Unknown(String::from(current_char)))
                            }
                        }
                    },
                    Token::GameTermination(str) => {
                        match current_char {
                            '0' | '1' | '2' | '/' | '-' => {
                                str.push(current_char)
                            },
                            char::SPACE | char::NEW_LINE => {/* skip */},
                            _ => {
                                self.state.push_token(Token::Unknown(String::from(current_char)));
                            }
                        }
                    },
                    Token::Unknown(ref mut str) => {
                        match current_char {
                            char::NEW_LINE => self.state.push_token(ws_new_line(current_char, After::Unknown)),
                            _ => str.push(current_char),
                        }
                    },
                    Token::WhiteSpace(_, white_space, white_space_token) => {
                        match white_space_token {
                            After::Space => {
                                /* skip */
                                trace!("skipping ' '");
                            },
                            After::NewLine => self.handle_char_after_newline(&current_char),
                            After::TagPairName => {
                                match current_char {
                                    char::SPACE => {
                                        /* skip */
                                        trace!("skipping ' '");
                                    },
                                    '"' => {
                                        self.state.push_token(Token::TagPairValue(String::from(current_char)));
                                    },
                                    _ => {
                                        self.state.push_token(Token::Unknown(String::from(current_char)));
                                    }
                                }
                            }
                            After::TagPairEnd => {
                                match current_char {
                                    char::SPACE => {
                                        /* skip */
                                        trace!("skipping ' '");
                                    },
                                    char::NEW_LINE => {
                                        let after = match white_space {
                                            WhiteSpace::NewLine => After::NewLine,
                                            WhiteSpace::Space => After::Space,
                                        };
                                        self.state.push_token(ws_new_line(current_char, after))
                                    },
                                    _ => self.handle_char_after_newline(&current_char),
                                }
                            }
                            After::TurnBegin => {
                                match current_char {
                                    char::SPACE => {
                                        /* skip */
                                        trace!("skipping ' '");
                                    },
                                    char_match!(from_piece) => {
                                        self.state.push_token(Token::PieceMoving(current_char));
                                    },
                                    char_match!(file) | char_match!(rank) => {
                                        self.state.push_token(Token::MovingFrom(current_char))
                                    },
                                    _ => self.state.push_token(Token::Unknown(String::from(current_char)))
                                }
                            }
                            After::MovingTo => {
                                self.handle_char_after_move(&current_char);
                            }
                            After::Promotion => {
                                self.handle_char_after_move(&current_char);
                            }
                            After::PromotionEnd => {
                                self.handle_char_after_move(&current_char);
                            }
                            After::CheckIndicator => {
                                self.handle_char_after_move(&current_char);
                            }
                            After::CheckMateIndicator => {
                                self.handle_char_after_move(&current_char);
                            }
                            After::AnnotationEnd => {
                                if current_char.is_ascii_digit() {
                                    self.state.push_token(Token::MaybeTurnBeginOrContinuationOrMovingFromOrGameTermination(String::from(current_char)));
                                } else {
                                    match current_char {
                                        char::SPACE => {
                                            /* skip */
                                            trace!("skipping ' '");
                                        },
                                        char::NEW_LINE => {
                                            self.state.push_token(ws_new_line(current_char, After::AnnotationEnd));
                                        }
                                        char_match!(from_piece) => {
                                            self.state.push_token(Token::PieceMoving(current_char));
                                        },
                                        char_match!(file) => {
                                            self.state.push_token(Token::MovingFrom(current_char))
                                        },
                                        '*' => {
                                            self.state.push_token(Token::GameTermination(String::from(current_char)));
                                        }
                                        '0' | '1' | '2' => {
                                            self.state.push_token(Token::MaybeTurnBeginOrContinuationOrMovingFromOrGameTermination(String::from(current_char)));
                                        },
                                        char_match!(rank) => {
                                            self.state.push_token(Token::MaybeTurnBeginOrContinuationOrMovingFromOrGameTermination(String::from(current_char)));
                                        },
                                        ';' => {
                                            self.state.push_token(Token::Annotation(String::from(current_char)));
                                        }
                                        _ => {
                                            self.state.push_token(Token::Unknown(String::from(current_char)))
                                        }
                                    }
                                }
                            }
                            After::MoveQuality => {
                                self.handle_char_after_nag(&current_char);
                            }
                            After::Nag => {
                                self.handle_char_after_nag(&current_char);
                            }
                            After::TurnContinuation => {
                                match current_char {
                                    char::SPACE => {
                                        /* skip */
                                        trace!("skipping ' '");
                                    },
                                    char_match!(file) | char_match!(rank) => {
                                        self.state.push_token(Token::MovingFrom(current_char))
                                    },
                                    _ => {
                                        self.state.push_token(Token::Unknown(String::from(current_char)));
                                    }
                                }
                            }
                            After::Annotation => {
                                self.handle_char_after_newline(&current_char);
                            }
                            After::Unknown => {
                                self.handle_char_after_newline(&current_char);
                            }
                        }
                    }
                    Token::MaybeTurnBeginOrContinuationOrMovingFromOrGameTermination(str) => {
                        if current_char.is_ascii_digit() && str.chars().last().unwrap_or_default().is_ascii_digit() {
                            str.push(current_char)
                        } else {
                            match current_char {
                                '/' | '-' | '*' => {
                                    trace!("replacing with GameTermination");
                                    *token = Token::GameTermination(format!("{str}{current_char}"));
                                }
                                '.' => {
                                    if str.ends_with("...") {
                                        self.state.push_token(Token::Unknown(String::from(current_char)))
                                    } else {
                                        str.push(current_char);
                                    }
                                },
                                char::SPACE => {
                                    let first_char_is_digit = str.chars().next().unwrap_or_default().is_ascii_digit();
                                    if first_char_is_digit && str.ends_with("...") {
                                        trace!("replacing with TurnContinuation");
                                        *token = Token::TurnContinuation(format!("{str}"));
                                        self.state.push_token(ws_space(current_char, After::TurnContinuation));
                                    } else if first_char_is_digit && str.ends_with('.') {
                                        trace!("replacing with TurnBegin");
                                        *token = Token::TurnBegin(format!("{str}"));
                                        self.state.push_token(ws_space(current_char, After::TurnBegin));
                                    } else {
                                        trace!("replacing with Unknown");
                                        *token = Token::Unknown(format!("{str}{current_char}"));
                                    }
                                },
                                char_match!(file) | char_match!(rank) => {
                                    if str.len() == 1 {
                                        trace!("replacing with MovingFrom");
                                        *token = Token::MovingFrom(str.chars().next().expect("impossible"));
                                        self.state.push_token(Token::MovingTo(String::from(current_char)));
                                    } else {
                                        trace!("replacing with Unknown");
                                        *token = Token::Unknown(format!("{str}{current_char}"));
                                    }
                                },
                                char::NEW_LINE => {
                                    trace!("replacing with Unknown");
                                    *token = Token::Unknown(str.clone());
                                    self.state.push_token(ws_new_line(current_char, After::Unknown));
                                },
                                _ => {
                                    trace!("replacing with Unknown");
                                    *token = Token::Unknown(format!("{str}{current_char}"));
                                },
                            }
                        }
                    }
                }
            }
        }
        self.log_last();
    }

    pub fn lex(data: &'a str) -> Vec<TokenWithContext<'a>> {
        let mut lexer = Lexer::init(data);
        for (line_ix, line) in data.lines().enumerate() {
            for (char_ix, char) in line.chars().enumerate() {
                lexer.state.context.update(line_ix, char_ix);
                lexer.handle_char(&char);
            }
            lexer.state.context.update(line_ix, line.len());
            lexer.handle_char(&char::NEW_LINE);
        }
        lexer.state.tokens
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use super::*;
    use rstest::rstest;
    use crate::utils::tracing::init_tracing;
    use token::Token::*;
    use token::After::*;

    #[rstest]
    #[case(
        "[Event \"Some Event\"]\n\
        \n\
        1. e4 d5 *",
        vec![
            TagPairStart('['),
            Token::TagPairName("Event".into()),
            ws_space(char::SPACE, After::TagPairName),
            TagPairValue("\"Some Event\"".into()),
            Token::TagPairEnd(']'),
            ws_new_line(char::NEW_LINE, After::TagPairEnd),
            ws_new_line(char::NEW_LINE, NewLine),
            Token::TurnBegin("1.".into()),
            ws_space(char::SPACE, After::TurnBegin),
            MovingFrom('e'),
            Token::MovingTo("4".into()),
            ws_space(char::SPACE, After::MovingTo),
            MovingFrom('d'),
            Token::MovingTo("5".into()),
            ws_space(char::SPACE, After::MovingTo),
            GameTermination("*".into()),
        ],
    )]
    #[case(
        "[Event \"Some Event\"]\n\
        \n\
        1. e4 {white comment} 1... d5 {black comment} 2. e4 !! d5 !? ; end of line comment\n\
        3. Qh8# $1\n\
        1-0",
        vec![
            TagPairStart('['),
            Token::TagPairName("Event".into()),
            ws_space(char::SPACE, After::TagPairName),
            TagPairValue("\"Some Event\"".into()),
            Token::TagPairEnd(']'),
            ws_new_line(char::NEW_LINE, After::TagPairEnd),
            ws_new_line(char::NEW_LINE, NewLine),
            Token::TurnBegin("1.".into()),
            ws_space(char::SPACE, After::TurnBegin),
            MovingFrom('e'),
            Token::MovingTo("4".into()),
            ws_space(char::SPACE, After::MovingTo),
            AnnotationStart('{'),
            Token::Annotation("white comment".into()),
            Token::AnnotationEnd('}'),
            ws_space(char::SPACE, After::AnnotationEnd),
            Token::TurnContinuation("1...".into()),
            ws_space(char::SPACE, After::TurnContinuation),
            MovingFrom('d'),
            Token::MovingTo("5".into()),
            ws_space(char::SPACE, After::MovingTo),
            AnnotationStart('{'),
            Token::Annotation("black comment".into()),
            Token::AnnotationEnd('}'),
            ws_space(char::SPACE, After::AnnotationEnd),
            Token::TurnBegin("2.".into()),
            ws_space(char::SPACE, After::TurnBegin),
            MovingFrom('e'),
            Token::MovingTo("4".into()),
            ws_space(char::SPACE, After::MovingTo),
            Token::MoveQuality("!!".into()),
            ws_space(char::SPACE, After::MoveQuality),
            MovingFrom('d'),
            Token::MovingTo("5".into()),
            ws_space(char::SPACE, After::MovingTo),
            Token::MoveQuality("!?".into()),
            ws_space(char::SPACE, After::MoveQuality),
            Token::Annotation("; end of line comment".into()),
            ws_new_line(char::NEW_LINE, After::Annotation),
            Token::TurnBegin("3.".into()),
            ws_space(char::SPACE, After::TurnBegin),
            PieceMoving('Q'),
            MovingFrom('h'),
            Token::MovingTo("8".into()),
            Token::CheckMateIndicator('#'),
            ws_space(char::SPACE, After::CheckMateIndicator),
            Token::Nag("$1".into()),
            ws_new_line(char::NEW_LINE, After::Nag),
            GameTermination("1-0".into()),
        ],
    )]
    fn test_lex(
        #[case] input: &'static str,
        #[case] expected: Vec<Token>,
    ) {
        init_tracing();
        let tokens_with_context = Lexer::lex(input);
        let tokens = tokens_with_context.iter().map(|TokenWithContext(token, _)| token).collect_vec();
        assert_eq!(expected.iter().collect_vec(), tokens);
    }
}

