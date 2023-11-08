use crate::direction::Direction;

pub type Limit = u8;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DirectionLimit(pub Direction, pub Limit);

impl DirectionLimit {
    pub fn from(direction: Direction, limit: Limit) -> Self {
        DirectionLimit(direction, limit)
    }
    pub fn direction(&self) -> Direction {
        self.0
    }
    pub fn limit(&self) -> Limit {
        self.1
    }
}
