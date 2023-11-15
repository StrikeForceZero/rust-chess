use crate::notation::pgn::pgn_roster::PgnRoster;
use crate::notation::pgn::pgn_turn_data::PgnTurnData;

#[derive(Default, Debug)]
pub struct PgnDataPartial {
    pub roster: Option<PgnRoster>,
    pub turns: Option<Vec<PgnTurnData>>,
}
