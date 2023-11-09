use crate::fen::Fen;
use crate::pgn::Pgn;

#[derive(Clone)]
pub struct History {
    pub fen: Vec<Fen>,
    pub pgn: Vec<Pgn>,
}
