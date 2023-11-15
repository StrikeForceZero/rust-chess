use crate::notation::pgn::tag_pairs::black::PgnTagPairBlack;
use crate::notation::pgn::tag_pairs::date::PgnTagPairDate;
use crate::notation::pgn::tag_pairs::event::PgnTagPairEvent;
use crate::notation::pgn::tag_pairs::fen::PgnTagPairFen;
use crate::notation::pgn::tag_pairs::result::PgnTagPairResult;
use crate::notation::pgn::tag_pairs::round::PgnTagPairRound;
use crate::notation::pgn::tag_pairs::site::PgnTagPairSite;
use crate::notation::pgn::tag_pairs::white::PgnTagPairWhite;

#[derive(Debug)]
pub struct PgnRoster {
    pub event: PgnTagPairEvent,
    // City, Region ThreeLetterCountryCode
    pub site: PgnTagPairSite,
    // YYYY.MM.DD
    pub date: PgnTagPairDate,
    // 0 - 999
    pub round: PgnTagPairRound,
    // Last name, First name
    pub white: PgnTagPairWhite,
    // Last name, First name
    pub black: PgnTagPairBlack,
    // white won: 1-0
    // black won: 0-1
    // draw: 1/2-1/2
    // game in progress: *
    pub result: PgnTagPairResult,

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
