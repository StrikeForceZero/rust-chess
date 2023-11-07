pub enum Color {
    White,
    Black,
}

impl Color {
    fn inverse(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
