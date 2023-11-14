

type Word = String;
type Line = String;
type Pos = usize;

#[derive(Debug)]
pub struct LineWordPosTuple(pub Line, pub Word, pub Pos);
