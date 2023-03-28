//! Combination used for state lookup when using disjoint databases heuristic

use serde::{Deserialize, Serialize};

use super::{DATABASE_SIZE, PUZZLE_SIZE};
use crate::puzzle_state::coordinates::BoardCoordinates;

const COORD_WIDTH: usize = 4;

/// Positions of each of database elements in permutation as index in permutation array.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct Combination {
    positions: u16, // 4 4-bit indexes encoded into one with binary shifts
}

impl Combination {
    /// Creates [Combination] instance from readable coordinates representation.
    pub fn from_readable(
        coordinates: [BoardCoordinates<PUZZLE_SIZE>; DATABASE_SIZE],
        ignore_last: bool,
    ) -> Self {
        let mut positions = 0;

        for (coord_index, coordinate) in coordinates.into_iter().enumerate() {
            if ignore_last && coord_index == (DATABASE_SIZE - 1) {
                break;
            }

            let (row, column) = coordinate.as_tuple();
            let coord_index_on_board = (row * (PUZZLE_SIZE as u8)) + column;

            positions += (coord_index_on_board as u16) << (COORD_WIDTH * coord_index);
        }

        Combination { positions }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_from_readable() {
        let coordinates = [
            BoardCoordinates::new(0, 0),
            BoardCoordinates::new(1, 1),
            BoardCoordinates::new(2, 2),
            BoardCoordinates::new(3, 3),
        ];

        let combination = Combination::from_readable(coordinates, false);

        assert_eq!(0b1111_1010_0101_0000, combination.positions);
    }

    #[test]
    fn create_from_readable_ignore_last() {
        let coordinates = [
            BoardCoordinates::new(0, 0),
            BoardCoordinates::new(1, 1),
            BoardCoordinates::new(2, 2),
            BoardCoordinates::new(3, 3),
        ];

        let combination = Combination::from_readable(coordinates, true);

        assert_eq!(0b0000_1010_0101_0000, combination.positions);
    }
}
