use std::collections::VecDeque;
use tracing::{error, warn};
use crate::notation::pgn::lexer::{Lexer, };
use crate::notation::pgn::lexer::token::{Token, WhiteSpaceToken};
use crate::notation::pgn::lexer::token_with_context::TokenWithContext;
use crate::notation::pgn::pgn_data_partial::PgnDataPartial;
use crate::notation::pgn::pgn_move_builder::PgnMoveBuilder;
use crate::notation::pgn::pgn_move_detail_builder::PgnMoveDetailBuilder;
use crate::notation::pgn::pgn_parsing_error::PgnParsingError;
use crate::notation::pgn::pgn_roster_partial::PgnRosterPartial;
use crate::notation::pgn::pgn_roster_raw_partial::PgnRosterRawPartial;
use crate::notation::pgn::pgn_turn_builder::PgnTurnBuilder;
use crate::notation::pgn::pgn_turn_data::PgnTurnData;
use crate::notation::pgn::tag_pairs::{parse_tag_pair, PgnTagPairParseError, resolve_tag_pair, TagPair, TagPairNameValueTuple};
use crate::utils::char::{NEW_LINE, SPACE};

#[derive(Default, Debug)]
pub struct ParserState {
    token_stack: VecDeque<Token>,
    roster: Option<PgnRosterPartial>,
    current_turn: Option<PgnTurnData>,
    turns: Vec<PgnTurnData>,
}

#[derive(Default, Debug)]
pub struct Parser {
    state: ParserState,
}

macro_rules! for_stack_push_expect_prev_token_to_be {
    ($self:expr, $token_with_context:expr, $token:pat) => {
        let TokenWithContext(token, ref token_context) = $token_with_context;
        match $self.state.token_stack.back() {
            Some($token) => {
                $self.state.token_stack.push_back(token);
            },
            _ => return Err(token_context.create_error())
        };
    };
}

macro_rules! for_stack_push_expect_prev_token_to_be_none_or {
    ($self:expr, $token_with_context:expr, $token:pat) => {
        let TokenWithContext(token, token_context) = $token_with_context;
        match $self.state.token_stack.back() {
            None | Some($token) => {
                $self.state.token_stack.push_back(token);
            },
            _ => return Err(token_context.create_error())
        };
    };
}

impl Parser {
    fn init() -> Self {
        Self {
            ..Default::default()
        }
    }
    fn build_string_from_stack(&self) -> String {
        let mut res = String::new();
        for token in &self.state.token_stack {
            match token {
                Token::TagPairStart(data) => { res.push(*data) }
                Token::TagPairName(data) => { res.push_str(data) }
                Token::TagPairValue(data) => { res.push_str(data) }
                Token::TagPairEnd(data) => { res.push(*data) }
                Token::TurnBegin(data) => { res.push_str(data) }
                Token::PieceMoving(data) => { res.push(*data) }
                Token::MovingFrom(data) => { res.push(*data) }
                Token::CaptureIndicator => { res.push('x') }
                Token::MovingTo(data) => { res.push_str(data) }
                Token::PromotionStart(data) => { res.push(*data) }
                Token::Promotion(data) => { res.push(*data) }
                Token::PromotionEnd(data) => { res.push(*data) }
                Token::CheckIndicator(data) => { res.push(*data) }
                Token::CheckMateIndicator(data) => { res.push(*data) }
                Token::AnnotationStart(data) => { res.push(*data) }
                Token::Annotation(data) => { res.push_str(data) }
                Token::AnnotationEnd(data) => { res.push(*data) }
                Token::MoveQuality(data) => { res.push_str(data) }
                Token::Nag(data) => { res.push_str(data) }
                Token::TurnContinuation(data) => { res.push_str(data) }
                Token::GameTermination(data) => { res.push_str(data) }
                Token::NewLine(_) => { res.push(NEW_LINE) }
                Token::WhiteSpace(_) => { res.push(SPACE) }

                _ => {
                    panic!("trying to build string of bad data")
                }
            }
        }
        res
    }
    pub fn parse(data: &str) -> Result<PgnDataPartial, PgnParsingError> {
        let mut parser = Self::init();
        let mut pgn_data = PgnDataPartial::default();

        for token_with_context in Lexer::lex(data) {
            parser.handle_token_with_context(token_with_context)?;
        }

        if let Some(ref mut raw_roster) = parser.state.roster.take() {
            pgn_data.roster = Some(raw_roster.build()?);
        }
        if !parser.state.turns.is_empty() {
            pgn_data.turns = Some(parser.state.turns);
        }
        Ok(pgn_data)
    }
    fn handle_token_with_context(&mut self, token_with_context: TokenWithContext) -> Result<(), PgnParsingError> {
        let TokenWithContext(ref token, ref token_context) = token_with_context;
        match token {
            Token::TagPairStart(_) => {
                for_stack_push_expect_prev_token_to_be_none_or!(self, token_with_context, Token::NewLine(_));
            }
            Token::TagPairName(_) => {
                for_stack_push_expect_prev_token_to_be!(self, token_with_context, Token::TagPairStart(_));
            }
            Token::TagPairValue(_) => {
                for_stack_push_expect_prev_token_to_be!(self, token_with_context, Token::WhiteSpace(WhiteSpaceToken::AfterTagPairName));
            }
            Token::TagPairEnd(_) => {
                for_stack_push_expect_prev_token_to_be!(self, token_with_context, Token::TagPairValue(_));
                let tag_pair_tuple = match parse_tag_pair(&self.build_string_from_stack()) {
                    Ok(res) => res,
                    Err(err) => {
                        error!("error parsing tag pair - inner error: {err:?}");
                        return Err(token_context.create_error());
                    }
                };
                let tag_pair = match resolve_tag_pair(tag_pair_tuple) {
                    Ok(res) => res,
                    Err(err) => {
                        error!("error resolving tag pair - inner error: {err:?}");
                        return Err(token_context.create_error());
                    }
                };
                match tag_pair {
                    TagPair::Event(data) => self.state.roster.get_or_insert(PgnRosterPartial::default()).event = Some(data),
                    TagPair::Site(data) => self.state.roster.get_or_insert(PgnRosterPartial::default()).site = Some(data),
                    TagPair::Date(data) => self.state.roster.get_or_insert(PgnRosterPartial::default()).date = Some(data),
                    TagPair::Round(data) => self.state.roster.get_or_insert(PgnRosterPartial::default()).round = Some(data),
                    TagPair::White(data) => self.state.roster.get_or_insert(PgnRosterPartial::default()).white = Some(data),
                    TagPair::Black(data) => self.state.roster.get_or_insert(PgnRosterPartial::default()).black = Some(data),
                    TagPair::Result(data) => self.state.roster.get_or_insert(PgnRosterPartial::default()).result = Some(data),
                    TagPair::Fen(data) => self.state.roster.get_or_insert(PgnRosterPartial::default()).fen = Some(data),
                }
                self.state.token_stack.clear();
            }
            Token::TurnBegin(_) => {
                match self.state.token_stack.back() {
                    None
                    | Some(Token::NewLine(_))
                    | Some(Token::WhiteSpace(WhiteSpaceToken::AfterMovingTo))
                    | Some(Token::WhiteSpace(WhiteSpaceToken::AfterPromotion))
                    | Some(Token::WhiteSpace(WhiteSpaceToken::AfterPromotionEnd))
                    | Some(Token::WhiteSpace(WhiteSpaceToken::AfterCheckIndicator))
                    | Some(Token::WhiteSpace(WhiteSpaceToken::AfterCheckMateIndicator))
                    | Some(Token::WhiteSpace(WhiteSpaceToken::AfterAnnotation))
                    | Some(Token::MovingTo(_))
                    | Some(Token::Promotion(_))
                    | Some(Token::PromotionEnd(_))
                    | Some(Token::CheckIndicator(_))
                    | Some(Token::CheckMateIndicator(_))
                    | Some(Token::Annotation(_))
                    | Some(Token::AnnotationEnd(_)) => {
                        let old_entry = self.build_string_from_stack();
                        if !old_entry.trim().is_empty() {
                            let mut turn_builder: Option<PgnTurnBuilder> = None;
                            for token in self.state.token_stack.iter() {
                                match token {
                                    Token::TurnBegin(data) => {
                                        let number_str = &data[0..data.len() - 1];
                                        let Ok(number) = number_str.parse::<usize>() else {
                                            return Err(token_context.create_error());
                                        };
                                        let mut last_number = if let Some(last_turn) = self.state.turns.last() {
                                            last_turn.turn_number
                                        } else {
                                            0
                                        };
                                        let expected_number = last_number + 1;
                                        let is_valid = number.is_ok() && data.ends_with('.') && expected_number == number;
                                        turn_builder.get_or_insert(PgnTurnBuilder::new())
                                    }
                                    Token::PieceMoving(data) => {}
                                    Token::MovingFrom(data) => {}
                                    Token::CaptureIndicator => {}
                                    Token::MovingTo(data) => {}
                                    Token::PromotionStart(data) => {}
                                    Token::Promotion(data) => {}
                                    Token::PromotionEnd(data) => {}
                                    Token::CheckIndicator(data) => {}
                                    Token::CheckMateIndicator(data) => {}
                                    Token::AnnotationStart(data) => {}
                                    Token::Annotation(data) => {}
                                    Token::AnnotationEnd(data) => {}
                                    Token::MoveQuality(data) => {}
                                    Token::Nag(data) => {}
                                    Token::TurnContinuation(data) => {}
                                    _ => {}
                                }
                            }
                        }
                        self.state.token_stack.clear();
                        let TokenWithContext(token, _) = token_with_context;
                        self.state.token_stack.push_back(token);
                    },
                    _ => return Err(token_context.create_error())
                };
            }
            Token::PieceMoving(_) => {}
            Token::MovingFrom(_) => {}
            Token::CaptureIndicator => {}
            Token::MovingTo(_) => {}
            Token::PromotionStart(_) => {}
            Token::Promotion(_) => {}
            Token::PromotionEnd(_) => {}
            Token::CheckIndicator(_) => {}
            Token::CheckMateIndicator(_) => {}
            Token::AnnotationStart(_) => {}
            Token::Annotation(_) => {}
            Token::AnnotationEnd(_) => {}
            Token::MoveQuality(_) => {}
            Token::Nag(_) => {}
            Token::TurnContinuation(_) => {}
            Token::GameTermination(_) => {}
            Token::Unknown(_) => {
                return Err(token_context.create_error());
            }
            Token::NewLine(after) => {
                match after {
                    WhiteSpaceToken::AfterNewLine => {}
                    WhiteSpaceToken::AfterTagPairName => {}
                    WhiteSpaceToken::AfterTagPairEnd => {}
                    WhiteSpaceToken::AfterTurnBegin => {}
                    WhiteSpaceToken::AfterMovingTo => {}
                    WhiteSpaceToken::AfterPromotion => {}
                    WhiteSpaceToken::AfterPromotionEnd => {}
                    WhiteSpaceToken::AfterCheckIndicator => {}
                    WhiteSpaceToken::AfterCheckMateIndicator => {}
                    WhiteSpaceToken::AfterAnnotation => {}
                    WhiteSpaceToken::AfterAnnotationEnd => {}
                    WhiteSpaceToken::AfterMoveQuality => {}
                    WhiteSpaceToken::AfterNag => {}
                    WhiteSpaceToken::AfterTurnContinuation => {}
                    WhiteSpaceToken::AfterUnknown => {}
                }
            }
            Token::WhiteSpace(after) => {
                match after {
                    WhiteSpaceToken::AfterNewLine => {}
                    WhiteSpaceToken::AfterTagPairName => {}
                    WhiteSpaceToken::AfterTagPairEnd => {}
                    WhiteSpaceToken::AfterTurnBegin => {}
                    WhiteSpaceToken::AfterMovingTo => {}
                    WhiteSpaceToken::AfterPromotion => {}
                    WhiteSpaceToken::AfterPromotionEnd => {}
                    WhiteSpaceToken::AfterCheckIndicator => {}
                    WhiteSpaceToken::AfterCheckMateIndicator => {}
                    WhiteSpaceToken::AfterAnnotation => {}
                    WhiteSpaceToken::AfterAnnotationEnd => {}
                    WhiteSpaceToken::AfterMoveQuality => {}
                    WhiteSpaceToken::AfterNag => {}
                    WhiteSpaceToken::AfterTurnContinuation => {}
                    WhiteSpaceToken::AfterUnknown => {}
                }
            }
            Token::MaybeTurnBeginOrContinuationOrMovingFromOrGameTermination(_) => {
                return Err(token_context.create_error());
            }
        }
        Ok(())
    }
}

//  FIDE Examples: d8Q, f8N, b1B, g1R
//  SAN Examples: d8=Q, f8=N, b1=B, g1=R
//  Alt Examples: d8(Q), f8/N
#[derive(PartialEq, Debug)]
pub enum PromotionFormat {
    FIDE,
    SAN,
    Parenthesis,
    ForwardSlash,
}
#[derive(PartialEq, Debug)]
pub enum AnnotationFormat {
    Parenthesis,
    CurlyBracket,
    SemiColonEOL,
}
