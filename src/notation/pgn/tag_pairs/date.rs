use std::fmt::{Display, Formatter};
use crate::notation::pgn::tag_pairs::PgnTagPairParseError;

#[derive(Debug, PartialEq)]
pub struct PgnTagPairDate(pub String);

impl PgnTagPairDate {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn from_str(s: &str) -> Result<Self, PgnTagPairParseError> {
        // TODO: actually parse the date
        Ok(Self(s.to_string()))
    }
}

// for easier copy paste
type ThisPgnTagPair = PgnTagPairDate;
const NAME: &str = "Date";
super::impl_named_tag_pair_for!(ThisPgnTagPair, NAME);

impl Display for PgnTagPairDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
