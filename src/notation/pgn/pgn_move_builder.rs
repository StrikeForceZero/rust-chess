use crate::notation::pgn::pgn_move::PgnMove;
use crate::notation::pgn::pgn_move_detail_builder::PgnMoveDetailBuilder;

#[derive(Debug)]
pub enum PgnMoveBuilder {
    Normal(PgnMoveDetailBuilder),
    Castle(PgnMoveDetailBuilder),
}

impl PgnMoveBuilder {
    pub fn get_move_detail(&self) -> &PgnMoveDetailBuilder {
        match self {
            Self::Normal(detail) => detail,
            Self::Castle(detail) => detail,
        }
    }
    pub fn get_move_detail_mut(&mut self) -> &mut PgnMoveDetailBuilder {
        match self {
            Self::Normal(detail) => detail,
            Self::Castle(detail) => detail,
        }
    }
    pub fn build(self) -> Result<PgnMove, &'static str> {
        Ok(match self {
            Self::Normal(detail) => PgnMove::Normal(detail.build()?),
            Self::Castle(detail) => PgnMove::Castle(detail.build()?),
        })
    }
}

impl Default for PgnMoveBuilder {
    fn default() -> Self {
        Self::Normal(PgnMoveDetailBuilder::default())
    }
}
