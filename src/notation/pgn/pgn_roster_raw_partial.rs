use crate::notation::pgn::pgn_parsing_error::PgnParsingError;
use crate::notation::pgn::pgn_roster_raw::PgnRosterRaw;
use crate::notation::pgn::tag_pairs::*;

#[derive(Debug, Default)]
pub struct PgnRosterRawPartial {
    pub event: Option<String>,
    // City, Region ThreeLetterCountryCode
    pub site: Option<String>,
    // YYYY.MM.DD
    pub date: Option<String>,
    // 0 - 999
    pub round: Option<String>,
    // Last name, First name
    pub white: Option<String>,
    // Last name, First name
    pub black: Option<String>,
    // white won: 1-0
    // black won: 0-1
    // draw: 1/2-1/2
    // game in progress: *
    pub result: Option<String>,

    pub annotator: Option<String>,
    pub ply_count: Option<String>,
    pub time_control: Option<String>,
    pub time: Option<String>,
    pub termination: Option<String>,
    pub mode: Option<String>,
    pub fen: Option<String>,
    pub set_up: Option<String>,
}

impl PgnRosterRawPartial {
    pub fn from_raw(raw_tag_pairs: &[String]) -> Result<Self, PgnParsingError> {
        let mut roster = Self::default();
        for raw_roster in raw_tag_pairs {
            let TagPairNameValueTuple(tag_pair_name, value) = match parse_tag_pair(&raw_roster) {
                Ok(tag_pair) => tag_pair,
                Err(err) => return Err(PgnParsingError::InvalidTagPair(err.to_string()))
            };
            match tag_pair_name.as_str() {
                event::PgnTagPairEvent::NAME => roster.event = Some(value),
                site::PgnTagPairSite::NAME => roster.site = Some(value),
                date::PgnTagPairDate::NAME => roster.date = Some(value),
                round::PgnTagPairRound::NAME => roster.round = Some(value),
                white::PgnTagPairWhite::NAME => roster.white = Some(value),
                black::PgnTagPairBlack::NAME => roster.black = Some(value),
                result::PgnTagPairResult::NAME => roster.result = Some(value),
                fen::PgnTagPairFen::NAME => roster.fen = Some(value),
                _ => return Err(PgnParsingError::InvalidTagPair(format!("unknown tag pair: {tag_pair_name}: {value}")))
            }
        }
        Ok(roster)
    }

    pub fn build(&mut self) -> Result<PgnRosterRaw, PgnParsingError> {
        let Some(event) = self.event.take() else {
            return Err(PgnParsingError::RosterMissingRequiredField("event"))
        };
        let Some(site) = self.site.take() else {
            return Err(PgnParsingError::RosterMissingRequiredField("site"))
        };
        let Some(date) = self.date.take() else {
            return Err(PgnParsingError::RosterMissingRequiredField("date"))
        };
        let Some(round) = self.round.take() else {
            return Err(PgnParsingError::RosterMissingRequiredField("round"))
        };
        let Some(white) = self.white.take() else {
            return Err(PgnParsingError::RosterMissingRequiredField("white"))
        };
        let Some(black) = self.black.take() else {
            return Err(PgnParsingError::RosterMissingRequiredField("black"))
        };
        let Some(result) = self.result.take() else {
            return Err(PgnParsingError::RosterMissingRequiredField("result"))
        };
        let mut roster = PgnRosterRaw {
            event,
            site,
            date,
            round,
            white,
            black,
            result,
            ..Default::default()
        };
        if let Some(annotator) = self.annotator.take() {
            roster.annotator = Some(annotator);
        };
        if let Some(ply_count) = self.ply_count.take() {
            roster.ply_count = Some(ply_count);
        };
        if let Some(time_control) = self.time_control.take() {
            roster.time_control = Some(time_control);
        };
        if let Some(time) = self.time.take() {
            roster.time = Some(time);
        };
        if let Some(termination) = self.termination.take() {
            roster.termination = Some(termination);
        };
        if let Some(mode) = self.mode.take() {
            roster.mode = Some(mode);
        };
        if let Some(fen) = self.fen.take() {
            roster.fen = Some(fen);
        };
        if let Some(set_up) = self.set_up.take() {
            roster.set_up = Some(set_up);
        };
        Ok(roster)
    }
}
