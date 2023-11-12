#[derive(Clone, Debug)]
pub struct MoveCounter {
    pub half_move: u16,
    pub full_move: u16,
}

impl MoveCounter {
    pub const fn new() -> Self {
        Self {
            half_move: 0,
            full_move: 1,
        }
    }
}
