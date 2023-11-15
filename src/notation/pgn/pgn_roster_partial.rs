use crate::notation::pgn::pgn_parsing_error::PgnParsingError;
use crate::notation::pgn::pgn_roster::PgnRoster;
use crate::notation::pgn::tag_pairs::black::PgnTagPairBlack;
use crate::notation::pgn::tag_pairs::date::PgnTagPairDate;
use crate::notation::pgn::tag_pairs::event::PgnTagPairEvent;
use crate::notation::pgn::tag_pairs::fen::PgnTagPairFen;
use crate::notation::pgn::tag_pairs::result::PgnTagPairResult;
use crate::notation::pgn::tag_pairs::round::PgnTagPairRound;
use crate::notation::pgn::tag_pairs::site::PgnTagPairSite;
use crate::notation::pgn::tag_pairs::white::PgnTagPairWhite;

#[derive(Debug, Default)]
pub struct PgnRosterPartial {
    pub event: Option<PgnTagPairEvent>,
    // City, Region ThreeLetterCountryCode
    pub site: Option<PgnTagPairSite>,
    // YYYY.MM.DD
    pub date: Option<PgnTagPairDate>,
    // 0 - 999
    pub round: Option<PgnTagPairRound>,
    // Last name, First name
    pub white: Option<PgnTagPairWhite>,
    // Last name, First name
    pub black: Option<PgnTagPairBlack>,
    // white won: 1-0
    // black won: 0-1
    // draw: 1/2-1/2
    // game in progress: *
    pub result: Option<PgnTagPairResult>,

    // TODO: implement
    // pub annotator: Option<String>,
    // pub ply_count: Option<String>,
    // pub time_control: Option<String>,
    // pub time: Option<String>,
    // pub termination: Option<String>,
    // pub mode: Option<String>,
    pub fen: Option<PgnTagPairFen>,
    // pub set_up: Option<String>,
}

impl PgnRosterPartial {
    pub fn build(&mut self) -> Result<PgnRoster, PgnParsingError> {
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
        let mut roster = PgnRoster {
            event,
            site,
            date,
            round,
            white,
            black,
            result,

            fen: None,
        };
        /*if let Some(annotator) = self.annotator.take() {
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
        };*/
        if let Some(fen) = self.fen.take() {
            roster.fen = Some(fen);
        };
        /*if let Some(set_up) = self.set_up.take() {
            roster.set_up = Some(set_up);
        };*/
        Ok(roster)
    }
}
