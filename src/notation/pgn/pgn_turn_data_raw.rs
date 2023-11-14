#[derive(Debug)]
pub struct PgnTurnDataRaw {
    pub turn_number: String,
    pub white: String,
    pub white_comment: Option<String>,
    pub turn_number_continuation: Option<String>,
    pub black: Option<String>,
    pub black_comment: Option<String>,
    pub comment: Option<String>,
}
