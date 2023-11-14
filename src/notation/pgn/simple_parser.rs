use itertools::Itertools;
use tracing::{instrument, trace, warn};
use crate::notation::pgn::pgn_parsing_error::PgnParsingError;
use crate::notation::pgn::pgn_turn_data_raw::PgnTurnDataRaw;
use crate::notation::pgn::pgn_turn_data_raw_partial::PgnTurnDataRawPartial;
use crate::notation::pgn::pgn_roster_raw_partial::PgnRosterRawPartial;
use crate::notation::pgn::util::{LineWordPosTuple, NEW_LINE, SPACE};

#[derive(Default, PartialEq, Debug)]
enum SimpleParserState {
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

impl SimpleParserState {
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

#[derive(Debug)]
pub struct SimpleParserContext<'a> {
    pub(crate) data: &'a str,
    pub(crate) line_ix: usize,
    pub(crate) word_ix: usize,
}

impl<'a> SimpleParserContext<'a> {
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
    pub(crate) fn resolve_line_word_pos_tuple(&self) -> LineWordPosTuple {
        let lines = self.data.split(NEW_LINE).map(|line| line.split(SPACE).collect_vec()).collect_vec();
        let line_words = lines.get(self.line_ix).expect("context has invalid line index");
        let word = line_words.get(self.word_ix).expect("context has invalid word index");
        let pos = &line_words[0..self.word_ix].join(" ").chars().collect_vec().len();
        LineWordPosTuple(line_words.join(" "), word.to_string(), *pos)
    }
    fn get_line(&self) -> String {
        self.data.split(NEW_LINE).collect_vec().get(self.line_ix).expect("context has invalid line index").to_string()
    }

    #[instrument]
    pub fn create_parsing_error(&self) -> PgnParsingError {
        let LineWordPosTuple(line, word, col) = self.resolve_line_word_pos_tuple();
        PgnParsingError::InvalidPgn(line, word, self.line_ix + 1, col + 1)
    }
}

#[derive(Default, Debug)]
pub struct SimpleParser {
    state: SimpleParserState,
    roster_raw: Vec<String>,
    roster: Option<PgnRosterRawPartial>,
    current_turn: Option<PgnTurnDataRawPartial>,
    raw_turns: Vec<PgnTurnDataRawPartial>,
    turns: Vec<PgnTurnDataRaw>,
}

#[derive(Debug)]
pub enum HandleWordResult {
    NextWord,
    NextLine,
}

fn is_result_word(word: &str) -> bool {
    word.contains("1-0") || word.contains("0-1") || word.contains("1/2-1/2") || word.contains("*")
}

impl SimpleParser {
    fn next_turn(&mut self) {
        trace!("next_turn");
        self.state = SimpleParserState::New;
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
        context: &SimpleParserContext,
        word: &str,
    ) -> Result<HandleWordResult, PgnParsingError> {
        trace!("parser state: {:?}", self.state);
        trace!("current word: \"{word}\"");
        let res = match self.state {
            SimpleParserState::TagPair => {
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
                        return Err(context.create_parsing_error());
                    }
                    trace!("pushing to roster_raw: \"{line}\"");
                    self.roster_raw.push(line.to_string());
                }
                HandleWordResult::NextLine
            }
            SimpleParserState::New => {
                if word.is_empty() {
                    trace!("empty");
                    return Ok(HandleWordResult::NextWord);
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
                    return Err(context.create_parsing_error());
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
                    return Err(context.create_parsing_error());
                };
                current_turn.turn_number = Some(word.to_string());
                trace!("current_turn.turn_number = {:?}", current_turn.turn_number);
                self.next();
                HandleWordResult::NextWord
            }
            SimpleParserState::WhiteMove => {
                let Some(ref mut current_turn) = &mut self.current_turn else {
                    trace!("no current turn!");
                    return Err(context.create_parsing_error());
                };
                current_turn.white = Some(word.to_string());
                trace!("current_turn.white = {:?}", current_turn.white);
                self.next();
                HandleWordResult::NextWord
            }
            SimpleParserState::WhiteCommentStart => {
                self.next();
                if word.starts_with('{') {
                    trace!("white comment start");
                    let Some(ref mut current_turn) = &mut self.current_turn else {
                        trace!("no current turn!");
                        return Err(context.create_parsing_error());
                    };
                    current_turn.white_comment = Some(word.to_string());
                    trace!("current_turn.white_comment = {:?}", current_turn.white_comment);
                } else {
                    trace!("not white comment");
                    // reparse current word
                    trace!("reparse");
                    return self.handle_word(context, word)
                }
                HandleWordResult::NextWord
            }
            SimpleParserState::WhiteCommentEnd => {
                let Some(ref mut current_turn) = &mut self.current_turn else {
                    trace!("no current turn!");
                    return Err(context.create_parsing_error());
                };
                if !word.ends_with('}') {
                    trace!("not white comment end");
                    // comment not terminated
                    if current_turn.white_comment.is_some() {
                        trace!("white comment not terminated");
                        return Err(context.create_parsing_error());
                    }
                    self.next();
                    // reparse current word
                    trace!("reparse");
                    return self.handle_word(context, word)
                }
                let Some(ref mut current_comment) = &mut current_turn.white_comment else {
                    trace!("no white comment!");
                    return Err(context.create_parsing_error());
                };
                let word_with_space_added_back = format!(" {word}");
                trace!("current_comment.push_str({:?})", word);
                current_comment.push_str(&word_with_space_added_back);
                trace!("current_comment: {:?}", current_comment);
                if word.ends_with('}') {
                    trace!("white comment end");
                    self.next();
                }
                HandleWordResult::NextWord
            }
            SimpleParserState::MoveContinuationAfterComment => {
                let Some(ref mut current_turn) = &mut self.current_turn else {
                    trace!("no current turn!");
                    return Err(context.create_parsing_error());
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
                        return Err(context.create_parsing_error());
                    }
                    let Some(move_str) = &current_turn.turn_number else {
                        trace!("missing move number!");
                        return Err(context.create_parsing_error());
                    };
                    // make sure move numbers match
                    if !word.contains(move_str) {
                        trace!("{word:?} does not contain {move_str:?}!");
                        return Err(context.create_parsing_error());
                    }
                    trace!("{word:?} contains {move_str:?}!");
                    current_turn.turn_number_continuation = Some(word.into());
                    trace!("current_turn.turn_number_continuation = {:?}", current_turn.turn_number_continuation);
                    self.next();
                    HandleWordResult::NextWord
                } else {
                    trace!("not move continuation");
                    self.next();
                    // reparse current word
                    trace!("reparse");
                    self.handle_word(context, word)?
                }
            }
            SimpleParserState::BlackMove => {
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
                    return Err(context.create_parsing_error());
                };
                current_turn.black = Some(word.to_string());
                trace!("current_turn.black = {:?}", current_turn.black);
                self.next();
                HandleWordResult::NextWord
            }
            SimpleParserState::BlackCommentStart => {
                self.next();
                if word.starts_with('{') {
                    trace!("is black comment start");
                    let Some(ref mut current_turn) = &mut self.current_turn else {
                        trace!("no current turn!");
                        return Err(context.create_parsing_error());
                    };
                    current_turn.black_comment = Some(word.to_string());
                    trace!("current_turn.black_comment = {:?}", current_turn.black_comment);
                } else {
                    trace!("not black comment start");
                    // reparse current word
                    trace!("reparse");
                    return self.handle_word(context, word);
                }
                HandleWordResult::NextWord
            }
            SimpleParserState::BlackCommentEnd => {
                let Some(ref mut current_turn) = &mut self.current_turn else {
                    trace!("no current turn!");
                    return Err(context.create_parsing_error());
                };
                if !word.ends_with('}') {
                    // comment not terminated
                    if current_turn.black_comment.is_some() {
                        trace!("black comment not terminated! {:?}", current_turn.black_comment);
                        return Err(context.create_parsing_error());
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
                    return Err(context.create_parsing_error());
                };
                let word_with_space_added_back = format!(" {word}");
                trace!("current_comment.push_str({:?})", word);
                current_comment.push_str(&word_with_space_added_back);
                trace!("current_comment: {:?}", current_comment);
                if word.ends_with('}') {
                    trace!("black comment end");
                    self.next();
                }
                HandleWordResult::NextWord
            }
            SimpleParserState::CommentUntilEndOfTheLineStart => {
                if !word.starts_with(';') {
                    trace!("not end of line comment");
                    self.next_turn();
                    // reparse current word
                    trace!("reparse");
                    return self.handle_word(context, word)
                }
                let Some(ref mut current_turn) = &mut self.current_turn else {
                    trace!("no current turn!");
                    return Err(context.create_parsing_error());
                };
                current_turn.comment = Some(word.to_string());
                trace!("current_turn.comment = {:?}", current_turn.comment);
                self.next();
                HandleWordResult::NextWord
            }
            SimpleParserState::CommentUntilEndOfTheLineEnd => {
                let Some(ref mut current_turn) = &mut self.current_turn else {
                    trace!("no current turn!");
                    return Err(context.create_parsing_error());
                };
                let Some(ref mut current_comment) = &mut current_turn.comment else {
                    trace!("no current comment!");
                    return Err(context.create_parsing_error());
                };
                let word_with_space_added_back = format!(" {word}");
                trace!("current_comment.push_str({:?})", word);
                current_comment.push_str(&word_with_space_added_back);
                trace!("current_comment: {:?}", current_comment);
                if word.ends_with('\n') {
                    trace!("end of line comment");
                    self.next()
                }
                HandleWordResult::NextWord
            }
            SimpleParserState::Result => {
                if !is_result_word(word) {
                    trace!("not result!");
                    return Err(context.create_parsing_error());
                }
                warn!("TODO: handle result");
                // TODO: handle result
                HandleWordResult::NextWord
            }
        };
        Ok(res)
    }


    #[tracing::instrument]
    pub fn parse(data: &str) -> Result<Self, PgnParsingError> {
        let mut parser = Self::default();
        let mut context = SimpleParserContext::create(data);
        let lines = data.split(NEW_LINE);
        for (line_ix, line) in lines.enumerate() {
            let line_num = line_ix + 1;
            trace!("line {line_num}: \"{line}\"");
            if parser.state != SimpleParserState::TagPair {
                parser.state = SimpleParserState::New;
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
                    HandleWordResult::NextWord => continue,
                    HandleWordResult::NextLine => break,
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
    use super::*;

    #[rstest]
    #[case(
        "1. Nxb5 {white comment} 1... Kh8 {black comment} ;line comment\n",
        vec![
            PgnTurnDataRawPartial {
                turn_number: Some("1.".to_string()),
                white: Some("Nxb5".to_string()),
                white_comment: Some("{white comment}".to_string()),
                turn_number_continuation: Some("1...".into()),
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
                turn_number_continuation: Some("1...".into()),
                black: Some("Kh8".to_string()),
                black_comment: Some("{black comment}".to_string()),
                comment: Some(";line comment".to_string()),
            },
            PgnTurnDataRawPartial {
                turn_number: Some("2.".to_string()),
                white: Some("f7".to_string()),
                turn_number_continuation: None,
                white_comment: None,
                black: Some("b6".to_string()),
                black_comment: None,
                comment: None,
            },
            PgnTurnDataRawPartial {
                turn_number: Some("3.".to_string()),
                white: Some("Qa1#".to_string()),
                white_comment: None,
                turn_number_continuation: None,
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
        let parser = SimpleParser::parse(input)?;
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
        let parser = SimpleParser::parse(input)?;
        assert_eq!(expected, parser.roster_raw);
        Ok(())
    }

    fn resolve_col(line: &str, word_ix: usize) -> usize {
        let mut context = SimpleParserContext::create(line);
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
        let parser_result = SimpleParser::parse(data);
        assert_eq!(Some(error), parser_result.err());
    }
}
