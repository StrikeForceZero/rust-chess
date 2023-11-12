pub mod serialize;
pub mod deserialize;
pub use serialize::serialize;
pub use deserialize::deserialize;

use std::fmt::{Display, Formatter};

pub const FEN_STARTING_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const FEN_EMPTY: &str = "8/8/8/8/8/8/8/8 w - - 0 1";

const BOARD_TERMINATOR: &str = "/";

#[derive(Clone, Debug)]
pub enum Fen {
    Static(&'static str),
    Owned(String),
}

impl Fen {
    pub fn get_str(&self) -> &str {
        match self {
            Fen::Static(s) => s,
            Fen::Owned(s) => s.as_ref(),
        }
    }
}

impl PartialEq<Fen> for Fen {
    fn eq(&self, other: &Fen) -> bool {
        self.get_str() == other.get_str()
    }
}

impl PartialEq<String> for Fen {
    fn eq(&self, other: &String) -> bool {
        self.get_str() == other
    }
}

impl PartialEq<str> for Fen {
    fn eq(&self, other: &str) -> bool {
        self.get_str() == other
    }
}

impl Display for Fen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_str())
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;
    use crate::state::game_state::GameState;

    #[rstest]
    #[case(GameState::new(), FEN_STARTING_POS)]
    #[case(GameState::empty(), FEN_EMPTY)]
    pub fn fen_serialize(#[case] game_state: GameState, #[case] expected: &'static str) {
        assert_eq!(expected, serialize::serialize(&game_state).get_str())
    }
}
