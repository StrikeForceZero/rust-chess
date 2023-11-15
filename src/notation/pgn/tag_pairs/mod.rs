use itertools::Itertools;
use thiserror::Error;
use std::collections::VecDeque;
use std::fmt::Display;

pub mod result;
pub mod event;
pub mod site;
pub mod date;
pub mod round;
pub mod white;
pub mod black;
pub mod fen;
mod macros;

#[derive(Debug, PartialEq)]
pub enum TagPair {
    Event(event::PgnTagPairEvent),
    Site(site::PgnTagPairSite),
    Date(date::PgnTagPairDate),
    Round(round::PgnTagPairRound),
    White(white::PgnTagPairWhite),
    Black(black::PgnTagPairBlack),
    Result(result::PgnTagPairResult),
    Fen(fen::PgnTagPairFen),
}

pub trait NamedTagPair: std::fmt::Display + std::fmt::Debug {
    const NAME: &'static str;

    fn create_parsing_error(value: &str) -> PgnTagPairParseError where Self: Sized {
        PgnTagPairParseError::FailedParsingTagPair(Self::NAME, value.to_string())
    }
}

macro_rules! impl_named_tag_pair_for {
    ($struct_name:ident, $name:expr) => {
        use crate::notation::pgn::tag_pairs::NamedTagPair;

        impl NamedTagPair for $struct_name {
            const NAME: &'static str = $name;
        }
    };
}
pub(crate) use impl_named_tag_pair_for;
use crate::notation::pgn::pgn_parsing_error::PgnParsingError;


#[derive(Error, Debug, Clone, PartialEq)]
pub enum PgnTagPairParseError {
    #[error("Failed parsing {0}: {1}")]
    FailedParsingTagPair(&'static str, String),
    #[error("{0} does not appear to be a valid tag pair")]
    InvalidTagString(String),
    #[error("{0} does not appear to be a known tag pair")]
    UnknownTagPair(String),
}

const SPLIT_STR: &str =  " ";

pub struct TagPairNameValueTuple(pub String, pub String);

pub fn parse_tag_pair(line: &str) -> Result<TagPairNameValueTuple, PgnTagPairParseError> {
    if !line.starts_with('[') || !line.ends_with(']') {
        return Err(PgnTagPairParseError::InvalidTagString(line.to_string()));
    }
    let line_len = line.len();
    let original_line = line;
    let line = &line[1..line_len - 1];
    let mut parts = line.split_terminator(SPLIT_STR).collect::<VecDeque<_>>();
    let maybe_tag_name = parts.pop_front();
    if maybe_tag_name.is_none() || parts.is_empty() {
        return Err(PgnTagPairParseError::InvalidTagString(original_line.to_string()));
    }
    let tag_name = maybe_tag_name.unwrap();
    let value = parts.iter().join(SPLIT_STR);
    if !value.starts_with('"') || !value.ends_with('"') || value.is_empty() {
        return Err(PgnTagPairParseError::InvalidTagString(line.to_string()));
    }
    let value_len = value.len();
    // remove quotes
    let value = &value[1..value_len - 1];
    Ok(TagPairNameValueTuple(tag_name.to_string(), value.to_string()))
}

pub fn resolve_tag_pair(tag_pair_name_value_tuple: TagPairNameValueTuple) -> Result<TagPair, PgnTagPairParseError> {
    let TagPairNameValueTuple(tag_name, value) = tag_pair_name_value_tuple;
    let tag_pair= match tag_name.as_str() {
        event::PgnTagPairEvent::NAME => TagPair::Event(event::PgnTagPairEvent::from_str(&value)),
        site::PgnTagPairSite::NAME => TagPair::Site(site::PgnTagPairSite::from_str(&value)?),
        date::PgnTagPairDate::NAME => TagPair::Date(date::PgnTagPairDate::from_str(&value)?),
        round::PgnTagPairRound::NAME => TagPair::Round(round::PgnTagPairRound::from_str(&value)?),
        white::PgnTagPairWhite::NAME => TagPair::White(white::PgnTagPairWhite::from_str(&value)?),
        black::PgnTagPairBlack::NAME => TagPair::Black(black::PgnTagPairBlack::from_str(&value)?),
        result::PgnTagPairResult::NAME => TagPair::Result(result::PgnTagPairResult::from_str(&value)?),
        // optional
        fen::PgnTagPairFen::NAME => TagPair::Fen(fen::PgnTagPairFen::from_str(&value)?),
        _ => return Err(PgnTagPairParseError::UnknownTagPair(format!("[{tag_name} \"{value}\"]"))),
    };
    Ok(tag_pair)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;
    use crate::notation::fen::Fen;

    #[rstest]
    #[case(
        "[Event \"Some Event Name\"]",
        Ok(TagPair::Event(event::PgnTagPairEvent("Some Event Name".to_string()))),
    )]
    #[rstest]
    #[case(
        "[Site \"Some City, Some Region AAA\"]",
        Ok(TagPair::Site(site::PgnTagPairSite { city: "Some City".to_string(), region: "Some Region".to_string(), country_code: "AAA".to_string() })),
    )]
    #[case(
        "[Date \"4567.01.23\"]",
        Ok(TagPair::Date(date::PgnTagPairDate("4567.01.23".to_string()))),
    )]
    #[case(
        "[Round \"123\"]",
        Ok(TagPair::Round(round::PgnTagPairRound(123))),
    )]
    #[case(
        "[White \"Last, First W.\"]",
        Ok(TagPair::White(white::PgnTagPairWhite("Last, First W.".to_string()))),
    )]
    #[case(
        "[Black \"Last, First B.\"]",
        Ok(TagPair::Black(black::PgnTagPairBlack("Last, First B.".to_string()))),
    )]
    #[case(
        "[Result \"1-0\"]",
        Ok(TagPair::Result(result::PgnTagPairResult::WhiteWon)),
    )]
    #[case(
        "[Result \"0-1\"]",
        Ok(TagPair::Result(result::PgnTagPairResult::BlackWon)),
    )]
    #[case(
        "[Result \"1/2-1/2\"]",
        Ok(TagPair::Result(result::PgnTagPairResult::Draw)),
    )]
    #[case(
        "[Result \"*\"]",
        Ok(TagPair::Result(result::PgnTagPairResult::GameInProgress)),
    )]
    #[case(
        "[FEN \"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1\"]",
        Ok(TagPair::Fen(fen::PgnTagPairFen(Fen::Owned("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string())))),
    )]
    fn test_resolve_tag_pair(
        #[case] input: &'static str,
        #[case] expected: Result<TagPair, PgnTagPairParseError>
    ) -> Result<(), PgnTagPairParseError> {
        let tag_pair_tuple = parse_tag_pair(input)?;
        assert_eq!(expected, resolve_tag_pair(tag_pair_tuple));
        Ok(())
    }
}
