use std::fmt::{Display, Formatter};
use crate::notation::fen::Fen;
use crate::notation::pgn::tag_pairs::PgnTagPairParseError;

#[derive(Debug, PartialEq)]
pub struct PgnTagPairFen(pub Fen);

impl PgnTagPairFen {
    pub fn as_str(&self) -> &str {
        &self.0.get_str()
    }

    pub fn from_str(s: &str) -> Result<Self, PgnTagPairParseError> {
        // TODO: validate?
        Ok(Self(Fen::Owned(s.to_string())))
    }
}

// for easier copy paste
type ThisPgnTagPair = PgnTagPairFen;
const NAME: &str = "FEN";
super::impl_named_tag_pair_for!(ThisPgnTagPair, NAME);

impl Display for PgnTagPairFen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
