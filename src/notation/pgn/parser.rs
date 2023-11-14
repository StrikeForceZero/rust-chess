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
