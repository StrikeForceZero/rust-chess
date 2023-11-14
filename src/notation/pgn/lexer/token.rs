#[derive(PartialEq, Debug)]
pub enum Token {
    TagPairStart(char),
    TagPairName(String),
    TagPairValue(String),
    TagPairEnd(char),
    TurnBegin(String),
    PieceMoving(char),
    MovingFrom(char),
    Capture,
    MovingTo(String),
    CheckOrMateIndicator(char),
    PromotionStart(char),
    Promotion(char),
    PromotionEnd(char),
    AnnotationStart(char),
    Annotation(String),
    AnnotationEnd(char),
    MoveQuality(String),
    Nag(String),
    TurnContinuation(String),
    GameTermination(String),
    Unknown(String),
    NewLine,
}

impl Token {
    pub fn is_tag_pair_start() -> bool {
        todo!("implement")
    }
    pub fn is_tag_pair_name() -> bool {
        todo!("implement")
    }
    pub fn is_tag_pair_value() -> bool {
        todo!("implement")
    }
    pub fn is_tag_pair_end() -> bool {
        todo!("implement")
    }
    pub fn is_turn_begin() -> bool {
        todo!("implement")
    }
    pub fn is_piece_moving() -> bool {
        todo!("implement")
    }
    pub fn is_moving_from() -> bool {
        todo!("implement")
    }
    pub fn is_capture() -> bool {
        todo!("implement")
    }
    pub fn is_moving_to() -> bool {
        todo!("implement")
    }
    pub fn is_check_or_mate_indicator() -> bool {
        todo!("implement")
    }
    pub fn is_promotion_start() -> bool {
        todo!("implement")
    }
    pub fn is_promotion() -> bool {
        todo!("implement")
    }
    pub fn is_promotion_end() -> bool {
        todo!("implement")
    }
    pub fn is_annotation_start() -> bool {
        todo!("implement")
    }
    pub fn is_annotation() -> bool {
        todo!("implement")
    }
    pub fn is_annotation_end() -> bool {
        todo!("implement")
    }
    pub fn is_move_quality() -> bool {
        todo!("implement")
    }
    pub fn is_nag() -> bool {
        todo!("implement")
    }
    pub fn is_turn_continuation() -> bool {
        todo!("implement")
    }
    pub fn is_game_termination() -> bool {
        todo!("implement")
    }
    pub fn is_unknown() -> bool {
        todo!("implement")
    }
    pub fn is_new_line() -> bool {
        todo!("implement")
    }
}
