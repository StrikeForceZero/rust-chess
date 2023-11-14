use crate::notation::pgn::pgn_roster::PgnRoster;
use crate::notation::pgn::tag_pairs::*;

#[derive(Debug, Default)]
pub struct PgnRosterRaw {
    pub event: String,
    // City, Region ThreeLetterCountryCode
    pub site: String,
    // YYYY.MM.DD
    pub date: String,
    // 0 - 999
    pub round: String,
    // Last name, First name
    pub white: String,
    // Last name, First name
    pub black: String,
    // white won: 1-0
    // black won: 0-1
    // draw: 1/2-1/2
    // game in progress: *
    pub result: String,

    pub annotator: Option<String>,
    pub ply_count: Option<String>,
    pub time_control: Option<String>,
    pub time: Option<String>,
    pub termination: Option<String>,
    pub mode: Option<String>,
    pub fen: Option<String>,
    pub set_up: Option<String>,
}

impl PgnRosterRaw {
    pub fn build(&mut self) -> Result<PgnRoster, PgnTagPairParseError> {
        let mut roster = PgnRoster {
            event: event::PgnTagPairEvent::from_str(&self.event),
            site: site::PgnTagPairSite::from_str(&self.site)?,
            date: date::PgnTagPairDate::from_str(&self.date)?,
            round: round::PgnTagPairRound::from_str(&self.round)?,
            white: white::PgnTagPairWhite::from_str(&self.white)?,
            black: black::PgnTagPairBlack::from_str(&self.black)?,
            result: result::PgnTagPairResult::from_str(&self.result)?,
            fen: None,

        };
        if let Some(_annotator) = self.annotator.take() {
            // roster.annotator = Some(annotator);
        };
        if let Some(_ply_count) = self.ply_count.take() {
            // roster.ply_count = Some(ply_count);
        };
        if let Some(_time_control) = self.time_control.take() {
            // roster.time_control = Some(time_control);
        };
        if let Some(_time) = self.time.take() {
            // roster.time = Some(time);
        };
        if let Some(_termination) = self.termination.take() {
            // roster.termination = Some(termination);
        };
        if let Some(_mode) = self.mode.take() {
            // roster.mode = Some(mode);
        };
        if let Some(fen) = self.fen.take() {
            roster.fen = Some(fen::PgnTagPairFen::from_str(fen.as_str())?);
        };
        if let Some(_set_up) = self.set_up.take() {
            // roster.set_up = Some(set_up);
        };
        Ok(roster)
    }
}
