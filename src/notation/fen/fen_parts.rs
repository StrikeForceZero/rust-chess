#[derive(Debug)]
pub struct FenParts<'a> {
    pub squares_str: &'a str,
    pub active_color_str: &'a str,
    pub castle_rights_str: &'a str,
    pub en_passant_str: &'a str,
    pub half_move_clock_str: &'a str,
    pub full_move_num_str: &'a str,
}
