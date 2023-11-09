use crate::fen::Fen;
use crate::pgn::Pgn;

#[derive(Clone)]
pub struct History {
    pub fen: Vec<Fen>,
    pub pgn: Vec<Pgn>,
}

impl History {
    pub const fn new() -> Self {
        Self {
            fen: Vec::new(),
            pgn: Vec::new(),
        }
    }
}
