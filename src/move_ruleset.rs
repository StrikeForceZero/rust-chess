use crate::castle_side::CastleSide;
use crate::direction::{DiagonalDirection, Direction};
use crate::direction_amount::DirectionAmount;
use crate::facing_direction::FacingDirection;


pub enum DirectionRestriction {
    LMove(DirectionAmount, DirectionAmount),
    Amount(DirectionAmount),
    Limit(DirectionAmount),
}

pub enum CaptureOnlyType {
    Normal,
    EnPassant,
}

#[derive(Default)]
pub enum MoveType {
    #[default]
    Normal,
    WhenCapturingOnly(CaptureOnlyType),
    Castle,
}

#[derive(Default)]
pub struct MoveRuleset {
    pub can_capture: bool,
    pub is_jump: bool,
    pub only_from_starting_pos: bool,
    pub move_type: MoveType,
    pub directional_restriction: Option<DirectionRestriction>,
    pub capture_offset: Option<DirectionAmount>,
}

impl MoveRuleset {
    pub const fn default() -> Self {
        Self {
            can_capture: false,
            is_jump: false,
            only_from_starting_pos: false,
            move_type: MoveType::Normal,
            directional_restriction: None,
            capture_offset: None,
        }
    }
    pub const fn single(direction: Direction, can_capture: bool) -> Self {
        Self {
            can_capture,
            directional_restriction: Some(DirectionRestriction::Amount(DirectionAmount(direction, 1))),
            ..Self::default()
        }
    }
    pub const fn capture_only_move(direction: Direction, capture_only_type: CaptureOnlyType) -> Self {
        Self {
            can_capture: true,
            move_type: MoveType::WhenCapturingOnly(capture_only_type),
            directional_restriction: Some(DirectionRestriction::Amount(DirectionAmount(direction, 1))),
            ..Self::default()
        }
    }
    pub const fn double(direction: Direction) -> Self {
        Self {
            only_from_starting_pos: true,
            directional_restriction: Some(DirectionRestriction::Amount(DirectionAmount(direction, 2))),
            ..Self::default()
        }
    }
    pub const fn full(direction: Direction) -> Self {
        Self {
            directional_restriction: Some(DirectionRestriction::Limit(DirectionAmount(direction, 7))),
            ..Self::default()
        }
    }
    pub const fn en_passant(diagonal_direction: DiagonalDirection) -> Self {
        let mut move_ruleset = Self::single(diagonal_direction.as_direction(), true);
        move_ruleset.move_type = MoveType::WhenCapturingOnly(CaptureOnlyType::EnPassant);
        move_ruleset.capture_offset = Some(DirectionAmount(diagonal_direction.as_facing_direction().as_simple_direction().as_direction(), 1));
        move_ruleset
    }
    pub const fn castle(castle_side: CastleSide) -> Self {
        let mut move_ruleset = Self::double(castle_side.as_simple_direction().as_direction());
        move_ruleset.move_type = MoveType::Castle;
        move_ruleset
    }
    pub const fn any_single() -> [Self; 8] {
        [
            Self::single(Direction::North, true),
            Self::single(Direction::NorthEast, true),
            Self::single(Direction::East, true),
            Self::single(Direction::SouthEast, true),
            Self::single(Direction::South, true),
            Self::single(Direction::SouthWest, true),
            Self::single(Direction::West, true),
            Self::single(Direction::NorthWest, true),
        ]
    }
    pub const fn any_facing_diagonal_capture(facing_direction: FacingDirection) -> [Self; 2] {
        let (left, right) = facing_direction.split();
        [
            Self::capture_only_move(left.as_direction(), CaptureOnlyType::Normal),
            Self::capture_only_move(right.as_direction(), CaptureOnlyType::Normal),
        ]
    }
    pub const fn any_full_straight() -> [Self; 4] {
        [
            Self::full(Direction::North),
            Self::full(Direction::South),
            Self::full(Direction::South),
            Self::full(Direction::North),
        ]
    }
    pub const fn any_full_diagonal() -> [Self; 4] {
        [
            Self::full(Direction::NorthEast),
            Self::full(Direction::SouthEast),
            Self::full(Direction::SouthWest),
            Self::full(Direction::NorthWest),
        ]
    }
    pub const fn any_full() -> [Self; 8] {
        [
            Self::full(Direction::North),
            Self::full(Direction::NorthEast),
            Self::full(Direction::East),
            Self::full(Direction::SouthEast),
            Self::full(Direction::South),
            Self::full(Direction::SouthWest),
            Self::full(Direction::West),
            Self::full(Direction::NorthWest),
        ]
    }
    pub const fn any_en_passant(facing_direction: FacingDirection) -> [Self; 2] {
        let (left, right) = facing_direction.split();
        [
            Self::en_passant(left),
            Self::en_passant(right),
        ]
    }
    pub const fn any_castle() -> [Self; 2] {
        [
            Self::castle(CastleSide::Queen),
            Self::castle(CastleSide::King)
        ]
    }
}
