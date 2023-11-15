use std::collections::VecDeque;
use tracing::{error, warn};
use crate::notation::pgn::lexer::{Lexer, };
use crate::notation::pgn::lexer::token::{Token, WhiteSpaceToken};
use crate::notation::pgn::lexer::token_with_context::TokenWithContext;
use crate::notation::pgn::pgn_data_partial::PgnDataPartial;
use crate::notation::pgn::pgn_parsing_error::PgnParsingError;
use crate::notation::pgn::pgn_roster_raw_partial::PgnRosterRawPartial;
use crate::notation::pgn::pgn_turn_data::PgnTurnData;
use crate::notation::pgn::tag_pairs::{parse_tag_pair, PgnTagPairParseError, resolve_tag_pair, TagPair, TagPairNameValueTuple};
use crate::utils::char::{NEW_LINE, SPACE};

#[derive(Default, Debug)]
pub struct ParserState {
    token_stack: VecDeque<Token>,
    roster: Option<PgnRosterRawPartial>,
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
        }
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
        }
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
    fn unwrap_result_or_bubble_context_error<T>(token_with_context: &TokenWithContext, res: Result<T, PgnParsingError>) -> Result<T, PgnParsingError> {
        Ok(match res {
            Ok(value) => value,
            Err(err) => {
                let TokenWithContext(ref token, ref token_context) = token_with_context;
                error!("inner error: {err:?}");
                return Err(token_context.create_error());
            }
        })
    }
    pub fn parse(data: &str) -> Result<PgnDataPartial, PgnParsingError> {
        let mut parser = Self::init();
        let mut pgn_data = PgnDataPartial::default();

        for token_with_context in Lexer::lex(data) {
            parser.handle_token_with_context(token_with_context)?;
        }

        if let Some(ref mut raw_roster) = parser.state.roster.take() {
            pgn_data.roster = Some(raw_roster.build()?.build()?);
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
                    TagPair::Event(data) => self.state.roster.get_or_insert(PgnRosterRawPartial::default()).event = Some(data),
                    TagPair::Site(data) => self.state.roster.get_or_insert(PgnRosterRawPartial::default()).site = Some(data),
                    TagPair::Date(data) => self.state.roster.get_or_insert(PgnRosterRawPartial::default()).date = Some(data),
                    TagPair::Round(data) => self.state.roster.get_or_insert(PgnRosterRawPartial::default()).round = Some(data),
                    TagPair::White(data) => self.state.roster.get_or_insert(PgnRosterRawPartial::default()).white = Some(data),
                    TagPair::Black(data) => self.state.roster.get_or_insert(PgnRosterRawPartial::default()).black = Some(data),
                    TagPair::Result(data) => self.state.roster.get_or_insert(PgnRosterRawPartial::default()).result = Some(data),
                    TagPair::Fen(data) => self.state.roster.get_or_insert(PgnRosterRawPartial::default()).fen = Some(data),
                }
                self.state.token_stack.clear();
            }
            Token::TurnBegin(_) => {}
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

