use std::fmt::{Display, Formatter};
use itertools::Itertools;
use crate::notation::pgn::tag_pairs::PgnTagPairParseError;

#[derive(Debug, PartialEq)]
pub struct PgnTagPairSite {
    pub city: String,
    pub region: String,
    pub country_code: String,
}

impl PgnTagPairSite {
    pub fn as_str(&self) -> String {
        format!("{}, {} {}", self.city, self.region, self.country_code)
    }

    pub fn from_str(s: &str) -> Result<Self, PgnTagPairParseError> {
        let parts = s.split_terminator(',').collect_vec();
        if parts.len() != 2 {
            return Err(Self::create_parsing_error(s));
        }
        let (city, rest) = (parts[0].to_string(), parts[1]);
        let mut region_country_parts = rest.split_terminator(' ').collect_vec();
        if region_country_parts.len() < 2 {
            return Err(Self::create_parsing_error(s));
        }
        let country_code = region_country_parts.pop().expect("impossible").to_string();
        let region = region_country_parts.join(" ").trim().to_string();
        Ok(Self {
            city,
            region,
            country_code,
        })
    }
}

// for easier copy paste
type ThisPgnTagPair = PgnTagPairSite;
const NAME: &str = "Site";
super::impl_named_tag_pair_for!(ThisPgnTagPair, NAME);

impl Display for PgnTagPairSite {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
