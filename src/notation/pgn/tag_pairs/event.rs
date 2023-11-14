use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct PgnTagPairEvent(pub String);

impl PgnTagPairEvent {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn from_str(s: &str) -> Self {
        Self(s.to_string())
    }
}

// for easier copy paste
type ThisPgnTagPair = PgnTagPairEvent;
const NAME: &str = "Event";
super::impl_named_tag_pair_for!(ThisPgnTagPair, NAME);

impl Display for PgnTagPairEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
