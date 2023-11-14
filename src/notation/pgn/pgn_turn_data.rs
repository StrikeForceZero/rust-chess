use std::fmt::{Display, Formatter};
use crate::notation::pgn::pgn_move_detail::PgnMoveDetail;

#[derive(Debug)]
pub struct PgnTurnData {
    pub turn_number: usize,
    pub white: PgnMoveDetail,
    pub black: Option<PgnMoveDetail>,
    pub comment: Option<String>,
}

impl Display for PgnTurnData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let turn_number = self.turn_number;
        let white = self.white.to_string();
        let mut continuation: String = String::from("");
        let black = if let Some(black) = &self.black {
            if self.white.comment.is_some() {
                continuation = format!(" {turn_number}...");
            }
            format!(" {black}")
        } else {
            String::from("")
        };
        let comment = if let Some(comment) = &self.comment {
            format!(" ;{comment}\n")
        } else {
            String::from("")
        };
        write!(f, "{turn_number}. {white}{continuation}{black}{comment}")
    }
}
