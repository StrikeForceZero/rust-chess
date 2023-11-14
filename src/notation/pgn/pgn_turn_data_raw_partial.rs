#[derive(Debug, Default, PartialEq)]
pub struct PgnTurnDataRawPartial {
    pub turn_number: Option<String>,
    pub white: Option<String>,
    pub white_comment: Option<String>,
    pub turn_number_continuation: Option<String>,
    pub black: Option<String>,
    pub black_comment: Option<String>,
    pub comment: Option<String>,
}
