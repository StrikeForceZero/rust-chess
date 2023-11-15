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

pub struct Lexer<'a> {
    state: LexerState<'a>,
}

impl<'a> Lexer<'a> {
    fn init(data: &'a str) -> Self {
        Self {
            state: LexerState::new(data),
        }
    }
    fn handle_char_after_newline(&mut self, &current_char: &char) {
        if current_char.is_ascii_digit() {
            self.state.push_token(Token::TurnBegin(String::from(current_char)))
        } else {
            match current_char {
                '[' => self.state.push_token(Token::TagPairStart(current_char)),
                char::NEW_LINE => self.state.push_token(Token::NewLine),
                char::SPACE => self.state.push_token(Token::WhiteSpace(WhiteSpaceToken::AfterNewLine)),
                _ => self.state.push_token(Token::Unknown(String::from(current_char))),
            };
        }
    }
    pub fn handle_char(&mut self, &current_char: &char) {
        match self.state.tokens.last_mut() {
            None => {
                self.handle_char_after_newline(&current_char);
            }
            Some(ref mut token_with_context) => {
                let TokenWithContext(token, _) = token_with_context;
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
                            'a'..='f' | '1'..='8' => {
                                self.state.push_token(Token::MovingFrom(current_char))
                            },
                            _ => {
                                self.state.push_token(Token::Unknown(String::from(current_char)));
                            },
                        }
                    },
                    Token::MovingFrom(char) => {
                        match current_char {
                            'a'..='f' | '1'..='8' => {
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
                            'a'..='f' | '1'..='8' => {
                                self.state.push_token(Token::MovingTo(String::from(current_char)))
                            },
                            _ => {
                                self.state.push_token(Token::Unknown(String::from(current_char)));
                            }
                        }
                    }
                    Token::MovingTo(str) => {
                        match current_char {
                            'a'..='f' | '1'..='8' => {
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
                            '0' | '1' | '/' | '-' | '*' => {
                                str.push(current_char)
                            },
                            char::SPACE => {
                                self.state.push_token(Token::WhiteSpace(WhiteSpaceToken::AfterGameTermination))
                            },
                            char::NEW_LINE => {
                                self.state.push_token(Token::NewLine)
                            },
                            _ => {
                                self.state.push_token(Token::Unknown(String::from(current_char)));
                            }
                        }
                        self.state.push_token(Token::Unknown(String::from(current_char)))
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
                                    char::SPACE => {/* skip */}
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
                                    char::NEW_LINE => self.state.push_token(Token::NewLine),
                                    _ => self.state.push_token(Token::Unknown(String::from(current_char))),
                                }
                            }
                            WhiteSpaceToken::AfterTurnBegin => {
                                match current_char {
                                    'K' | 'Q' | 'B' | 'N' | 'R' => {
                                        self.state.push_token(Token::PieceMoving(current_char));
                                    },
                                    'a'..='f' | '1'..='8' => {
                                        self.state.push_token(Token::MovingFrom(current_char))
                                    },
                                    _ => self.state.push_token(Token::Unknown(String::from(current_char)))
                                }
                            }
                            WhiteSpaceToken::AfterMovingTo => {
                                match current_char {
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
                                    _ => self.state.push_token(Token::Unknown(String::from(current_char))),
                                }
                            }
                            WhiteSpaceToken::AfterPromotion => {}
                            WhiteSpaceToken::AfterPromotionEnd => {}
                            WhiteSpaceToken::AfterCheckIndicator => {}
                            WhiteSpaceToken::AfterCheckMateIndicator => {}
                            WhiteSpaceToken::AfterAnnotationEnd => {}
                            WhiteSpaceToken::AfterMoveQuality => {}
                            WhiteSpaceToken::AfterNag => {}
                            WhiteSpaceToken::AfterTurnContinuation => {}
                            WhiteSpaceToken::AfterGameTermination => {}
                        }
                    }
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

