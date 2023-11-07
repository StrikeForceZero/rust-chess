use crate::board_file::BoardFile;
use crate::board_position::BoardPosition;
use crate::board_rank::BoardRank;

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::NorthEast => Direction::SouthWest,
            Direction::East => Direction::West,
            Direction::SouthEast => Direction::NorthWest,
            Direction::South => Direction::North,
            Direction::SouthWest => Direction::NorthEast,
            Direction::West => Direction::East,
            Direction::NorthWest => Direction::SouthEast,
        }
    }
    pub fn get_next_pos(&self, from: BoardPosition) -> Option<BoardPosition> {
        let BoardPosition(file, rank) = from;
        let next_pos: (Option<BoardFile>, Option<BoardRank>) = match self {
            Direction::North =>
                (Some(file), rank.next()),
            Direction::NorthEast =>
                (file.next(), rank.next()),
            Direction::East =>
                (file.next(), Some(rank)),
            Direction::SouthEast =>
                (file.next(), rank.prev()),
            Direction::South =>
                (Some(file), rank.prev()),
            Direction::SouthWest =>
                (file.prev(), rank.prev()),
            Direction::West =>
                (file.prev(), Some(rank)),
            Direction::NorthWest =>
                (file.prev(), rank.next()),
        };
        match next_pos {
            (Some(file), Some(rank)) => Some(BoardPosition(file, rank)),
            (None, _) | (_, None) => None,
        }
    }

    pub fn get_next_pos_n(&self, mut from: BoardPosition, n: u8) -> Option<BoardPosition> {
        if n == 0 {
            // TODO: warn for potentially wrong, yet valid usage
            panic!("n must be greater than 0");
        }
        for _ in 0..n {
            from = self.get_next_pos(from)?;
        }
        Some(from)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum SimpleDirection {
    North,
    East,
    South,
    West,
}

impl SimpleDirection {
    pub fn as_direction(&self) -> Direction {
        match self {
            SimpleDirection::North => Direction::North,
            SimpleDirection::East => Direction::East,
            SimpleDirection::South => Direction::South,
            SimpleDirection::West => Direction::West,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum DiagonalDirection {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

impl DiagonalDirection {
    pub fn as_direction(&self) -> Direction {
        match self {
            DiagonalDirection::NorthEast => Direction::NorthEast,
            DiagonalDirection::SouthEast => Direction::SouthEast,
            DiagonalDirection::SouthWest => Direction::SouthWest,
            DiagonalDirection::NorthWest => Direction::NorthWest,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::direction::Direction;
    use crate::position::{A1, A2};

    #[test]
    fn direction() {
        assert_eq!(Direction::East.get_next_pos_n(A1, 1).unwrap(), A2);
        assert_eq!(Direction::East.get_next_pos_n(A1, 7).unwrap(), A2);
    }
}
