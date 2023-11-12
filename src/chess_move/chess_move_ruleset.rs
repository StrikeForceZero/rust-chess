use crate::board::board_rank::BoardRank;
use crate::direction::castle_side::CastleSide;
use crate::direction::direction::{DiagonalDirection, Direction, SimpleDirection};
use crate::direction::direction_amount::DirectionAmount;
use crate::direction::facing_direction::FacingDirection;
use crate::piece::promotion_piece::PromotionPiece;

#[derive(Copy, Clone, Debug)]
pub enum DirectionRestriction {
    LMove(DirectionAmount, DirectionAmount),
    Amount(DirectionAmount),
    Limit(DirectionAmount),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CaptureOnlyType {
    Normal,
    EnPassant,
}

#[derive(Default, Copy, Clone, Debug)]
pub enum ChessMoveRulesetType {
    #[default]
    Normal,
    WhenCapturingOnly(CaptureOnlyType),
    Castle,
}

#[derive(Default, Debug)]
pub struct ChessMoveRuleset {
    pub can_capture: bool,
    pub is_jump: bool,
    pub only_from_starting_pos: bool,
    pub move_type: ChessMoveRulesetType,
    pub directional_restriction: Option<DirectionRestriction>,
    pub capture_offset: Option<DirectionAmount>,
    pub promote_to: Option<PromotionPiece>,
    pub requires_rank: Option<BoardRank>,
}

impl ChessMoveRuleset {
    pub const fn default() -> Self {
        Self {
            can_capture: false,
            is_jump: false,
            only_from_starting_pos: false,
            move_type: ChessMoveRulesetType::Normal,
            directional_restriction: None,
            capture_offset: None,
            promote_to: None,
            requires_rank: None,
        }
    }
    pub const fn single(direction: Direction, can_capture: bool) -> Self {
        Self {
            can_capture,
            directional_restriction: Some(DirectionRestriction::Amount(DirectionAmount(
                direction, 1,
            ))),
            ..Self::default()
        }
    }
    pub const fn capture_only_move(
        direction: Direction,
        capture_only_type: CaptureOnlyType,
    ) -> Self {
        Self {
            can_capture: true,
            move_type: ChessMoveRulesetType::WhenCapturingOnly(capture_only_type),
            directional_restriction: Some(DirectionRestriction::Amount(DirectionAmount(
                direction, 1,
            ))),
            ..Self::default()
        }
    }
    pub const fn double(direction: Direction) -> Self {
        Self {
            only_from_starting_pos: true,
            directional_restriction: Some(DirectionRestriction::Amount(DirectionAmount(
                direction, 2,
            ))),
            ..Self::default()
        }
    }
    pub const fn full(direction: Direction) -> Self {
        Self {
            directional_restriction: Some(DirectionRestriction::Limit(DirectionAmount(
                direction, 7,
            ))),
            can_capture: true,
            ..Self::default()
        }
    }
    pub const fn l_jump(
        first_direction_amount: DirectionAmount,
        second_direction_amount: DirectionAmount,
    ) -> Self {
        Self {
            is_jump: true,
            directional_restriction: Some(DirectionRestriction::LMove(
                first_direction_amount,
                second_direction_amount,
            )),
            can_capture: true,
            ..Self::default()
        }
    }
    pub const fn either_l_jump(simple_direction: SimpleDirection) -> [Self; 2] {
        let (perp_a, perp_b) = simple_direction.as_perpendicular_simple_direction_tuple();
        [
            Self::l_jump(
                DirectionAmount(simple_direction.as_direction(), 2),
                DirectionAmount(perp_a.as_direction(), 1),
            ),
            Self::l_jump(
                DirectionAmount(simple_direction.as_direction(), 2),
                DirectionAmount(perp_b.as_direction(), 1),
            ),
        ]
    }
    pub const fn en_passant(diagonal_direction: DiagonalDirection) -> Self {
        let mut move_ruleset = Self::single(diagonal_direction.as_direction(), true);
        move_ruleset.move_type =
            ChessMoveRulesetType::WhenCapturingOnly(CaptureOnlyType::EnPassant);
        move_ruleset.capture_offset = Some(DirectionAmount(
            diagonal_direction
                .as_facing_direction()
                .as_simple_direction()
                .as_direction(),
            1,
        ));
        move_ruleset
    }
    pub const fn castle(castle_side: CastleSide) -> Self {
        let mut move_ruleset = Self::double(castle_side.as_simple_direction().as_direction());
        move_ruleset.move_type = ChessMoveRulesetType::Castle;
        move_ruleset
    }

    pub const fn promotion(facing_direction: FacingDirection, piece: PromotionPiece) -> Self {
        let mut ruleset = Self::forward(facing_direction);
        ruleset.promote_to = Some(piece);
        ruleset.requires_rank = Some(match facing_direction {
            FacingDirection::North => BoardRank::Seven,
            FacingDirection::South => BoardRank::Two,
        });
        ruleset
    }

    pub const fn forward(facing_direction: FacingDirection) -> Self {
        Self::single(facing_direction.as_simple_direction().as_direction(), false)
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
            Self::full(Direction::East),
            Self::full(Direction::South),
            Self::full(Direction::West),
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
    pub const fn any_l_jump() -> [Self; 8] {
        let [a, b] = ChessMoveRuleset::either_l_jump(SimpleDirection::North);
        let [c, d] = ChessMoveRuleset::either_l_jump(SimpleDirection::East);
        let [e, f] = ChessMoveRuleset::either_l_jump(SimpleDirection::South);
        let [g, h] = ChessMoveRuleset::either_l_jump(SimpleDirection::West);
        [a, b, c, d, e, f, g, h]
    }
    pub const fn any_en_passant(facing_direction: FacingDirection) -> [Self; 2] {
        let (left, right) = facing_direction.split();
        [Self::en_passant(left), Self::en_passant(right)]
    }
    pub const fn any_castle() -> [Self; 2] {
        [
            Self::castle(CastleSide::King),
            Self::castle(CastleSide::Queen),
        ]
    }
}
