use crate::direction::Direction;

type Amount = u8;

#[derive(Copy, Clone, Debug)]
pub struct DirectionAmount(pub Direction, pub Amount);

impl DirectionAmount {
    pub const fn direction(&self) -> Direction {
        self.0
    }
    pub const fn amount(&self) -> Amount {
        self.1
    }
}
