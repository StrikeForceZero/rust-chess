use std::io::Split;
use std::num::ParseIntError;
use itertools::Itertools;
use thiserror::Error;
use tracing::{debug, error, instrument, trace, warn};
use crate::notation::pgn::pgn_turn_data::PgnTurnData;
use crate::notation::pgn::pgn_turn_data_raw::PgnTurnDataRaw;
use crate::notation::pgn::pgn_turn_data_raw_partial::PgnTurnDataRawPartial;
use crate::notation::pgn::pgn_roster_raw_partial::PgnRosterRawPartial;
use crate::notation::pgn::pgn_roster_raw::PgnRosterRaw;
use crate::notation::pgn::tag_pairs::{parse_tag_pair, PgnTagPairParseError, TagPair, TagPairNameValueTuple};

#[derive(Default, PartialEq, Debug)]
enum ParserLookingForState {
    #[default]
    TagPair,

    New,
    WhiteMove,
    WhiteCommentStart,
    WhiteCommentEnd,

    MoveContinuationAfterComment,

    BlackMove,
    BlackCommentStart,
    BlackCommentEnd,
    CommentUntilEndOfTheLineStart,
    CommentUntilEndOfTheLineEnd,

    Result,
}

impl ParserLookingForState {
    pub const fn next(&self) -> Self {
        match self {
            Self::TagPair => Self::TagPair,
            Self::New => Self::WhiteMove,
            Self::WhiteMove => Self::WhiteCommentStart,
            Self::WhiteCommentStart => Self::WhiteCommentEnd,
            Self::WhiteCommentEnd => Self::MoveContinuationAfterComment,
            Self::MoveContinuationAfterComment => Self::BlackMove,
            Self::BlackMove => Self::BlackCommentStart,
            Self::BlackCommentStart => Self::BlackCommentEnd,
            Self::BlackCommentEnd => Self::CommentUntilEndOfTheLineStart,
            Self::CommentUntilEndOfTheLineStart => Self::CommentUntilEndOfTheLineEnd,
            Self::CommentUntilEndOfTheLineEnd => Self::New,
            // TODO: would it be better to return None?
            Self::Result => Self::Result,
        }
    }
    pub const fn next_section(&self) -> Self {
        match self {
            Self::TagPair => Self::New,
            Self::New => Self::Result,
            Self::WhiteMove => Self::Result,
            Self::WhiteCommentStart => Self::Result,
            Self::WhiteCommentEnd => Self::Result,
            Self::MoveContinuationAfterComment => Self::Result,
            Self::BlackMove => Self::Result,
            Self::BlackCommentStart => Self::Result,
            Self::BlackCommentEnd => Self::Result,
            Self::CommentUntilEndOfTheLineStart => Self::Result,
            Self::CommentUntilEndOfTheLineEnd => Self::Result,
            // TODO: would it be better to return None?
            Self::Result => Self::Result,
        }
    }
}

#[derive(Debug, Clone, Error, PartialEq)]
pub enum PgnParsingError {
    #[error("Invalid PGN - {1:?} @{2}:{3}:{0:?}")]
    InvalidPgn(String, String, usize, usize),
    #[error("Invalid PGN - {0}")]
    InvalidTagPair(String),
}

impl PgnParsingError {
    #[instrument]
    pub fn create(parsing_context: &ParsingContext) -> Self {
        let LineWordPosTuple(line, word, col) = parsing_context.resolve_line_word_pos_tuple();
        Self::InvalidPgn(line, word, parsing_context.line_ix + 1, col + 1)
    }
}

type Word = String;
type Line = String;
type Pos = usize;

#[derive(Debug)]
pub struct LineWordPosTuple(Line, Word, Pos);

#[derive(Debug)]
pub struct ParsingContext<'a> {
    data: &'a str,
    line_ix: usize,
    word_ix: usize,
}

impl<'a> ParsingContext<'a> {
    pub fn create(data: &'a str) -> Self {
        Self {
            data,
            line_ix: 0,
            word_ix: 0,
        }
    }
    pub fn update(&mut self, line_ix: usize, word_ix: usize) {
        self.line_ix = line_ix;
        self.word_ix = word_ix;
    }
    fn resolve_line_word_pos_tuple(&self) -> LineWordPosTuple {
        let lines = self.data.split(NEW_LINE).map(|line| line.split(SPACE).collect_vec()).collect_vec();
        let line_words = lines.get(self.line_ix).expect("context has invalid line index");
        let word = line_words.get(self.word_ix).expect("context has invalid word index");
        let pos = &line_words[0..self.word_ix].join(" ").chars().collect_vec().len();
        LineWordPosTuple(line_words.join(" "), word.to_string(), *pos)
    }
    fn get_line(&self) -> String {
        self.data.split(NEW_LINE).collect_vec().get(self.line_ix).expect("context has invalid line index").to_string()
    }
}

#[derive(Default, Debug)]
pub struct Parser {
    state: ParserLookingForState,
    roster_raw: Vec<String>,
    roster: Option<PgnRosterRawPartial>,
    current_turn: Option<PgnTurnDataRawPartial>,
    raw_turns: Vec<PgnTurnDataRawPartial>,
    turns: Vec<PgnTurnDataRaw>,
}

const NEW_LINE: char = '\n';
const SPACE: char = ' ';

#[derive(Debug)]
pub enum WordHandleResult {
    NextWord,
    NextLine,
}

fn is_result_word(word: &str) -> bool {
    word.contains("1-0") || word.contains("0-1") || word.contains("1/2-1/2") || word.contains("*")
}

impl Parser {
    fn next_turn(&mut self) {
        trace!("next_turn");
        self.state = ParserLookingForState::New;
    }
    fn next(&mut self) {
        trace!("next");
        self.state = self.state.next();
    }

    fn next_section(&mut self) {
        trace!("next-section");
        self.state = self.state.next_section();
    }

    fn handle_word(
        &mut self,
        context: &ParsingContext,
        word: &str,
    ) -> Result<WordHandleResult, PgnParsingError> {
        trace!("parser state: {:?}", self.state);
        trace!("current word: \"{word}\"");
        let res = match self.state {
            ParserLookingForState::TagPair => {
                if word.is_empty() || word.ends_with('.') {
                    trace!("not tag pair 1");
                    self.next_section();
                    // reparse current word
                    trace!("reparse");
                    return self.handle_word(context, word);
                } else {
                    trace!("maybe tag pair");
                    let line = context.get_line();
                    if !word.starts_with('[') || !line.ends_with(']') {
                        trace!("not tag pair 2");
                        return Err(PgnParsingError::create(&context));
                    }
                    trace!("pushing to roster_raw: \"{line}\"");
                    self.roster_raw.push(line.to_string());
                }
                WordHandleResult::NextLine
            }
            ParserLookingForState::New => {
                if word.is_empty() {
                    trace!("empty");
                    return Ok(WordHandleResult::NextWord);
                }
                if !word.ends_with('.') {
                    trace!("not move number");
                    if is_result_word(word) {
                        trace!("is result str");
                        self.next_section();
                        // reparse current word
                        trace!("reparse");
                        return self.handle_word(context, word)
                    }
                    return Err(PgnParsingError::create(&context));
                }
                /*
                let parts = word.split('.').collect_vec();
                if parts.len() != 2 {
                    return Err(PgnParsingError::create(data, line_num, word_num));
                }
                let num_str = parts.first().unwrap();
                let num = match num_str.parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => return Err(PgnParsingError::create(data, line_num, word_num)),
                };
                if parser.current_turn_number + 1 != num {
                    return Err(PgnParsingError::create(data, line_num, word_num));
                }*/
                trace!("is move number");
                let previous_turn = self.current_turn.replace(PgnTurnDataRawPartial::default());;
                if let Some(turn) = previous_turn {
                    trace!("saving previous turn: {turn:?}");
                    self.raw_turns.push(turn);
                }
                let Some(ref mut current_turn) = &mut self.current_turn else {
                    trace!("no current turn!");
                    return Err(PgnParsingError::create(&context));
                };
                current_turn.turn_number = Some(word.to_string());
                trace!("current_turn.turn_number = {:?}", current_turn.turn_number);
                self.next();
                WordHandleResult::NextWord
            }
            ParserLookingForState::WhiteMove => {
                let Some(ref mut current_turn) = &mut self.current_turn else {
                    trace!("no current turn!");
                    return Err(PgnParsingError::create(&context));
                };
                current_turn.white = Some(word.to_string());
                trace!("current_turn.white = {:?}", current_turn.white);
                self.next();
                WordHandleResult::NextWord
            }
            ParserLookingForState::WhiteCommentStart => {
                self.next();
                if word.starts_with('{') {
                    trace!("white comment start");
                    let Some(ref mut current_turn) = &mut self.current_turn else {
                        trace!("no current turn!");
                        return Err(PgnParsingError::create(&context));
                    };
                    current_turn.white_comment = Some(word.to_string());
                    trace!("current_turn.white_comment = {:?}", current_turn.white_comment);
                } else {
                    trace!("not white comment");
                    // reparse current word
                    trace!("reparse");
                    return self.handle_word(context, word)
                }
                WordHandleResult::NextWord
            }
            ParserLookingForState::WhiteCommentEnd => {
                let Some(ref mut current_turn) = &mut self.current_turn else {
                    trace!("no current turn!");
                    return Err(PgnParsingError::create(&context));
                };
                if !word.ends_with('}') {
                    trace!("not white comment end");
                    // comment not terminated
                    if current_turn.white_comment.is_some() {
                        trace!("white comment not terminated");
                        return Err(PgnParsingError::create(&context));
                    }
                    self.next();
                    // reparse current word
                    trace!("reparse");
                    return self.handle_word(context, word)
                }
                let Some(ref mut current_comment) = &mut current_turn.white_comment else {
                    trace!("no white comment!");
                    return Err(PgnParsingError::create(&context));
                };
                let word_with_space_added_back = format!(" {word}");
                trace!("current_comment.push_str({:?})", word);
                current_comment.push_str(&word_with_space_added_back);
                trace!("current_comment: {:?}", current_comment);
                if word.ends_with('}') {
                    trace!("white comment end");
                    self.next();
                }
                WordHandleResult::NextWord
            }
            ParserLookingForState::MoveContinuationAfterComment => {
                let Some(current_turn) = &self.current_turn else {
                    trace!("no current turn!");
                    return Err(PgnParsingError::create(&context));
                };
                if current_turn.white_comment.is_some() {
                    trace!("white comment exists");
                    if !word.ends_with("...") {
                        trace!("not move number continuation");
                        if is_result_word(word) {
                            trace!("is result");
                            self.next_section();
                            // reparse current word
                            trace!("reparse");
                            return self.handle_word(context, word)
                        }
                        trace!("missing move number continuation!");
                        return Err(PgnParsingError::create(&context));
                    }
                    let Some(move_str) = &current_turn.turn_number else {
                        trace!("missing move number!");
                        return Err(PgnParsingError::create(&context));
                    };
                    // make sure move numbers match
                    if !word.contains(move_str) {
                        trace!("{word:?} does not contain {move_str:?}!");
                        return Err(PgnParsingError::create(&context));
                    }
                    trace!("{word:?} contains {move_str:?}!");
                    self.next();
                    WordHandleResult::NextWord
                } else {
                    trace!("not move continuation");
                    self.next();
                    // reparse current word
                    trace!("reparse");
                    self.handle_word(context, word)?
                }
            }
            ParserLookingForState::BlackMove => {
                if is_result_word(word) {
                    trace!("not black move; is result");
                    self.next_section();
                    // reparse current word
                    trace!("reparse");
                    return self.handle_word(context, word);
                }
                if word.starts_with(';') {
                    trace!("not black move; is end of line comment");
                    self.next();
                    // reparse current word
                    trace!("reparse");
                    return self.handle_word(context, word);
                }
                trace!("is black move");
                let Some(ref mut current_turn) = &mut self.current_turn else {
                    trace!("no current turn!");
                    return Err(PgnParsingError::create(&context));
                };
                current_turn.black = Some(word.to_string());
                trace!("current_turn.black = {:?}", current_turn.black);
                self.next();
                WordHandleResult::NextWord
            }
            ParserLookingForState::BlackCommentStart => {
                self.next();
                if word.starts_with('{') {
                    trace!("is black comment start");
                    let Some(ref mut current_turn) = &mut self.current_turn else {
                        trace!("no current turn!");
                        return Err(PgnParsingError::create(&context));
                    };
                    current_turn.black_comment = Some(word.to_string());
                    trace!("current_turn.black_comment = {:?}", current_turn.black_comment);
                } else {
                    trace!("not black comment start");
                    // reparse current word
                    trace!("reparse");
                    return self.handle_word(context, word);
                }
                WordHandleResult::NextWord
            }
            ParserLookingForState::BlackCommentEnd => {
                let Some(ref mut current_turn) = &mut self.current_turn else {
                    trace!("no current turn!");
                    return Err(PgnParsingError::create(&context));
                };
                if !word.ends_with('}') {
                    // comment not terminated
                    if current_turn.black_comment.is_some() {
                        trace!("black comment not terminated! {:?}", current_turn.black_comment);
                        return Err(PgnParsingError::create(&context));
                    }
                    trace!("not black comment");
                    self.next();
                    // reparse current word
                    trace!("reparse");
                    return self.handle_word(context, word)
                }
                trace!("is black comment");
                let Some(ref mut current_comment) = &mut current_turn.black_comment else {
                    trace!("no current comment!");
                    return Err(PgnParsingError::create(&context));
                };
                let word_with_space_added_back = format!(" {word}");
                trace!("current_comment.push_str({:?})", word);
                current_comment.push_str(&word_with_space_added_back);
                trace!("current_comment: {:?}", current_comment);
                if word.ends_with('}') {
                    trace!("black comment end");
                    self.next();
                }
                WordHandleResult::NextWord
            }
            ParserLookingForState::CommentUntilEndOfTheLineStart => {
                if !word.starts_with(';') {
                    trace!("not end of line comment");
                    self.next_turn();
                    // reparse current word
                    trace!("reparse");
                    return self.handle_word(context, word)
                }
                let Some(ref mut current_turn) = &mut self.current_turn else {
                    trace!("no current turn!");
                    return Err(PgnParsingError::create(&context));
                };
                current_turn.comment = Some(word.to_string());
                trace!("current_turn.comment = {:?}", current_turn.comment);
                self.next();
                WordHandleResult::NextWord
            }
            ParserLookingForState::CommentUntilEndOfTheLineEnd => {
                let Some(ref mut current_turn) = &mut self.current_turn else {
                    trace!("no current turn!");
                    return Err(PgnParsingError::create(&context));
                };
                let Some(ref mut current_comment) = &mut current_turn.comment else {
                    trace!("no current comment!");
                    return Err(PgnParsingError::create(&context));
                };
                let word_with_space_added_back = format!(" {word}");
                trace!("current_comment.push_str({:?})", word);
                current_comment.push_str(&word_with_space_added_back);
                trace!("current_comment: {:?}", current_comment);
                if word.ends_with('\n') {
                    trace!("end of line comment");
                    self.next()
                }
                WordHandleResult::NextWord
            }
            ParserLookingForState::Result => {
                if !is_result_word(word) {
                    trace!("not result!");
                    return Err(PgnParsingError::create(&context));
                }
                warn!("TODO: handle result");
                // TODO: handle result
                WordHandleResult::NextWord
            }
        };
        Ok(res)
    }


    #[tracing::instrument]
    pub fn parse(data: &str) -> Result<Self, PgnParsingError> {
        let mut parser = Self::default();
        let mut context = ParsingContext::create(data);
        let lines = data.split(NEW_LINE);
        for (line_ix, line) in lines.enumerate() {
            let line_num = line_ix + 1;
            trace!("line {line_num}: \"{line}\"");
            if parser.state != ParserLookingForState::TagPair {
                parser.state = ParserLookingForState::New;
            }
            let words = line.split(" ");
            trace!("words: {:?}", words.clone().collect_vec());
            for (word_ix, word) in words.enumerate() {
                context.update(line_ix, word_ix);
                let word_num = word_ix + 1;
                trace!("word {word_num}: \"{word}\"");
                let res = parser.handle_word(&context, word)?;
                trace!("handle_word res: {res:?}");
                match res {
                    WordHandleResult::NextWord => continue,
                    WordHandleResult::NextLine => break,
                }
            }
        }
        if let Some(current_turn) = parser.current_turn.take() {
            trace!("saving previous turn: {current_turn:?}");
            parser.raw_turns.push(current_turn);
        }
        Ok(parser)
    }

    pub fn build_roster(&mut self) -> Result<(), PgnParsingError> {
        self.roster = Some(PgnRosterRawPartial::from_raw(self.roster_raw.as_slice())?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Index;
    use rstest::rstest;
    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::fmt::format::FmtSpan;
    use super::*;

    #[rstest]
    #[case(
        "1. Nxb5 {white comment} 1... Kh8 {black comment} ;line comment\n",
        vec![
            PgnTurnDataRawPartial {
                turn_number: Some("1.".to_string()),
                white: Some("Nxb5".to_string()),
                white_comment: Some("{white comment}".to_string()),
                black: Some("Kh8".to_string()),
                black_comment: Some("{black comment}".to_string()),
                comment: Some(";line comment".to_string()),
            },
        ],
    )]
    #[case(
        "1. Nxb5 {white comment} 1... Kh8 {black comment} ;line comment\n\
         2. f7 b6 3. Qa1#",
        vec![
            PgnTurnDataRawPartial {
                turn_number: Some("1.".to_string()),
                white: Some("Nxb5".to_string()),
                white_comment: Some("{white comment}".to_string()),
                black: Some("Kh8".to_string()),
                black_comment: Some("{black comment}".to_string()),
                comment: Some(";line comment".to_string()),
            },
            PgnTurnDataRawPartial {
                turn_number: Some("2.".to_string()),
                white: Some("f7".to_string()),
                white_comment: None,
                black: Some("b6".to_string()),
                black_comment: None,
                comment: None,
            },
            PgnTurnDataRawPartial {
                turn_number: Some("3.".to_string()),
                white: Some("Qa1#".to_string()),
                white_comment: None,
                black: None,
                black_comment: None,
                comment: None,
            },
        ],
    )]
    fn test_parser_partial_turns(
        #[case] input: &'static str,
        #[case] expected: Vec<PgnTurnDataRawPartial>,
    ) -> Result<(), PgnParsingError> {
        // crate::utils::tracing::init_tracing();
        let parser = Parser::parse(input)?;
        assert_eq!(expected, parser.raw_turns);
        Ok(())
    }

    #[rstest]
    #[case(
        "[Event \"Foo Bar\"]",
        vec![
            "[Event \"Foo Bar\"]",
        ],
    )]
    fn test_parser_partial_roster(
        #[case] input: &'static str,
        #[case] expected: Vec<&'static str>,
    ) -> Result<(), PgnParsingError> {
        // crate::utils::tracing::init_tracing();
        let parser = Parser::parse(input)?;
        assert_eq!(expected, parser.roster_raw);
        Ok(())
    }

    fn resolve_col(line: &str, word_ix: usize) -> usize {
        let mut context = ParsingContext::create(line);
        context.word_ix = word_ix;
        let LineWordPosTuple(_line, _word, pos) = context.resolve_line_word_pos_tuple();
        pos
    }

    struct MatchinErrorTuple {
        data: &'static str,
        error: PgnParsingError,
    }

    impl MatchinErrorTuple {
        fn create(data: &'static str, bad_data: &'static str) -> Self {
            let lines = data.lines().collect_vec();
            let (bad_data_line_ix, _) = lines.iter().find_position(|line| line.contains(bad_data)).expect(format!("couldn't find {bad_data:?} in {data:?}").as_str());
            let line = lines.get(bad_data_line_ix).expect("invalid line ix");
            let line_words = line.split(SPACE).collect_vec();
            let (word_ix, _) = line_words.iter().find_position(|word| word.contains(bad_data)).expect(format!("couldn't find {bad_data:?} in {data:?} line {line:?}").as_str());
            Self {
                data,
                error: PgnParsingError::InvalidPgn(data.to_string(), bad_data.to_string(), bad_data_line_ix + 1, resolve_col(line, word_ix) + 1)
            }
        }
    }

    #[rstest]
    #[case(
        MatchinErrorTuple::create("1. Nxb5 {white comment} 2... Kh8", "2..."),
    )]
    fn test_parser_failures(
        #[case] input: MatchinErrorTuple,
    ) {
        // crate::utils::tracing::init_tracing();
        let MatchinErrorTuple { data, error } = input;
        let parser_result = Parser::parse(data);
        assert_eq!(Some(error), parser_result.err());
    }
}
