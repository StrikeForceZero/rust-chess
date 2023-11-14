use crate::notation::pgn::pgn_roster_raw_partial::PgnRosterRawPartial;
use crate::notation::pgn::pgn_turn_data_raw::PgnTurnDataRaw;
use crate::notation::pgn::pgn_turn_data_raw_partial::PgnTurnDataRawPartial;

#[derive(Default, Debug)]
pub struct Parser {
    roster_raw: Vec<String>,
    roster: Option<PgnRosterRawPartial>,
    current_turn: Option<PgnTurnDataRawPartial>,
    raw_turns: Vec<PgnTurnDataRawPartial>,
    turns: Vec<PgnTurnDataRaw>,
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
