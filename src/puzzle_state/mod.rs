//! Puzzle state for sliding puzzle game.

pub mod coordinates;
pub mod direction;
pub mod errors;
pub mod parity_check_permutation;
pub mod puzzle_move;

use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;

use coordinates::BoardCoordinates;
use direction::Direction;
use errors::{PuzzleStateCreationError, PuzzleStateParseError};
use parity_check_permutation::ParityCheckPermutation;
use puzzle_move::Move;

use crate::heuristics::Heuristic;

const BLANK_NUMBER: u64 = 0b1111;
const MAX_NUMBER_WIDTH: usize = 4;

/// Stores puzzle state for sliding puzzle game of `PUZZLE_SIZE` size.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct PuzzleState<const PUZZLE_SIZE: usize> {
    numbers: u64,
}

// API impl block
impl<const PUZZLE_SIZE: usize> PuzzleState<PUZZLE_SIZE> {
    /// Creates new instance of [PuzzleState].
    pub fn new(
        numbers: [[Option<u8>; PUZZLE_SIZE]; PUZZLE_SIZE],
    ) -> Result<Self, PuzzleStateCreationError> {
        Self::check_numbers(&numbers)?;

        Ok(PuzzleState {
            numbers: Self::numbers_from_readable(&numbers),
        })
    }

    /// Returns currently contained numbers in readable form.
    pub fn readable_numbers(&self) -> [[Option<u8>; PUZZLE_SIZE]; PUZZLE_SIZE] {
        Self::numbers_into_readable(self.numbers)
    }

    /// Checks if state is a valid solution in sliding puzzle game.
    pub fn is_solved(&self) -> bool {
        let blank_manhattan_distance = self.blank_position().blank_manhattan_distance();

        (blank_manhattan_distance == 0) && self.is_solved_permutation()
    }

    /// Checks if goal state is achievable from this state.
    pub fn is_solvable(&self) -> bool {
        let parity_check_permutation =
            ParityCheckPermutation::from_numbers(&self.readable_numbers());
        let blank_manhattan_distance = self.blank_position().blank_manhattan_distance();
        let is_blank_manhattan_distance_even = (blank_manhattan_distance % 2) == 0;

        (parity_check_permutation.is_even() && is_blank_manhattan_distance_even)
            || (!parity_check_permutation.is_even() && !is_blank_manhattan_distance_even)
    }

    /// Creates state obtained by moving blank in given `direction`.
    pub fn create_neighbour_move_state(&self, direction: Direction) -> PuzzleState<PUZZLE_SIZE> {
        let (diff_row, diff_column) = direction.as_coordinates();
        let (blank_row, blank_column) = self.blank_position().as_tuple();

        // Values for blanks can't be so big, not to fit isize.
        // blank + diff values can't be negative (checked before this function call).
        let swap_row = ((blank_row as isize) + diff_row) as usize;
        let swap_column = ((blank_column as isize) + diff_column) as usize;

        let mut new_numbers = self.readable_numbers();
        let swap_number = new_numbers[swap_row][swap_column];

        new_numbers[swap_row][swap_column] = None;
        new_numbers[blank_row as usize][blank_column as usize] = swap_number;

        PuzzleState {
            numbers: Self::numbers_from_readable(&new_numbers),
        }
    }

    /// Creates states obtainable from current one by performing one move.
    pub fn neighbours(&self) -> Vec<Move<PUZZLE_SIZE>> {
        let mut moves = vec![];

        if !self.blank_position().at_upper_edge() {
            moves.push(Move::new(
                Direction::Up,
                self.create_neighbour_move_state(Direction::Up),
            ));
        }

        if !self.blank_position().at_bottom_edge() {
            moves.push(Move::new(
                Direction::Down,
                self.create_neighbour_move_state(Direction::Down),
            ));
        }

        if !self.blank_position().at_left_edge() {
            moves.push(Move::new(
                Direction::Left,
                self.create_neighbour_move_state(Direction::Left),
            ));
        }

        if !self.blank_position().at_right_edge() {
            moves.push(Move::new(
                Direction::Right,
                self.create_neighbour_move_state(Direction::Right),
            ));
        }

        moves
    }

    /// Calculates `heuristic` value on state.
    pub fn calculate_heuristic(&self, heuristic: &dyn Heuristic<PUZZLE_SIZE>) -> u8 {
        heuristic.calculate(&self.readable_numbers())
    }
}

// Private impl block
impl<const PUZZLE_SIZE: usize> PuzzleState<PUZZLE_SIZE> {
    /// Returns calculated blank position
    fn blank_position(&self) -> BoardCoordinates<PUZZLE_SIZE> {
        let readable_numbers = self.readable_numbers();

        for (row, numbers_row) in readable_numbers.iter().enumerate() {
            for (column, number) in numbers_row.iter().enumerate() {
                if number.is_none() {
                    return BoardCoordinates::new(row as u8, column as u8);
                }
            }
        }

        unreachable!("Blank has to be found in numbers");
    }

    /// Transforms numbers from internal form to readable form.
    fn numbers_into_readable(numbers: u64) -> [[Option<u8>; PUZZLE_SIZE]; PUZZLE_SIZE] {
        let mut readable_numbers = [[None; PUZZLE_SIZE]; PUZZLE_SIZE];
        let mut number_index = 0;

        for numbers_row in &mut readable_numbers {
            for number in numbers_row {
                let internal_number =
                    ((numbers >> (MAX_NUMBER_WIDTH * number_index)) & 0b1111) as u8;

                if (internal_number as u64) < BLANK_NUMBER {
                    *number = Some(internal_number + 1);
                }

                number_index += 1;
            }
        }

        readable_numbers
    }

    /// Transforms numbers from readable form to internal form.
    fn numbers_from_readable(numbers: &[[Option<u8>; PUZZLE_SIZE]; PUZZLE_SIZE]) -> u64 {
        let mut internal_number: u64 = 0;
        let mut internal_number_index = 0;

        for number_row in numbers {
            for number in number_row {
                if let Some(number_value) = number {
                    internal_number +=
                        ((number_value - 1) as u64) << (MAX_NUMBER_WIDTH * internal_number_index);
                } else {
                    internal_number += BLANK_NUMBER << (MAX_NUMBER_WIDTH * internal_number_index);
                }
                internal_number_index += 1;
            }
        }
        internal_number
    }

    /// Checks if `numbers` are correct for [PuzzleState]
    fn check_numbers(
        numbers: &[[Option<u8>; PUZZLE_SIZE]; PUZZLE_SIZE],
    ) -> Result<(), PuzzleStateCreationError> {
        let max_number_value = ((PUZZLE_SIZE * PUZZLE_SIZE) as u8) - 1;
        let mut permutation_numbers: HashSet<_> = (1..=max_number_value).into_iter().collect();
        let mut blank_found = false;

        for number_row in numbers {
            for number in number_row {
                if let Some(number_value) = number {
                    if !permutation_numbers.remove(number_value) {
                        return Err(PuzzleStateCreationError::NotPermutation);
                    }
                } else if !blank_found {
                    blank_found = true;
                } else {
                    return Err(PuzzleStateCreationError::TwoBlanks);
                }
            }
        }

        if !permutation_numbers.is_empty() {
            return Err(PuzzleStateCreationError::NotPermutation);
        }

        // blank has to be found at this stage
        Ok(())
    }

    /// Checks if number permutation is in solved position.
    fn is_solved_permutation(&self) -> bool {
        let mut curr_correct_number_value = 1;

        for number_row in self.readable_numbers() {
            for number in number_row {
                if number != Some(curr_correct_number_value) {
                    return false;
                }

                curr_correct_number_value += 1;

                if curr_correct_number_value == (PUZZLE_SIZE * PUZZLE_SIZE) as u8 {
                    break;
                }
            }
        }

        true
    }
}

impl<const PUZZLE_SIZE: usize> FromStr for PuzzleState<PUZZLE_SIZE> {
    type Err = PuzzleStateParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        let permutation_start_index = s.find('[').ok_or(PuzzleStateParseError::NoBrackets)?;
        let permutation_end_index = s.find(']').ok_or(PuzzleStateParseError::NoBrackets)?;

        let permutation_start_index = permutation_start_index + '['.len_utf8();
        let permutation = &s[permutation_start_index..permutation_end_index];

        let mut permutation_members = permutation.split(",");

        let mut numbers = [[None; PUZZLE_SIZE]; PUZZLE_SIZE];

        for numbers_row in &mut numbers {
            for number in numbers_row {
                let permutation_member = permutation_members
                    .next()
                    .ok_or(PuzzleStateParseError::NotEnoughNumbers)?
                    .trim();

                if permutation_member != "" {
                    let number_value = permutation_member
                        .parse::<u8>()
                        .map_err(|_| PuzzleStateParseError::NumberParseError)?;

                    *number = Some(number_value);
                }
            }
        }

        if permutation_members.next().is_some() {
            Err(PuzzleStateParseError::TooManyNumbers)
        } else {
            Ok(PuzzleState::new(numbers)?)
        }
    }
}

impl<const PUZZLE_SIZE: usize> Display for PuzzleState<PUZZLE_SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut number_strings = vec![];

        let readable_numbers = self.readable_numbers();

        for number_row in readable_numbers {
            for number in number_row {
                if let Some(number_value) = number {
                    number_strings.push(number_value.to_string());
                } else {
                    number_strings.push("".to_string());
                }
            }
        }

        write!(f, "[{}]", number_strings.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE_SIZE: usize = 2;
    const BIGGER_PUZZLE_SIZE: usize = 3;
    const BIGGEST_PUZZLE_SIZE: usize = 4;

    #[test]
    fn readable_and_internal() {
        let readable_numbers = [[Some(1), Some(2)], [Some(3), None]];
        let internal_numbers = PuzzleState::<PUZZLE_SIZE>::numbers_from_readable(&readable_numbers);
        let readable_from_internal_numbers =
            PuzzleState::<PUZZLE_SIZE>::numbers_into_readable(internal_numbers);

        assert_eq!(readable_numbers, readable_from_internal_numbers);
        assert_eq!(
            0b00000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111_0010_0001_0000,
            internal_numbers
        );
    }

    #[test]
    fn not_permutation_puzzle_state() {
        let puzzle_state = PuzzleState::<PUZZLE_SIZE>::new([[Some(1), Some(1)], [Some(2), None]]);

        assert!(matches!(
            puzzle_state,
            Err(PuzzleStateCreationError::NotPermutation)
        ));
    }

    #[test]
    fn two_blanks_puzzle_state() {
        let puzzle_state = PuzzleState::<PUZZLE_SIZE>::new([[Some(1), Some(2)], [None, None]]);

        assert!(matches!(
            puzzle_state,
            Err(PuzzleStateCreationError::TwoBlanks)
        ));
    }

    #[test]
    fn valid_puzzle_state_created() {
        let puzzle_state = PuzzleState::<PUZZLE_SIZE>::new([[Some(2), Some(1)], [None, Some(3)]]);

        assert!(puzzle_state.is_ok());
    }

    #[test]
    fn blank_positioning_works() {
        let puzzle_state =
            PuzzleState::<PUZZLE_SIZE>::new([[Some(2), Some(1)], [None, Some(3)]]).unwrap();

        assert_eq!(BoardCoordinates::new(1, 0), puzzle_state.blank_position());

        let puzzle_state =
            PuzzleState::<PUZZLE_SIZE>::new([[Some(2), None], [Some(1), Some(3)]]).unwrap();

        assert_eq!(BoardCoordinates::new(0, 1), puzzle_state.blank_position());
    }

    #[test]
    fn not_solved_state() {
        let puzzle_state =
            PuzzleState::<PUZZLE_SIZE>::new([[Some(2), Some(1)], [None, Some(3)]]).unwrap();

        assert!(!puzzle_state.is_solved());

        let puzzle_state =
            PuzzleState::<PUZZLE_SIZE>::new([[Some(2), Some(1)], [Some(3), None]]).unwrap();

        assert!(!puzzle_state.is_solved());
    }

    #[test]
    fn solved_state() {
        let puzzle_state =
            PuzzleState::<PUZZLE_SIZE>::new([[Some(1), Some(2)], [Some(3), None]]).unwrap();

        assert!(puzzle_state.is_solved());
    }

    #[test]
    fn solvable_state() {
        let puzzle_state =
            PuzzleState::<PUZZLE_SIZE>::new([[Some(1), Some(2)], [Some(3), None]]).unwrap();

        assert!(puzzle_state.is_solvable());

        let puzzle_state =
            PuzzleState::<PUZZLE_SIZE>::new([[Some(3), Some(1)], [Some(2), None]]).unwrap();

        assert!(puzzle_state.is_solvable());

        let puzzle_state =
            PuzzleState::<PUZZLE_SIZE>::new([[Some(3), Some(1)], [None, Some(2)]]).unwrap();

        assert!(puzzle_state.is_solvable());

        let puzzle_state = PuzzleState::<BIGGEST_PUZZLE_SIZE>::new([
            [Some(1), Some(2), Some(3), Some(4)],
            [Some(5), Some(6), Some(7), Some(8)],
            [Some(9), Some(10), Some(11), Some(12)],
            [Some(13), Some(14), None, Some(15)],
        ])
        .unwrap();

        assert!(puzzle_state.is_solvable());
    }

    #[test]
    fn not_solvable_state() {
        let puzzle_state =
            PuzzleState::<PUZZLE_SIZE>::new([[Some(2), Some(1)], [Some(3), None]]).unwrap();

        assert!(!puzzle_state.is_solvable());

        let puzzle_state =
            PuzzleState::<PUZZLE_SIZE>::new([[Some(2), Some(1)], [None, Some(3)]]).unwrap();

        assert!(!puzzle_state.is_solvable());
    }

    #[test]
    fn upper_neighbour() {
        let puzzle_state =
            PuzzleState::<PUZZLE_SIZE>::new([[Some(3), Some(1)], [None, Some(2)]]).unwrap();

        let expected_numbers = PuzzleState::<PUZZLE_SIZE>::numbers_from_readable(&[
            [None, Some(1)],
            [Some(3), Some(2)],
        ]);

        let expected_obtained_state = PuzzleState {
            numbers: expected_numbers,
        };

        let obtained_state = puzzle_state.create_neighbour_move_state(Direction::Up);

        assert_eq!(expected_obtained_state, obtained_state);
    }

    #[test]
    fn bottom_neighbour() {
        let puzzle_state =
            PuzzleState::<PUZZLE_SIZE>::new([[None, Some(1)], [Some(3), Some(2)]]).unwrap();

        let expected_numbers = PuzzleState::<PUZZLE_SIZE>::numbers_from_readable(&[
            [Some(3), Some(1)],
            [None, Some(2)],
        ]);

        let expected_obtained_state = PuzzleState {
            numbers: expected_numbers,
        };

        let obtained_state = puzzle_state.create_neighbour_move_state(Direction::Down);

        assert_eq!(expected_obtained_state, obtained_state);
    }

    #[test]
    fn left_neighbour() {
        let puzzle_state =
            PuzzleState::<PUZZLE_SIZE>::new([[Some(1), None], [Some(3), Some(2)]]).unwrap();

        let expected_numbers = PuzzleState::<PUZZLE_SIZE>::numbers_from_readable(&[
            [None, Some(1)],
            [Some(3), Some(2)],
        ]);

        let expected_obtained_state = PuzzleState {
            numbers: expected_numbers,
        };

        let obtained_state = puzzle_state.create_neighbour_move_state(Direction::Left);

        assert_eq!(expected_obtained_state, obtained_state);
    }

    #[test]
    fn right_neighbour() {
        let puzzle_state =
            PuzzleState::<PUZZLE_SIZE>::new([[None, Some(1)], [Some(3), Some(2)]]).unwrap();

        let expected_numbers = PuzzleState::<PUZZLE_SIZE>::numbers_from_readable(&[
            [Some(1), None],
            [Some(3), Some(2)],
        ]);

        let expected_obtained_state = PuzzleState {
            numbers: expected_numbers,
        };

        let obtained_state = puzzle_state.create_neighbour_move_state(Direction::Right);

        assert_eq!(expected_obtained_state, obtained_state);
    }

    #[test]
    fn two_neighbours() {
        let puzzle_state = PuzzleState::<BIGGER_PUZZLE_SIZE>::new([
            [None, Some(1), Some(2)],
            [Some(3), Some(4), Some(5)],
            [Some(6), Some(7), Some(8)],
        ])
        .unwrap();

        let neighbours = puzzle_state.neighbours();

        assert_eq!(2, neighbours.len());
    }

    #[test]
    fn three_neighbours() {
        let puzzle_state = PuzzleState::<BIGGER_PUZZLE_SIZE>::new([
            [Some(1), None, Some(2)],
            [Some(3), Some(4), Some(5)],
            [Some(6), Some(7), Some(8)],
        ])
        .unwrap();

        let neighbours = puzzle_state.neighbours();

        assert_eq!(3, neighbours.len());
    }

    #[test]
    fn four_neighbours() {
        let puzzle_state = PuzzleState::<BIGGER_PUZZLE_SIZE>::new([
            [Some(1), Some(4), Some(2)],
            [Some(3), None, Some(5)],
            [Some(6), Some(7), Some(8)],
        ])
        .unwrap();

        let neighbours = puzzle_state.neighbours();

        assert_eq!(4, neighbours.len());
    }

    #[test]
    fn parse_succesfull() {
        let puzzle_state_str = "[1, 4, 2, 3, , 5, 6, 7, 8]";
        let expected_numbers = [
            [Some(1), Some(4), Some(2)],
            [Some(3), None, Some(5)],
            [Some(6), Some(7), Some(8)],
        ];

        let puzzle_state_parse_result = puzzle_state_str.parse::<PuzzleState<BIGGER_PUZZLE_SIZE>>();

        assert!(puzzle_state_parse_result.is_ok());

        let puzzle_state = puzzle_state_parse_result.unwrap();

        assert_eq!(expected_numbers, puzzle_state.readable_numbers());
    }

    #[test]
    fn parse_no_brackets() {
        let puzzle_state_str = "[1, 4, 2, 3, , 5, 6, 7, 8";
        let puzzle_state_parse_result = puzzle_state_str.parse::<PuzzleState<BIGGER_PUZZLE_SIZE>>();

        assert!(matches!(
            puzzle_state_parse_result,
            Err(PuzzleStateParseError::NoBrackets)
        ));

        let puzzle_state_str = "1, 4, 2, 3, , 5, 6, 7, 8]";
        let puzzle_state_parse_result = puzzle_state_str.parse::<PuzzleState<BIGGER_PUZZLE_SIZE>>();

        assert!(matches!(
            puzzle_state_parse_result,
            Err(PuzzleStateParseError::NoBrackets)
        ));

        let puzzle_state_str = "1, 4, 2, 3, , 5, 6, 7, 8";
        let puzzle_state_parse_result = puzzle_state_str.parse::<PuzzleState<BIGGER_PUZZLE_SIZE>>();

        assert!(matches!(
            puzzle_state_parse_result,
            Err(PuzzleStateParseError::NoBrackets)
        ));
    }

    #[test]
    fn parse_not_enough_numbers() {
        let puzzle_state_str = "[1, 4, 2, 3, , 5, 6, 7]";
        let puzzle_state_parse_result = puzzle_state_str.parse::<PuzzleState<BIGGER_PUZZLE_SIZE>>();

        assert!(matches!(
            puzzle_state_parse_result,
            Err(PuzzleStateParseError::NotEnoughNumbers)
        ));
    }

    #[test]
    fn parse_too_many_numbers() {
        let puzzle_state_str = "[1, 4, 2, 3, , 5, 6, 7, 8, 9]";
        let puzzle_state_parse_result = puzzle_state_str.parse::<PuzzleState<BIGGER_PUZZLE_SIZE>>();

        assert!(matches!(
            puzzle_state_parse_result,
            Err(PuzzleStateParseError::TooManyNumbers)
        ));
    }

    #[test]
    fn parse_number_parse_error() {
        let puzzle_state_str = "[1, 4, 2, 3, , 5, 6, 7, a]";
        let puzzle_state_parse_result = puzzle_state_str.parse::<PuzzleState<BIGGER_PUZZLE_SIZE>>();

        assert!(matches!(
            puzzle_state_parse_result,
            Err(PuzzleStateParseError::NumberParseError)
        ));
    }

    #[test]
    fn puzzle_state_to_string() {
        let puzzle_state_str = "[1, 4, 2, 3, , 5, 6, 7, 8]";
        let puzzle_state: PuzzleState<BIGGER_PUZZLE_SIZE> = puzzle_state_str.parse().unwrap();
        let puzzle_state_str = puzzle_state.to_string();

        assert_eq!("[1, 4, 2, 3, , 5, 6, 7, 8]", &puzzle_state_str);
    }
}
