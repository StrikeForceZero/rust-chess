use tracing::{debug, instrument};

#[derive(PartialEq, Debug)]
pub enum After {
    Space,
    NewLine,
    TagPairName,
    TagPairEnd,
    TurnBegin,
    MovingTo,
    Promotion,
    PromotionEnd,
    CheckIndicator,
    CheckMateIndicator,
    Annotation,
    AnnotationEnd,
    MoveQuality,
    Nag,
    TurnContinuation,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum WhiteSpace {
    NewLine,
    Space,
}


#[derive(Debug, PartialEq)]
pub enum Token {
    TagPairStart(char),
    TagPairName(String),
    TagPairValue(String),
    TagPairEnd(char),
    TurnBegin(String),
    PieceMoving(char),
    MovingFrom(char),
    CaptureIndicator(char),
    MovingTo(String),
    PromotionStart(char),
    Promotion(char),
    PromotionEnd(char),
    CheckIndicator(char),
    CheckMateIndicator(char),
    AnnotationStart(char),
    Annotation(String),
    AnnotationEnd(char),
    MoveQuality(String),
    Nag(String),
    TurnContinuation(String),
    GameTermination(String),
    Unknown(String),
    WhiteSpace(char, WhiteSpace, After),
    MaybeTurnBeginOrContinuationOrMovingFromOrGameTermination(String),
}

pub fn ws_new_line(value: char, after: After) -> Token {
    Token::WhiteSpace(value, WhiteSpace::NewLine, after)
}

pub fn ws_space(value: char, after: After) -> Token {
    Token::WhiteSpace(value, WhiteSpace::Space, after)
}
