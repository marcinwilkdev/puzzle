//! States used to keep BFS state in frontier.

use super::board_state::BoardState;
use super::combination::Combination;
use super::PUZZLE_SIZE;
use crate::puzzle_state::coordinates::BoardCoordinates;

/// Elements of BFS frontier.
#[derive(Debug)]
pub struct BFSState {
    board_state: BoardState,
    element_shifts: u8,
    ignore_last: bool,
}

impl BFSState {
    /// Initial BFS state corresponding to solved puzzles.
    pub fn initial(database_first_element_index: usize, ignore_last: bool) -> BFSState {
        let elements_row = database_first_element_index / PUZZLE_SIZE;

        let mut elements_coordinates = [BoardCoordinates::<PUZZLE_SIZE>::new(0, 0); PUZZLE_SIZE];

        for element_column in 0..PUZZLE_SIZE {
            elements_coordinates[element_column] =
                BoardCoordinates::new(elements_row as u8, element_column as u8);
        }

        let blank_coordinates =
            BoardCoordinates::new((PUZZLE_SIZE - 1) as u8, (PUZZLE_SIZE - 1) as u8);

        BFSState {
            board_state: BoardState::new(elements_coordinates, blank_coordinates, ignore_last),
            element_shifts: 0,
            ignore_last,
        }
    }

    /// Returns current [BFSState] as [Combination] and distance for use in Database hashmap.
    pub fn combination_and_distance(&self) -> (Combination, u8) {
        (self.board_state.extract_combination(), self.element_shifts)
    }

    /// Returns this state's board state.
    pub fn board_state(&self) -> BoardState {
        self.board_state
    }

    /// Creates neighbours as BFS states
    pub fn neighbours(&self) -> Vec<BFSState> {
        let mut neighbours = vec![];

        let board_state_neighbours = self.board_state.neighbours();

        for board_state_neighbour in board_state_neighbours {
            let neighbour_board_state = board_state_neighbour.board_state();
            let neighbour_moved_element = board_state_neighbour.moved_element();

            if neighbour_moved_element {
                neighbours.push(BFSState {
                    board_state: neighbour_board_state,
                    element_shifts: self.element_shifts + 1,
                    ignore_last: self.ignore_last,
                });
            } else {
                neighbours.push(BFSState {
                    board_state: neighbour_board_state,
                    element_shifts: self.element_shifts,
                    ignore_last: self.ignore_last,
                });
            }
        }

        neighbours
    }
}

impl PartialEq for BFSState {
    fn eq(&self, other: &Self) -> bool {
        self.element_shifts.eq(&other.element_shifts)
    }
}

impl Eq for BFSState {}

impl PartialOrd for BFSState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BFSState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.element_shifts.cmp(&other.element_shifts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state_works() {
        let initial = BFSState::initial(0, false);

        assert_eq!(
            BoardState::new(
                [
                    BoardCoordinates::new(0, 0),
                    BoardCoordinates::new(0, 1),
                    BoardCoordinates::new(0, 2),
                    BoardCoordinates::new(0, 3)
                ],
                BoardCoordinates::new((PUZZLE_SIZE - 1) as u8, (PUZZLE_SIZE - 1) as u8),
                false,
            ),
            initial.board_state
        );

        assert_eq!(0, initial.element_shifts);

        let initial = BFSState::initial(4, false);

        assert_eq!(
            BoardState::new(
                [
                    BoardCoordinates::new(1, 0),
                    BoardCoordinates::new(1, 1),
                    BoardCoordinates::new(1, 2),
                    BoardCoordinates::new(1, 3)
                ],
                BoardCoordinates::new((PUZZLE_SIZE - 1) as u8, (PUZZLE_SIZE - 1) as u8),
                false
            ),
            initial.board_state
        );

        assert_eq!(0, initial.element_shifts);
    }

    #[test]
    fn neighbours_work() {
        let initial = BFSState::initial(8, false);
        let neighbours = initial.neighbours();

        let sum_moved: u8 = neighbours
            .iter()
            .map(|neighbour| neighbour.element_shifts)
            .sum();

        assert_eq!(1, sum_moved);

        let initial = BFSState::initial(4, false);
        let neighbours = initial.neighbours();

        let sum_moved: u8 = neighbours
            .iter()
            .map(|neighbour| neighbour.element_shifts)
            .sum();

        assert_eq!(0, sum_moved);
    }
}
