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
    use rstest::rstest;
    use crate::board_position::BoardPosition;
    use crate::direction::Direction;
    use crate::position::*;

    #[rstest]
    #[case(Direction::East, A1, 1, Some(B1))]
    #[case(Direction::East, A1, 2, Some(C1))]
    #[case(Direction::East, A1, 3, Some(D1))]
    #[case(Direction::East, A1, 4, Some(E1))]
    #[case(Direction::East, A1, 5, Some(F1))]
    #[case(Direction::East, A1, 6, Some(G1))]
    #[case(Direction::East, A1, 7, Some(H1))]
    #[case(Direction::East, A1, 8, None)]
    fn direction_get_next_pos_n_east(
        #[case] direction: Direction,
        #[case] from: BoardPosition,
        #[case] n: u8,
        #[case] expected: Option<BoardPosition>
    ) {
        assert_eq!(expected, direction.get_next_pos_n(from, n));
    }

    #[rstest]
    #[case(Direction::North, A1, 1, Some(A2))]
    #[case(Direction::North, A1, 2, Some(A3))]
    #[case(Direction::North, A1, 3, Some(A4))]
    #[case(Direction::North, A1, 4, Some(A5))]
    #[case(Direction::North, A1, 5, Some(A6))]
    #[case(Direction::North, A1, 6, Some(A7))]
    #[case(Direction::North, A1, 7, Some(A8))]
    #[case(Direction::North, A1, 8, None)]
    fn direction_get_next_pos_n_north(
        #[case] direction: Direction,
        #[case] from: BoardPosition,
        #[case] n: u8,
        #[case] expected: Option<BoardPosition>
    ) {
        assert_eq!(expected, direction.get_next_pos_n(from, n));
    }

    #[rstest]
    #[case(Direction::NorthEast, A1, 1, Some(B2))]
    #[case(Direction::NorthEast, A1, 2, Some(C3))]
    #[case(Direction::NorthEast, A1, 3, Some(D4))]
    #[case(Direction::NorthEast, A1, 4, Some(E5))]
    #[case(Direction::NorthEast, A1, 5, Some(F6))]
    #[case(Direction::NorthEast, A1, 6, Some(G7))]
    #[case(Direction::NorthEast, A1, 7, Some(H8))]
    #[case(Direction::NorthEast, A1, 8, None)]
    fn direction_get_next_pos_n_north_east(
        #[case] direction: Direction,
        #[case] from: BoardPosition,
        #[case] n: u8,
        #[case] expected: Option<BoardPosition>
    ) {
        assert_eq!(expected, direction.get_next_pos_n(from, n));
    }

    #[rstest]
    #[case(Direction::West, H1, 1, Some(G1))]
    #[case(Direction::West, H1, 2, Some(F1))]
    #[case(Direction::West, H1, 3, Some(E1))]
    #[case(Direction::West, H1, 4, Some(D1))]
    #[case(Direction::West, H1, 5, Some(C1))]
    #[case(Direction::West, H1, 6, Some(B1))]
    #[case(Direction::West, H1, 7, Some(A1))]
    #[case(Direction::West, H1, 8, None)]
    fn direction_get_next_pos_n_west(
        #[case] direction: Direction,
        #[case] from: BoardPosition,
        #[case] n: u8,
        #[case] expected: Option<BoardPosition>
    ) {
        assert_eq!(expected, direction.get_next_pos_n(from, n));
    }

    #[rstest]
    #[case(Direction::South, H8, 1, Some(H7))]
    #[case(Direction::South, H8, 2, Some(H6))]
    #[case(Direction::South, H8, 3, Some(H5))]
    #[case(Direction::South, H8, 4, Some(H4))]
    #[case(Direction::South, H8, 5, Some(H3))]
    #[case(Direction::South, H8, 6, Some(H2))]
    #[case(Direction::South, H8, 7, Some(H1))]
    #[case(Direction::South, H8, 8, None)]
    fn direction_get_next_pos_n_south(
        #[case] direction: Direction,
        #[case] from: BoardPosition,
        #[case] n: u8,
        #[case] expected: Option<BoardPosition>
    ) {
        assert_eq!(expected, direction.get_next_pos_n(from, n));
    }
}
