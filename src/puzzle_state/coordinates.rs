//! Coordinates for sliding puzzle board.

/// Struct for holding coordinates on puzzle board.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct BoardCoordinates<const PUZZLE_SIZE: usize> {
    row: u8,
    column: u8,
}

impl<const PUZZLE_SIZE: usize> BoardCoordinates<PUZZLE_SIZE> {
    /**
     * Creates new puzzle board coordinates.
     *
     * # Panics
     *
     * If `row` or `column` are greater or equal `PUZZLE_SIZE`.
     */
    pub fn new(row: u8, column: u8) -> Self {
        assert!(
            (row as usize) < PUZZLE_SIZE && (column as usize) < PUZZLE_SIZE,
            "Coordinates not on puzzle board: ({row}, {column}) for puzzle size: {PUZZLE_SIZE}"
        );

        BoardCoordinates { row, column }
    }

    /// Returns coordinates as `(row, column)` tuple.
    pub fn as_tuple(&self) -> (u8, u8) {
        (self.row, self.column)
    }

    /// Calculates distance between two [Coord2D] instances.
    pub fn manhattan_distance(&self, other: &Self) -> u8 {
        self.row.abs_diff(other.row) + self.column.abs_diff(other.column)
    }

    /// Calculates coordinates manhattan distance from correct blank position.
    pub fn blank_manhattan_distance(&self) -> u8 {
        let correct_blank_position = Self::new((PUZZLE_SIZE - 1) as u8, (PUZZLE_SIZE - 1) as u8);

        self.manhattan_distance(&correct_blank_position)
    }

    /// Checks if coordinates are at upper game board edge.
    pub fn at_upper_edge(&self) -> bool {
        self.row == 0
    }

    /// Checks if coordinates are at left game board edge.
    pub fn at_left_edge(&self) -> bool {
        self.column == 0
    }

    /// Checks if coordinates are at right game board edge.
    pub fn at_right_edge(&self) -> bool {
        self.column == (PUZZLE_SIZE - 1) as u8
    }

    /// Checks if coordinates are at bottom game board edge.
    pub fn at_bottom_edge(&self) -> bool {
        self.row == (PUZZLE_SIZE - 1) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE_SIZE: usize = 4;

    #[test]
    fn coordinates_on_board() {
        let _coordinates = BoardCoordinates::<PUZZLE_SIZE>::new(3, 2);
    }

    #[test]
    #[should_panic]
    fn coordinates_not_on_board() {
        let _coordinates = BoardCoordinates::<PUZZLE_SIZE>::new(PUZZLE_SIZE as u8, PUZZLE_SIZE as u8);
    }

    #[test]
    fn correct_manhattan_distance() {
        let first_coordinates = BoardCoordinates::<PUZZLE_SIZE>::new(1, 3);
        let second_coordinates = BoardCoordinates::<PUZZLE_SIZE>::new(3, 2);
        let manhattan_distance = first_coordinates.manhattan_distance(&second_coordinates);
        let reverse_manhattan_distance = second_coordinates.manhattan_distance(&first_coordinates);

        assert_eq!(3, manhattan_distance);
        assert_eq!(manhattan_distance, reverse_manhattan_distance);
    }

    #[test]
    fn correct_blank_manhattan_distance() {
        let coordinates = BoardCoordinates::<PUZZLE_SIZE>::new(1, 2);
        let blank_manhattan_distance = coordinates.blank_manhattan_distance();

        assert_eq!(3, blank_manhattan_distance);
    }

    #[test]
    fn at_upper_edge() {
        let coordinates = BoardCoordinates::<PUZZLE_SIZE>::new(0, 1);
        assert!(coordinates.at_upper_edge());
    }

    #[test]
    fn not_at_upper_edge() {
        let coordinates = BoardCoordinates::<PUZZLE_SIZE>::new(1, 2);
        assert!(!coordinates.at_upper_edge());
    }

    #[test]
    fn at_bottom_edge() {
        let coordinates = BoardCoordinates::<PUZZLE_SIZE>::new((PUZZLE_SIZE - 1) as u8, 1);
        assert!(coordinates.at_bottom_edge());
    }

    #[test]
    fn not_at_bottom_edge() {
        let coordinates = BoardCoordinates::<PUZZLE_SIZE>::new(1, 1);
        assert!(!coordinates.at_bottom_edge());
    }

    #[test]
    fn at_left_edge() {
        let coordinates = BoardCoordinates::<PUZZLE_SIZE>::new(1, 0);
        assert!(coordinates.at_left_edge());
    }

    #[test]
    fn not_at_left_edge() {
        let coordinates = BoardCoordinates::<PUZZLE_SIZE>::new(1, 1);
        assert!(!coordinates.at_left_edge());
    }

    #[test]
    fn at_right_edge() {
        let coordinates = BoardCoordinates::<PUZZLE_SIZE>::new((PUZZLE_SIZE - 1) as u8, (PUZZLE_SIZE - 1) as u8);
        assert!(coordinates.at_right_edge());
    }

    #[test]
    fn not_at_right_edge() {
        let coordinates = BoardCoordinates::<PUZZLE_SIZE>::new(1, 1);
        assert!(!coordinates.at_right_edge());
    }
}
