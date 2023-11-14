use tracing::{debug, warn};
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
            Self::Normal(detail_builder) => detail_builder,
            Self::Castle(detail_builder) => detail_builder,
        }
    }
    pub fn get_move_detail_mut(&mut self) -> &mut PgnMoveDetailBuilder {
        match self {
            Self::Normal(detail_builder) => detail_builder,
            Self::Castle(detail_builder) => detail_builder,
        }
    }
    pub fn build(self) -> Result<PgnMove, String> {
        Ok(match self {
            Self::Normal(detail_builder) => {
                let detail = detail_builder.build()?;
                if detail.looks_like_castle() {
                    return Err(format!("pgn move originally stored as normal, looks like a castle! {detail}"));
                }
                PgnMove::Normal(detail)
            }
            Self::Castle(detail_builder) => {
                let detail = detail_builder.build()?;
                if !detail.looks_like_castle() {
                    return Err(format!("pgn move originally stored as castle, does not look like a castle! {detail}"));
                }
                PgnMove::Castle(detail)
            },
        })
    }
    pub fn build_implicit(self) -> Result<PgnMove, &'static str> {
        Ok(match self {
            Self::Normal(detail_builder) => {
                let detail = detail_builder.build()?;
                if detail.looks_like_castle() {
                    debug!("upgrading pgn move to castle: {detail}");
                    PgnMove::Castle(detail)
                } else {
                    PgnMove::Normal(detail)
                }
            }
            Self::Castle(detail_builder) => {
                let detail = detail_builder.build()?;
                if detail.looks_like_castle() {
                    PgnMove::Castle(detail)
                } else {
                    warn!("pgn move originally stored as castle, does not look like a castle! {detail}");
                    PgnMove::Normal(detail)
                }
            },
        })
    }
}

impl Default for PgnMoveBuilder {
    fn default() -> Self {
        Self::Normal(PgnMoveDetailBuilder::default())
    }
}
