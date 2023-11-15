use crate::notation::pgn::pgn_roster::PgnRoster;
use crate::notation::pgn::pgn_turn_data::PgnTurnData;

#[derive(Debug)]
pub struct PgnData {
    pub roster: PgnRoster,
    pub turns: Vec<PgnTurnData>,
}
