//! Move structure used to keep track of how we move on board when solving sliding puzzle.

use super::{Direction, PuzzleState};

/// Move of blank in sliding puzzle game.
pub struct Move<const PUZZLE_SIZE: usize> {
    direction: Direction,
    obtained_state: PuzzleState<PUZZLE_SIZE>,
}

impl<const PUZZLE_SIZE: usize> Move<PUZZLE_SIZE> {
    /// Destructures [Move] into `direction` and `obtained_state`.
    pub fn into_direction_and_puzzle_state(self) -> (Direction, PuzzleState<PUZZLE_SIZE>) {
        (self.direction, self.obtained_state)
    }

    /// Creates new instance of [Move].
    pub fn new(direction: Direction, obtained_state: PuzzleState<PUZZLE_SIZE>) -> Self {
        Move {
            direction,
            obtained_state,
        }
    }
}
