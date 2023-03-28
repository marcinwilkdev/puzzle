//! Generation of random initial states for sliding puzzle.

use rand::prelude::*;

use crate::puzzle_state::coordinates::BoardCoordinates;
use crate::{Direction, PuzzleState};

pub const PUZZLE_SIZE: usize = 4;
pub const SHUFFLE_ITERATIONS: usize = 1;

/// Generates random 15 puzzle state to be solved by solver.
pub fn generate_random_puzzle_state(steps_back: usize) -> PuzzleState<PUZZLE_SIZE> {
    let mut numbers = [
        [Some(1), Some(2), Some(3), Some(4)],
        [Some(5), Some(6), Some(7), Some(8)],
        [Some(9), Some(10), Some(11), Some(12)],
        [Some(13), Some(14), Some(15), None],
    ];

    let mut last_coordinates =
        BoardCoordinates::<PUZZLE_SIZE>::new((PUZZLE_SIZE - 1) as u8, (PUZZLE_SIZE - 1) as u8);
    let mut last_direction = Direction::Down;

    for _ in 0..steps_back {
        let mut available_directions = vec![];

        if !last_coordinates.at_upper_edge() {
            available_directions.push(Direction::Up);
        }

        if !last_coordinates.at_bottom_edge() {
            available_directions.push(Direction::Down);
        }

        if !last_coordinates.at_left_edge() {
            available_directions.push(Direction::Left);
        }

        if !last_coordinates.at_right_edge() {
            available_directions.push(Direction::Right);
        }

        available_directions.shuffle(&mut rand::thread_rng());

        let mut direction = available_directions[0];

        if direction == last_direction {
            direction = available_directions[1];
        }

        let (row, column) = last_coordinates.as_tuple();
        let (diff_row, diff_column) = direction.as_coordinates();

        let swap_row = ((row as isize) + diff_row) as u8;
        let swap_column = ((column as isize) + diff_column) as u8;

        let tmp = numbers[row as usize][column as usize];
        numbers[row as usize][column as usize] = numbers[swap_row as usize][swap_column as usize];
        numbers[swap_row as usize][swap_column as usize] = tmp;

        last_coordinates = BoardCoordinates::new(swap_row, swap_column);
        last_direction = direction.opposite();
    }

    PuzzleState::new(numbers).expect("Numbers have to be correct permutation after shuffle")
}
