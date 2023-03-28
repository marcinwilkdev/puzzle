//! State used to keep track which states were visited in BFS.

use super::combination::Combination;
use super::{DATABASE_SIZE, PUZZLE_SIZE};
use crate::puzzle_state::coordinates::BoardCoordinates;
use crate::puzzle_state::direction::Direction;

/**
* Neighbour created when moving blank, containing information if element other than blank was
* moved.
*/
pub struct Neighbour {
    board_state: BoardState,
    moved_element: bool,
}

impl Neighbour {
    /// Accessor for `board_state` field.
    pub fn board_state(&self) -> BoardState {
        self.board_state
    }

    /// Accessor for `moved_element` field.
    pub fn moved_element(&self) -> bool {
        self.moved_element
    }
}

/// Distinct board states visited by BFS algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoardState {
    elements_coordinates: [BoardCoordinates<PUZZLE_SIZE>; DATABASE_SIZE],
    blank_coordinates: BoardCoordinates<PUZZLE_SIZE>,
    ignore_last: bool,
}

impl BoardState {
    /// Creates new instance of [BoardState].
    pub fn new(
        elements_coordinates: [BoardCoordinates<PUZZLE_SIZE>; DATABASE_SIZE],
        blank_coordinates: BoardCoordinates<PUZZLE_SIZE>,
        ignore_last: bool,
    ) -> BoardState {
        BoardState {
            elements_coordinates,
            blank_coordinates,
            ignore_last,
        }
    }

    /// Extracts elements_coordinates as [Combination].
    pub fn extract_combination(&self) -> Combination {
        Combination::from_readable(self.elements_coordinates, self.ignore_last)
    }

    /// Creates state neighbours obtained by moving blank one move in each direction.
    pub fn neighbours(&self) -> Vec<Neighbour> {
        let mut neighbours = vec![];

        if !self.blank_coordinates.at_upper_edge() {
            neighbours.push(self.create_neighbour(Direction::Up));
        }

        if !self.blank_coordinates.at_bottom_edge() {
            neighbours.push(self.create_neighbour(Direction::Down));
        }

        if !self.blank_coordinates.at_left_edge() {
            neighbours.push(self.create_neighbour(Direction::Left));
        }

        if !self.blank_coordinates.at_right_edge() {
            neighbours.push(self.create_neighbour(Direction::Right));
        }

        neighbours
    }

    /// Creates neighbour obtained by moving blank in `direction`.
    fn create_neighbour(&self, direction: Direction) -> Neighbour {
        let (diff_row, diff_column) = direction.as_coordinates();
        let (blank_row, blank_column) = self.blank_coordinates.as_tuple();
        let new_blank_row = ((blank_row as isize) + diff_row) as u8;
        let new_blank_column = ((blank_column as isize) + diff_column) as u8;
        let new_blank_coordinates = BoardCoordinates::new(new_blank_row, new_blank_column);

        let mut elements_coordinates = self.elements_coordinates;
        let mut moved_element = false;

        for (element_index, element_coords) in elements_coordinates.iter_mut().enumerate() {
            if *element_coords == new_blank_coordinates {
                if self.ignore_last && element_index == (DATABASE_SIZE - 1) {
                    break;
                }

                let (row, column) = element_coords.as_tuple();
                let new_row = ((row as isize) + (diff_row * -1)) as u8;
                let new_column = ((column as isize) + (diff_column * -1)) as u8;

                *element_coords = BoardCoordinates::new(new_row, new_column);
                moved_element = true;

                break;
            }
        }

        let neighbour_board_state = BoardState::new(
            elements_coordinates,
            new_blank_coordinates,
            self.ignore_last,
        );

        Neighbour {
            board_state: neighbour_board_state,
            moved_element,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbour_without_move() {
        let board_state = BoardState::new(
            [
                BoardCoordinates::new(1, 0),
                BoardCoordinates::new(1, 1),
                BoardCoordinates::new(1, 2),
                BoardCoordinates::new(1, 3),
            ],
            BoardCoordinates::new((PUZZLE_SIZE - 1) as u8, (PUZZLE_SIZE - 1) as u8),
            false,
        );

        let neighbour = board_state.create_neighbour(Direction::Up);

        let expected_neighbour_board_state = BoardState::new(
            [
                BoardCoordinates::new(1, 0),
                BoardCoordinates::new(1, 1),
                BoardCoordinates::new(1, 2),
                BoardCoordinates::new(1, 3),
            ],
            BoardCoordinates::new((PUZZLE_SIZE - 2) as u8, (PUZZLE_SIZE - 1) as u8),
            false,
        );

        assert_eq!(expected_neighbour_board_state, neighbour.board_state);
        assert!(!neighbour.moved_element);
    }

    #[test]
    fn neighbour_with_move() {
        let board_state = BoardState::new(
            [
                BoardCoordinates::new(1, 0),
                BoardCoordinates::new(1, 1),
                BoardCoordinates::new(1, 2),
                BoardCoordinates::new(1, 3),
            ],
            BoardCoordinates::new(2, 3),
            false,
        );

        let neighbour = board_state.create_neighbour(Direction::Up);

        let expected_neighbour_board_state = BoardState::new(
            [
                BoardCoordinates::new(1, 0),
                BoardCoordinates::new(1, 1),
                BoardCoordinates::new(1, 2),
                BoardCoordinates::new(2, 3),
            ],
            BoardCoordinates::new(1, 3),
            false,
        );

        assert_eq!(expected_neighbour_board_state, neighbour.board_state);
        assert!(neighbour.moved_element);
    }

    #[test]
    fn neighbour_with_ignore_last() {
        let board_state = BoardState::new(
            [
                BoardCoordinates::new(1, 0),
                BoardCoordinates::new(1, 1),
                BoardCoordinates::new(1, 2),
                BoardCoordinates::new(1, 3),
            ],
            BoardCoordinates::new(2, 3),
            true,
        );

        let neighbour = board_state.create_neighbour(Direction::Up);

        let expected_neighbour_board_state = BoardState::new(
            [
                BoardCoordinates::new(1, 0),
                BoardCoordinates::new(1, 1),
                BoardCoordinates::new(1, 2),
                BoardCoordinates::new(1, 3),
            ],
            BoardCoordinates::new(1, 3),
            true,
        );

        assert_eq!(expected_neighbour_board_state, neighbour.board_state);
        assert!(!neighbour.moved_element);
    }

    #[test]
    fn neighbours_generated() {
        let board_state = BoardState::new(
            [
                BoardCoordinates::new(1, 0),
                BoardCoordinates::new(1, 1),
                BoardCoordinates::new(1, 2),
                BoardCoordinates::new(1, 3),
            ],
            BoardCoordinates::new(2, 3),
            false,
        );

        let neighbours = board_state.neighbours();

        assert_eq!(3, neighbours.len());

        let board_state = BoardState::new(
            [
                BoardCoordinates::new(1, 0),
                BoardCoordinates::new(1, 1),
                BoardCoordinates::new(1, 2),
                BoardCoordinates::new(1, 3),
            ],
            BoardCoordinates::new(3, 3),
            false,
        );

        let neighbours = board_state.neighbours();

        assert_eq!(2, neighbours.len());

        let board_state = BoardState::new(
            [
                BoardCoordinates::new(1, 0),
                BoardCoordinates::new(1, 1),
                BoardCoordinates::new(1, 2),
                BoardCoordinates::new(1, 3),
            ],
            BoardCoordinates::new(2, 2),
            false,
        );

        let neighbours = board_state.neighbours();

        assert_eq!(4, neighbours.len());
    }
}
