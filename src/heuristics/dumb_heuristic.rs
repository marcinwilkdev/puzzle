//! Heuristic used for testing.

use super::Heuristic;

/// Heuristic used for testing (it just sums values of numbers in `numbers`).
pub struct DumbHeuristic;

impl<const PUZZLE_SIZE: usize> Heuristic<PUZZLE_SIZE> for DumbHeuristic {
    fn calculate(&self, numbers: &[[Option<u8>; PUZZLE_SIZE]; PUZZLE_SIZE]) -> u8 {
        let mut heuristic_value = 0;

        for numbers_row in numbers {
            for number in numbers_row {
                if let Some(number_value) = number {
                    heuristic_value += number_value;
                }
            }
        }

        heuristic_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::puzzle_state::PuzzleState;

    const BIGGER_PUZZLE_SIZE: usize = 3;

    #[test]
    fn heuristic_works() {
        let puzzle_state = PuzzleState::<BIGGER_PUZZLE_SIZE>::new([
            [Some(1), Some(4), Some(2)],
            [Some(3), None, Some(5)],
            [Some(6), Some(7), Some(8)],
        ])
        .unwrap();

        let heuristic_value = puzzle_state.calculate_heuristic(&DumbHeuristic);

        assert_eq!((1..=8).sum::<u8>(), heuristic_value);
    }
}
