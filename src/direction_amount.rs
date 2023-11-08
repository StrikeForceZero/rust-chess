use crate::direction::Direction;

pub type Amount = u8;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DirectionAmount(pub Direction, pub Amount);

impl DirectionAmount {
    pub fn from(direction: Direction, limit: Amount) -> Self {
        Self(direction, limit)
    }
    pub fn direction(&self) -> Direction {
        self.0
    }
    pub fn limit(&self) -> Amount {
        self.1
    }
}
