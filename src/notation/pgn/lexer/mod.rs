use tracing::{instrument, trace};
use token::Token;
use token_context::TokenContext;
use crate::notation::pgn::lexer::token::WhiteSpaceToken;
use crate::notation::pgn::lexer::token_with_context::TokenWithContext;
use crate::utils::char;

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
                char::NEW_LINE => self.state.push_token(Token::NewLine),
                char::SPACE => self.state.push_token(Token::WhiteSpace(WhiteSpaceToken::AfterNewLine)),
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
            'K' | 'Q' | 'B' | 'N' | 'R' => {
                self.state.push_token(Token::PieceMoving(current_char));
            },
            '*' => {
                self.state.push_token(Token::GameTermination(String::from(current_char)));
            }
            '0' | '1' | '2' => {
                self.state.push_token(Token::MaybeTurnBeginOrContinuationOrMovingFromOrGameTermination(String::from(current_char)));
            },
            'a'..='h' | '1'..='8' => {
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
                    self.state.push_token(Token::NewLine);
                }
                'K' | 'Q' | 'B' | 'N' | 'R' => {
                    self.state.push_token(Token::PieceMoving(current_char));
                },
                'a'..='h' => {
                    self.state.push_token(Token::MovingFrom(current_char))
                },
                '*' => {
                    self.state.push_token(Token::GameTermination(String::from(current_char)));
                }
                '0' | '1' | '2' => {
                    self.state.push_token(Token::MaybeTurnBeginOrContinuationOrMovingFromOrGameTermination(String::from(current_char)));
                },
                '1'..='9' => {
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
                            char::NEW_LINE => self.state.push_token(Token::NewLine),
                            char::SPACE => self.state.push_token(Token::WhiteSpace(WhiteSpaceToken::AfterTagPairName)),
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
                            char::NEW_LINE => self.state.push_token(Token::NewLine),
                            char::SPACE => self.state.push_token(Token::WhiteSpace(WhiteSpaceToken::AfterTagPairEnd)),
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
                                char::SPACE => self.state.push_token(Token::WhiteSpace(WhiteSpaceToken::AfterTurnBegin)),
                                _ => self.state.push_token(Token::Unknown(String::from(current_char)))
                            }
                        }
                    },
                    Token::PieceMoving(char) => {
                        match current_char {
                            'a'..='h' | '1'..='8' => {
                                self.state.push_token(Token::MovingFrom(current_char))
                            },
                            _ => {
                                self.state.push_token(Token::Unknown(String::from(current_char)));
                            },
                        }
                    },
                    Token::MovingFrom(char) => {
                        match current_char {
                            'a'..='h' | '1'..='8' => {
                                self.state.push_token(Token::MovingTo(String::from(current_char)))
                            },
                            'x' => {
                                self.state.push_token(Token::CaptureIndicator)
                            },
                            _ => {
                                self.state.push_token(Token::Unknown(String::from(current_char)));
                            }
                        }
                    },
                    Token::CaptureIndicator => {
                        match current_char {
                            'a'..='h' | '1'..='8' => {
                                self.state.push_token(Token::MovingTo(String::from(current_char)))
                            },
                            _ => {
                                self.state.push_token(Token::Unknown(String::from(current_char)));
                            }
                        }
                    }
                    Token::MovingTo(str) => {
                        match current_char {
                            'a'..='h' | '1'..='8' => {
                                if str.len() <= 2 {
                                    self.state.push_token(Token::MovingTo(String::from(current_char)))
                                } else {
                                    self.state.push_token(Token::Unknown(String::from(current_char)));
                                }
                            },
                            'K' | 'Q' | 'B' | 'N' | 'R' => {
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
                                self.state.push_token(Token::WhiteSpace(WhiteSpaceToken::AfterMovingTo))
                            }
                            _ => {
                                self.state.push_token(Token::Unknown(String::from(current_char)));
                            }
                        }
                    },
                    Token::PromotionStart(char) => {
                        match current_char {
                            'K' | 'Q' | 'B' | 'N' | 'R' => {
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
                                self.state.push_token(Token::WhiteSpace(WhiteSpaceToken::AfterPromotion));
                            }
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
                                self.state.push_token(Token::WhiteSpace(WhiteSpaceToken::AfterPromotionEnd));
                            }
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
                                self.state.push_token(Token::WhiteSpace(WhiteSpaceToken::AfterMoveQuality));
                            }
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
                                    self.state.push_token(Token::WhiteSpace(WhiteSpaceToken::AfterNag));
                                }
                                _ => {
                                    self.state.push_token(Token::Unknown(String::from(current_char)));
                                }
                            }
                        }
                    },
                    Token::CheckIndicator(char) => {
                        match current_char {
                            char::SPACE => {
                                self.state.push_token(Token::WhiteSpace(WhiteSpaceToken::AfterCheckIndicator));
                            }
                            _ => {
                                self.state.push_token(Token::Unknown(String::from(current_char)));
                            }
                        }
                    },
                    Token::CheckMateIndicator(char) => {
                        match current_char {
                            char::SPACE => {
                                self.state.push_token(Token::WhiteSpace(WhiteSpaceToken::AfterCheckMateIndicator));
                            }
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
                                self.state.push_token(Token::NewLine);
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
                                self.state.push_token(Token::NewLine);
                            }
                            char::SPACE => {
                                self.state.push_token(Token::WhiteSpace(WhiteSpaceToken::AfterAnnotationEnd));
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
                                char::SPACE => self.state.push_token(Token::WhiteSpace(WhiteSpaceToken::AfterTurnContinuation)),
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
                            char::NEW_LINE => self.state.push_token(Token::NewLine),
                            _ => str.push(current_char),
                        }
                    },
                    Token::NewLine => {
                        self.handle_char_after_newline(&current_char);
                    },
                    Token::WhiteSpace(white_space_token) => {
                        match white_space_token {
                            WhiteSpaceToken::AfterNewLine => self.handle_char_after_newline(&current_char),
                            WhiteSpaceToken::AfterTagPairName => {
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
                            WhiteSpaceToken::AfterTagPairEnd => {
                                match current_char {
                                    char::SPACE => {
                                        /* skip */
                                        trace!("skipping ' '");
                                    },
                                    char::NEW_LINE => self.state.push_token(Token::NewLine),
                                    _ => self.state.push_token(Token::Unknown(String::from(current_char))),
                                }
                            }
                            WhiteSpaceToken::AfterTurnBegin => {
                                match current_char {
                                    char::SPACE => {
                                        /* skip */
                                        trace!("skipping ' '");
                                    },
                                    'K' | 'Q' | 'B' | 'N' | 'R' => {
                                        self.state.push_token(Token::PieceMoving(current_char));
                                    },
                                    'a'..='h' | '1'..='8' => {
                                        self.state.push_token(Token::MovingFrom(current_char))
                                    },
                                    _ => self.state.push_token(Token::Unknown(String::from(current_char)))
                                }
                            }
                            WhiteSpaceToken::AfterMovingTo => {
                                self.handle_char_after_move(&current_char);
                            }
                            WhiteSpaceToken::AfterPromotion => {
                                self.handle_char_after_move(&current_char);
                            }
                            WhiteSpaceToken::AfterPromotionEnd => {
                                self.handle_char_after_move(&current_char);
                            }
                            WhiteSpaceToken::AfterCheckIndicator => {
                                self.handle_char_after_move(&current_char);
                            }
                            WhiteSpaceToken::AfterCheckMateIndicator => {
                                self.handle_char_after_move(&current_char);
                            }
                            WhiteSpaceToken::AfterAnnotationEnd => {
                                if current_char.is_ascii_digit() {
                                    self.state.push_token(Token::MaybeTurnBeginOrContinuationOrMovingFromOrGameTermination(String::from(current_char)));
                                } else {
                                    match current_char {
                                        char::SPACE => {
                                            /* skip */
                                            trace!("skipping ' '");
                                        },
                                        char::NEW_LINE => {
                                            self.state.push_token(Token::NewLine);
                                        }
                                        'K' | 'Q' | 'B' | 'N' | 'R' => {
                                            self.state.push_token(Token::PieceMoving(current_char));
                                        },
                                        'a'..='h' => {
                                            self.state.push_token(Token::MovingFrom(current_char))
                                        },
                                        '*' => {
                                            self.state.push_token(Token::GameTermination(String::from(current_char)));
                                        }
                                        '0' | '1' | '2' => {
                                            self.state.push_token(Token::MaybeTurnBeginOrContinuationOrMovingFromOrGameTermination(String::from(current_char)));
                                        },
                                        '1'..='8' => {
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
                            WhiteSpaceToken::AfterMoveQuality => {
                                self.handle_char_after_nag(&current_char);
                            }
                            WhiteSpaceToken::AfterNag => {
                                self.handle_char_after_nag(&current_char);
                            }
                            WhiteSpaceToken::AfterTurnContinuation => {
                                match current_char {
                                    char::SPACE => {
                                        /* skip */
                                        trace!("skipping ' '");
                                    },
                                    'a'..='h' | '1'..='8' => {
                                        self.state.push_token(Token::MovingFrom(current_char))
                                    },
                                    _ => {
                                        self.state.push_token(Token::Unknown(String::from(current_char)));
                                    }
                                }
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
                                        *token = Token::TurnContinuation(format!("{str}{current_char}"));
                                        self.state.push_token(Token::WhiteSpace(WhiteSpaceToken::AfterTurnContinuation));
                                    } else if first_char_is_digit && str.ends_with('.') {
                                        trace!("replacing with TurnBegin");
                                        *token = Token::TurnBegin(format!("{str}{current_char}"));
                                        self.state.push_token(Token::WhiteSpace(WhiteSpaceToken::AfterTurnBegin));
                                    } else {
                                        trace!("replacing with Unknown");
                                        *token = Token::Unknown(format!("{str}{current_char}"));
                                    }
                                },
                                'a'..='h' | '1'..='8' => {
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
                                    self.state.push_token(Token::NewLine);
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
    use std::hint::black_box;
    use itertools::Itertools;
    use super::*;
    use rstest::rstest;
    use crate::utils::tracing::init_tracing;
    use token::Token::*;
    use token::WhiteSpaceToken::*;

    #[rstest]
    #[case(
        "[Event \"Some Event\"]\n\
        \n\
        1. e4 d5 *",
        vec![
            TagPairStart('['),
            TagPairName("Event".into()),
            WhiteSpace(AfterTagPairName),
            TagPairValue("\"Some Event\"".into()),
            TagPairEnd(']'),
            NewLine,
            NewLine,
            TurnBegin("1.".into()),
            WhiteSpace(AfterTurnBegin),
            MovingFrom('e'),
            MovingTo("4".into()),
            WhiteSpace(AfterMovingTo),
            MovingFrom('d'),
            MovingTo("5".into()),
            WhiteSpace(AfterMovingTo),
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
            TagPairName("Event".into()),
            WhiteSpace(AfterTagPairName),
            TagPairValue("\"Some Event\"".into()),
            TagPairEnd(']'),
            NewLine,
            NewLine,
            TurnBegin("1.".into()),
            WhiteSpace(AfterTurnBegin),
            MovingFrom('e'),
            MovingTo("4".into()),
            WhiteSpace(AfterMovingTo),
            AnnotationStart('{'),
            Annotation("white comment".into()),
            AnnotationEnd('}'),
            WhiteSpace(AfterAnnotationEnd),
            TurnContinuation("1... ".into()),
            WhiteSpace(AfterTurnContinuation),
            MovingFrom('d'),
            MovingTo("5".into()),
            WhiteSpace(AfterMovingTo),
            AnnotationStart('{'),
            Annotation("black comment".into()),
            AnnotationEnd('}'),
            WhiteSpace(AfterAnnotationEnd),
            TurnBegin("2. ".into()),
            WhiteSpace(AfterTurnBegin),
            MovingFrom('e'),
            MovingTo("4".into()),
            WhiteSpace(AfterMovingTo),
            MoveQuality("!!".into()),
            WhiteSpace(AfterMoveQuality),
            MovingFrom('d'),
            MovingTo("5".into()),
            WhiteSpace(AfterMovingTo),
            MoveQuality("!?".into()),
            WhiteSpace(AfterMoveQuality),
            Annotation("; end of line comment".into()),
            NewLine,
            TurnBegin("3.".into()),
            WhiteSpace(AfterTurnBegin),
            PieceMoving('Q'),
            MovingFrom('h'),
            MovingTo("8".into()),
            CheckMateIndicator('#'),
            WhiteSpace(AfterCheckMateIndicator),
            Nag("$1".into()),
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

