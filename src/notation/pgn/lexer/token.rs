use tracing::{debug, instrument};

#[derive(PartialEq, Debug)]
pub enum WhiteSpaceToken {
    AfterNewLine,
    AfterTagPairName,
    AfterTagPairEnd,
    AfterTurnBegin,
    AfterMovingTo,
    AfterPromotion,
    AfterPromotionEnd,
    AfterCheckIndicator,
    AfterCheckMateIndicator,
    AfterAnnotation,
    AfterAnnotationEnd,
    AfterMoveQuality,
    AfterNag,
    AfterTurnContinuation,
    AfterUnknown,
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
    CaptureIndicator,
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
    NewLine(WhiteSpaceToken),
    WhiteSpace(WhiteSpaceToken),
    MaybeTurnBeginOrContinuationOrMovingFromOrGameTermination(String),
}
