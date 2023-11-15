use std::collections::VecDeque;
use tracing::{error, warn};
use crate::board::board_file::{BoardFile, BoardFileError};
use crate::board::board_position::BoardPosition;
use crate::board::board_rank::BoardRank;
use crate::char_match;
use crate::color::Color;
use crate::notation::pgn::lexer::{Lexer, };
use crate::notation::pgn::lexer::token::{Token, WhiteSpaceToken};
use crate::notation::pgn::lexer::token_context::TokenContext;
use crate::notation::pgn::lexer::token_with_context::TokenWithContext;
use crate::notation::pgn::pgn_data_partial::PgnDataPartial;
use crate::notation::pgn::pgn_move_builder::PgnMoveBuilder;
use crate::notation::pgn::pgn_move_detail::PgnMoveDetail;
use crate::notation::pgn::pgn_move_detail_builder::PgnMoveDetailBuilder;
use crate::notation::pgn::pgn_parsing_error::PgnParsingError;
use crate::notation::pgn::pgn_roster_partial::PgnRosterPartial;
use crate::notation::pgn::pgn_roster_raw_partial::PgnRosterRawPartial;
use crate::notation::pgn::pgn_turn_builder::PgnTurnBuilder;
use crate::notation::pgn::pgn_turn_data::PgnTurnData;
use crate::notation::pgn::tag_pairs::{parse_tag_pair, PgnTagPairParseError, resolve_tag_pair, TagPair, TagPairNameValueTuple};
use crate::piece::piece::Piece;
use crate::utils::char::{NEW_LINE, SPACE};

#[derive(Default, Debug)]
pub struct ParserState<'a> {
    token_stack: VecDeque<&'a Token>,
    roster: Option<PgnRosterPartial>,
    current_turn: Option<PgnTurnBuilder>,
    turns: Vec<PgnTurnData>,
}

#[derive(Default, Debug)]
pub struct Parser<'a> {
    state: ParserState<'a>,
}

macro_rules! for_stack_push_expect_prev_token_to_be {
    ($self:expr, $token_with_context:expr, $token:pat) => {
        let TokenWithContext(token, token_context) = $token_with_context;
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

fn get_move_detail_for_next<F, M>(
    turn_builder: &mut Option<PgnTurnBuilder>,
    token_context: &TokenContext,
    prop_check_fn: F,
    mut prop_update_fn: M,
) -> Result<(), PgnParsingError>
    where F: Fn(&PgnMoveDetailBuilder) -> bool,
          M: FnMut(&mut PgnMoveDetailBuilder) -> Result<(), PgnParsingError>
{
    let Some(ref mut turn_builder) = *turn_builder else {
        return Err(token_context.create_error())
    };
    let mut color_to_update: Option<Color> = None;
    if let Some(white) = &turn_builder.white {
        if prop_check_fn(white.get_move_detail()) {
            // if white defined, lets check black
            if let Some(black) = &turn_builder.black {
                if prop_check_fn(black.get_move_detail()) {
                    return Err(token_context.create_error());
                }
                color_to_update = Some(Color::Black);
            } else {
                color_to_update = Some(Color::Black);
            }
        } else {
            color_to_update = Some(Color::White);
        }
    } else {
        color_to_update = Some(Color::White);
    };
    let color = color_to_update.unwrap();
    prop_update_fn(turn_builder.get_or_insert(color).get_move_detail_mut())?;
    Ok(())
}

impl<'a> Parser<'a> {
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

        let tokens = Lexer::lex(data);

        for token_with_context in tokens.iter() {
            parser.handle_token_with_context(&token_with_context)?;
        }

        if let Some(ref mut raw_roster) = parser.state.roster.take() {
            pgn_data.roster = Some(raw_roster.build()?);
        }
        if !parser.state.turns.is_empty() {
            pgn_data.turns = Some(parser.state.turns);
        }
        Ok(pgn_data)
    }
    fn handle_token_with_context(&mut self, token_with_context: &'a TokenWithContext) -> Result<(), PgnParsingError> {
        let TokenWithContext(ref cur_token, ref cur_token_context) = &token_with_context;
        match cur_token {
            Token::TagPairStart(_) => {
                for_stack_push_expect_prev_token_to_be_none_or!(self, &token_with_context, Token::NewLine(_));
            }
            Token::TagPairName(_) => {
                for_stack_push_expect_prev_token_to_be!(self, &token_with_context, Token::TagPairStart(_));
            }
            Token::TagPairValue(_) => {
                for_stack_push_expect_prev_token_to_be!(self, &token_with_context, Token::WhiteSpace(WhiteSpaceToken::AfterTagPairName));
            }
            Token::TagPairEnd(_) => {
                for_stack_push_expect_prev_token_to_be!(self, &token_with_context, Token::TagPairValue(_));
                let tag_pair_tuple = match parse_tag_pair(&self.build_string_from_stack()) {
                    Ok(res) => res,
                    Err(err) => {
                        error!("error parsing tag pair - inner error: {err:?}");
                        return Err(cur_token_context.create_error());
                    }
                };
                let tag_pair = match resolve_tag_pair(tag_pair_tuple) {
                    Ok(res) => res,
                    Err(err) => {
                        error!("error resolving tag pair - inner error: {err:?}");
                        return Err(cur_token_context.create_error());
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
            Token::TurnBegin(data) => {
                let number_str = &data[0..data.len() - 1];
                let Ok(number) = number_str.parse::<usize>() else {
                    return Err(cur_token_context.create_error());
                };
                let mut last_number = if let Some(last_turn) = self.state.turns.last() {
                    last_turn.turn_number
                } else {
                    0
                };
                let expected_number = last_number + 1;
                let is_valid = data.ends_with('.') && expected_number == number;
                if !is_valid {
                    return Err(cur_token_context.create_error());
                }
                if let Some(ref mut turn_builder) = self.state.current_turn.replace(PgnTurnBuilder::new(number)) {
                    let turn = turn_builder.build();
                    match turn {
                        Ok(turn) => self.state.turns.push(turn),
                        Err(err) => {
                            error!("error parsing turn - inner error: {err:?}");
                            return Err(cur_token_context.create_error());
                        }
                    }
                    return Err(cur_token_context.create_error());
                }
            }
            Token::PieceMoving(data) => {
                match data {
                    char_match!(from_piece) => {
                        let Ok(piece) = Piece::from_char(*data) else {
                            return Err(cur_token_context.create_error());
                        };
                        let Some(ref mut turn_builder) = self.state.current_turn else {
                            return Err(cur_token_context.create_error())
                        };
                        let mut color_to_update: Option<Color> = None;
                        if let Some(white) = &turn_builder.white {
                            // if white defined, lets check black
                            if let Some(black) = &turn_builder.black {
                                // shouldn't have white and black defined by yet, so probably more than 2 moves in a turn
                                return Err(cur_token_context.create_error());
                            } else {
                                color_to_update = Some(Color::Black);
                            };
                        } else {
                            color_to_update = Some(Color::White);
                        };
                        let color = color_to_update.unwrap();
                        let move_detail = turn_builder.get_or_insert(color).get_move_detail_mut();
                        move_detail.chess_piece = Some(piece.as_chess_piece(color));
                    }
                    _ => return Err(cur_token_context.create_error())
                }
            }
            Token::MovingFrom(data) => {
                let Some(ref mut turn_builder) = self.state.current_turn else {
                    return Err(cur_token_context.create_error())
                };
                let mut color_to_update: Option<Color> = None;
                if let Some(white) = &turn_builder.white {
                    if white.get_move_detail().has_from() {
                        // if white defined, lets check black
                        if let Some(black) = &turn_builder.black {
                            if black.get_move_detail().has_from() {
                                return Err(cur_token_context.create_error());
                            }
                            color_to_update = Some(Color::Black);
                        } else {
                            color_to_update = Some(Color::Black);
                        }
                    } else {
                        color_to_update = Some(Color::White);
                    }
                } else {
                    color_to_update = Some(Color::White);
                };
                let color = color_to_update.unwrap();
                let move_detail = turn_builder.get_or_insert(color).get_move_detail_mut();
                match data {
                    char_match!(file) => {
                        match BoardFile::from_char(*data) {
                            Ok(board_file) => move_detail.from_board_file = Some(board_file),
                            Err(err) => {
                                error!("error parsing file - inner error: {err:?}");
                                return Err(cur_token_context.create_error());
                            }
                        }
                    },
                    char_match!(rank) => {
                        match BoardRank::from_char(*data) {
                            Ok(board_rank) => move_detail.from_board_rank = Some(board_rank),
                            Err(err) => {
                                error!("error parsing rank - inner error: {err:?}");
                                return Err(cur_token_context.create_error());
                            }
                        }
                    }
                    _ => {
                        return Err(cur_token_context.create_error());
                    }
                }
                if move_detail.chess_piece.is_none() {
                    move_detail.chess_piece = Some(Piece::Pawn.as_chess_piece(color));
                }
            }
            Token::CaptureIndicator => {
                let Some(ref mut turn_builder) = self.state.current_turn else {
                    return Err(cur_token_context.create_error())
                };
                let mut color_to_update: Option<Color> = None;
                if let Some(white) = &turn_builder.white {
                    if white.get_move_detail().is_capture.is_some() {
                        // if white defined, lets check black
                        if let Some(black) = &turn_builder.black {
                            if black.get_move_detail().is_capture.is_some() {
                                return Err(cur_token_context.create_error());
                            }
                            color_to_update = Some(Color::Black);
                        } else {
                            color_to_update = Some(Color::Black);
                        }
                    } else {
                        color_to_update = Some(Color::White);
                    }
                } else {
                    color_to_update = Some(Color::White);
                };
                let color = color_to_update.unwrap();
                let move_detail = turn_builder.get_or_insert(color).get_move_detail_mut();
                move_detail.is_capture = Some(true);
            }
            Token::MovingTo(data) => {
                let Some(ref mut turn_builder) = self.state.current_turn else {
                    return Err(cur_token_context.create_error())
                };
                let mut color_to_update: Option<Color> = None;
                if let Some(white) = &turn_builder.white {
                    if white.get_move_detail().to_pos.is_some() {
                        // if white defined, lets check black
                        if let Some(black) = &turn_builder.black {
                            if black.get_move_detail().to_pos.is_some() {
                                return Err(cur_token_context.create_error());
                            }
                            color_to_update = Some(Color::Black);
                        } else {
                            color_to_update = Some(Color::Black);
                        }
                    } else {
                        color_to_update = Some(Color::White);
                    }
                } else {
                    color_to_update = Some(Color::White);
                };
                let color = color_to_update.unwrap();
                let move_detail = turn_builder.get_or_insert(color).get_move_detail_mut();
                match BoardPosition::from_str(data) {
                    Ok(board_pos) => move_detail.to_pos = Some(board_pos),
                    Err(err) => {
                        error!("error parsing to pos - inner error: {err:?}");
                        return Err(cur_token_context.create_error());
                    }
                }
            }
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
                return Err(cur_token_context.create_error());
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
                return Err(cur_token_context.create_error());
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

