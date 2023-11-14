use std::fmt::{Display, Formatter};
use crate::direction::castle_side::CastleSide;
use crate::notation::pgn::pgn_move_detail::PgnMoveDetail;

#[derive(Debug)]
pub enum PgnMove {
    Normal(PgnMoveDetail),
    Castle(PgnMoveDetail),
}

impl PgnMove {
    pub fn get_move_detail(&self) -> &PgnMoveDetail {
        match self {
            Self::Normal(detail) => detail,
            Self::Castle(detail) => detail,
        }
    }
}

impl Display for PgnMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PgnMove::Normal(detail) => write!(f, "{detail}"),
            PgnMove::Castle(detail) => {
                write!(f, "{}", CastleSide::from_pos(detail.to_pos).as_pgn_str())
            },
        }
    }
}
