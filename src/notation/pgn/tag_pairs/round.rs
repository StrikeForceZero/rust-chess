use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::notation::pgn::tag_pairs::PgnTagPairParseError;

#[derive(Debug, PartialEq)]
pub struct PgnTagPairRound(pub usize);

impl PgnTagPairRound {
    pub fn as_str(&self) -> String {
        self.0.to_string()
    }

    pub fn from_str(s: &str) -> Result<Self, PgnTagPairParseError> {
        Ok(match usize::from_str(s) {
            Ok(n) => Self(n),
            Err(e) => return Err(Self::create_parsing_error(s)),
        })
    }
}

// for easier copy paste
type ThisPgnTagPair = PgnTagPairRound;
const NAME: &str = "Round";
super::impl_named_tag_pair_for!(ThisPgnTagPair, NAME);

impl Display for PgnTagPairRound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
