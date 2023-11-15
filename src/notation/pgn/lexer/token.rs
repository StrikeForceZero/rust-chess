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
    AfterAnnotationEnd,
    AfterMoveQuality,
    AfterNag,
    AfterTurnContinuation,
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
    NewLine,
    WhiteSpace(WhiteSpaceToken),
    MaybeTurnBeginOrContinuationOrMovingFromOrGameTermination(String),
}

impl PartialEq<&Token> for Token {
    fn eq(&self, other: &&Token) -> bool {
        *self == *other
    }
}
