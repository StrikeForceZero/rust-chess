use crate::castle_rights::CastleRights;

#[derive(Clone)]
pub struct ColorCastleRights {
    white: CastleRights,
    black: CastleRights,
}
