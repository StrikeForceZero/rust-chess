use crate::fen::{Fen, FEN_STARTING_POS};
use crate::pgn::Pgn;

#[derive(Clone)]
pub struct History {
    pub fen: Vec<Fen>,
    pub pgn: Vec<Pgn>,
}

impl History {
    pub const fn empty() -> Self {
        Self {
            fen: Vec::new(),
            pgn: Vec::new(),
        }
    }
    pub const fn new() -> Self {
        Self {
            fen: Vec::from([Fen::Static(FEN_STARTING_POS)]),
            pgn: Vec::new(),
        }
    }
}
