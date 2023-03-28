//! Manhattan Distance heuristic.

use std::collections::HashMap;

use super::Heuristic;
use crate::puzzle_state::coordinates::BoardCoordinates;

/**
 * [Manhattan Distance](https://en.wikipedia.org/wiki/Taxicab_geometry) heuristic.
 * It calculates manhattan distance of each number from its proper position and sums those
 * distances up.
 */
pub struct ManhattanDistance<const PUZZLE_SIZE: usize> {
    solved_positions: HashMap<Option<u8>, BoardCoordinates<PUZZLE_SIZE>>,
}

impl<const PUZZLE_SIZE: usize> ManhattanDistance<PUZZLE_SIZE> {
    /**
     * Creates new instance of [ManhattanDistance] with precalculated solved positions for
     * numbers.
     */
    pub fn new() -> Self {
        let solved_positions = Self::create_solved_positions();

        ManhattanDistance { solved_positions }
    }

    /// Returns number coordinates in solved sliding puzzle game.
    fn solved_coordinates(number_value: usize) -> BoardCoordinates<PUZZLE_SIZE> {
        let number_index = number_value - 1;

        let row = number_index / PUZZLE_SIZE;
        let column = number_index % PUZZLE_SIZE;

        BoardCoordinates::new(row as u8, column as u8)
    }

    /// Creates map of number positions for which the sliding game is solved.
    fn create_solved_positions() -> HashMap<Option<u8>, BoardCoordinates<PUZZLE_SIZE>> {
        let number_count = (PUZZLE_SIZE * PUZZLE_SIZE) - 1;
        let mut solved_positions = HashMap::with_capacity(number_count + 1);

        for number_value in 1..=number_count {
            let number = Some(number_value as u8);
            let solved_coordinates = Self::solved_coordinates(number_value);

            solved_positions.insert(number, solved_coordinates);
        }

        solved_positions.insert(
            None,
            BoardCoordinates::new((PUZZLE_SIZE - 1) as u8, (PUZZLE_SIZE - 1) as u8),
        );

        solved_positions
    }
}

impl<const PUZZLE_SIZE: usize> Heuristic<PUZZLE_SIZE> for ManhattanDistance<PUZZLE_SIZE> {
    fn calculate(&self, numbers: &[[Option<u8>; PUZZLE_SIZE]; PUZZLE_SIZE]) -> u8 {
        let mut distance = 0;

        for (row, number_row) in numbers.iter().enumerate() {
            for (column, number) in number_row.iter().enumerate() {
                if number.is_some() {
                    let number_solved_coordinates = self
                        .solved_positions
                        .get(number)
                        .expect("ManhattanDistance has to have all number distances cached.");
                    let number_actual_coordinates =
                        BoardCoordinates::<PUZZLE_SIZE>::new(row as u8, column as u8);

                    distance +=
                        number_actual_coordinates.manhattan_distance(number_solved_coordinates);
                }
            }
        }

        distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::puzzle_state::PuzzleState;

    const BIGGER_PUZZLE_SIZE: usize = 3;

    #[test]
    fn heuristic_works() {
        let manhattan_distance = ManhattanDistance::new();

        let puzzle_state = PuzzleState::<BIGGER_PUZZLE_SIZE>::new([
            [Some(1), Some(4), Some(2)],
            [Some(3), None, Some(5)],
            [Some(6), Some(7), Some(8)],
        ])
        .unwrap();

        let heuristic_value = puzzle_state.calculate_heuristic(&manhattan_distance);

        assert_eq!(12, heuristic_value);

        let puzzle_state = PuzzleState::<BIGGER_PUZZLE_SIZE>::new([
            [Some(1), Some(4), Some(2)],
            [Some(6), None, Some(5)],
            [Some(7), Some(3), Some(8)],
        ])
        .unwrap();

        let heuristic_value = puzzle_state.calculate_heuristic(&manhattan_distance);

        assert_eq!(10, heuristic_value);
    }
}
