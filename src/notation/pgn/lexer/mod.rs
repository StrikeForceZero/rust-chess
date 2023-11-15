use token::Token;
use token_context::TokenContext;
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
                char::SPACE | char::NEW_LINE => { /* skip */ },
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
                            char::SPACE => self.state.push_token(Token::TagPairValue(None)),
                            ']' => self.state.push_token(Token::TagPairEnd(current_char)),
                            _ => str.push(current_char),
                        }
                    },
                    Token::TagPairValue(str_option) => {
                        match current_char {
                            ']' => self.state.push_token(Token::TagPairEnd(current_char)),
                            _ => str_option.get_or_insert(String::new()).push(current_char),
                        }
                    },
                    Token::TagPairEnd(_) => {
                        match current_char {
                            char::NEW_LINE => self.state.push_token(Token::NewLine),
                            char::SPACE => { /* skip */},
                            _ => self.state.push_token(Token::Unknown(String::from(current_char))),
                        }
                    },
                    Token::TurnBegin(str) => {
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
                                ' ' => self.state.push_token(Token::PieceMoving(None)),
                                _ => self.state.push_token(Token::Unknown(String::from(current_char)))
                            }
                        }
                    },
                    Token::PieceMoving(char_option) => {
                        match char_option {
                            None => {
                                match current_char {
                                    'K' | 'Q' | 'B' | 'N' | 'R' => {
                                        let TokenWithContext(_, context) = token_with_context;
                                        *context = self.state.context.clone();
                                        char_option.replace(current_char);
                                    },
                                    'a'..='f' | '1'..='8' => {
                                        self.state.tokens.pop();
                                        self.state.push_token(Token::MovingFrom(Some(current_char)))
                                    },
                                    _ => {
                                        self.state.tokens.pop();
                                        self.state.push_token(Token::Unknown(String::from(current_char)));
                                    },
                                };
                            },
                            Some(char) => {
                                match current_char {
                                    'a'..='f' | '1'..='8' => {
                                        self.state.push_token(Token::MovingFrom(Some(current_char)))
                                    },
                                    _ => {
                                        self.state.push_token(Token::Unknown(String::from(current_char)));
                                    },
                                }
                            }
                        }
                    },
                    Token::MovingFrom(char_option) => {
                        match char_option {
                            None => {
                                match current_char {
                                    'a'..='f' | '1'..='8' => {
                                        char_option.replace(current_char);
                                    },
                                    _ => {
                                        self.state.tokens.pop();
                                        self.state.push_token(Token::Unknown(String::from(current_char)));
                                    },
                                }
                            }
                            Some(_) => {
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
                            ' ' => {

                            }
                            _ => {
                                self.state.push_token(Token::Unknown(String::from(current_char)));
                            }
                        }
                    },
                    Token::MoveQuality(str) => {},
                    Token::Nag(str) => {},
                    Token::PromotionEnd(char) => {},
                    Token::CheckIndicator(char) => {},
                    Token::CheckMateIndicator(char) => {},
                    Token::AnnotationStart(char) => {},
                    Token::Annotation(str) => {},
                    Token::AnnotationEnd(char) => {},
                    Token::TurnContinuation(str) => {},
                    Token::GameTermination(str) => {
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

