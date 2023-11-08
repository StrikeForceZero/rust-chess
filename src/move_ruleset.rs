use crate::direction_amount::DirectionAmount;
use crate::direction_limit::DirectionLimit;

pub enum DirectionalRestriction {
    Amount(Vec<DirectionAmount>),
    Limit(Vec<DirectionLimit>),
}

#[derive(Default)]
pub struct MoveRuleset {
    pub can_capture: bool,
    pub only_from_starting_pos: bool,
    pub directional_restrictions: Option<DirectionalRestriction>,
}
