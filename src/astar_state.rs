//! State used when performing A* algorithm to find solution of sliding puzzle game.

use std::collections::HashMap;

use crate::heuristics::Heuristic;
use crate::puzzle_state::direction::Direction;
use crate::puzzle_state::puzzle_move::Move;
use crate::puzzle_state::PuzzleState;

/// A* state errors.
#[derive(Debug)]
pub enum AstarStateError {
    /// Initial state provided to algorithm is not solvable.
    InitialStateNotSolvable,
}

/// A* searching state.
#[derive(Debug, Clone)]
pub struct AstarState<const PUZZLE_SIZE: usize> {
    f_value: u8,
    last_direction: Option<Direction>,
    distance_from_start: u8,
    puzzle_state: PuzzleState<PUZZLE_SIZE>,
}

impl<const PUZZLE_SIZE: usize> AstarState<PUZZLE_SIZE> {
    /// Create initial [AstarState] from initial [PuzzleState].
    pub fn inital(
        puzzle_state: PuzzleState<PUZZLE_SIZE>,
        heuristic: &dyn Heuristic<PUZZLE_SIZE>,
    ) -> Result<Self, AstarStateError> {
        if !puzzle_state.is_solvable() {
            Err(AstarStateError::InitialStateNotSolvable)
        } else {
            Ok(AstarState {
                f_value: puzzle_state.calculate_heuristic(heuristic),
                last_direction: None,
                distance_from_start: 0,
                puzzle_state,
            })
        }
    }

    /// Returns [AstarState] after move to given neighbour.
    pub fn moved_to_neighbour(
        &self,
        direction: Direction,
        obtained_state: PuzzleState<PUZZLE_SIZE>,
        heuristic: &dyn Heuristic<PUZZLE_SIZE>,
    ) -> AstarState<PUZZLE_SIZE> {
        let neighbour_shortest_path_len = self.distance_from_start + 1;
        // There can't occur overflow here for puzzle of size 4.
        let f_value = neighbour_shortest_path_len + obtained_state.calculate_heuristic(heuristic);

        AstarState {
            f_value,
            last_direction: Some(direction),
            distance_from_start: neighbour_shortest_path_len,
            puzzle_state: obtained_state,
        }
    }

    /// Create neighbours of current A* state.
    pub fn neighbours(&self) -> Vec<Move<PUZZLE_SIZE>> {
        self.puzzle_state.neighbours()
    }

    /// Checks if state equals goal state.
    pub fn is_solved(&self) -> bool {
        self.puzzle_state.is_solved()
    }

    /// Returns inner puzzle state.
    pub fn puzzle_state(&self) -> PuzzleState<PUZZLE_SIZE> {
        self.puzzle_state
    }

    /// Accessor for `last_direction` field.
    pub fn last_direction(&self) -> Option<Direction> {
        self.last_direction
    }

    /// Creates route leading from first puzzle_state to current one.
    pub fn create_route(
        &self,
        last_directions: &HashMap<PuzzleState<PUZZLE_SIZE>, Option<Direction>>,
    ) -> Vec<Direction> {
        let mut curr_puzzle_state = self.puzzle_state.clone();
        let mut curr_direction = self.last_direction;
        let mut reversed_route = vec![];

        while let Some(direction) = curr_direction {
            reversed_route.push(direction);

            let opposite_direction = direction.opposite();
            curr_puzzle_state = curr_puzzle_state.create_neighbour_move_state(opposite_direction);
            curr_direction = *last_directions
                .get(&curr_puzzle_state)
                .expect("There has to be entry in last_directions for puzzle route.");
        }

        reversed_route.into_iter().rev().collect()
    }
}

impl<const PUZZLE_SIZE: usize> PartialEq for AstarState<PUZZLE_SIZE> {
    fn eq(&self, other: &Self) -> bool {
        self.f_value == other.f_value
    }
}

impl<const PUZZLE_SIZE: usize> Eq for AstarState<PUZZLE_SIZE> {}

impl<const PUZZLE_SIZE: usize> PartialOrd for AstarState<PUZZLE_SIZE> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const PUZZLE_SIZE: usize> Ord for AstarState<PUZZLE_SIZE> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f_value.cmp(&other.f_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::heuristics::dumb_heuristic::DumbHeuristic;

    const BIGGER_PUZZLE_SIZE: usize = 3;

    #[test]
    fn initial_state() {
        let puzzle_state = PuzzleState::<BIGGER_PUZZLE_SIZE>::new([
            [Some(1), Some(2), Some(3)],
            [Some(4), Some(5), Some(6)],
            [Some(7), Some(8), None],
        ])
        .unwrap();

        let expected_puzzle_state = puzzle_state.clone();

        let astar_state_result = AstarState::inital(puzzle_state, &DumbHeuristic);

        assert!(astar_state_result.is_ok());

        let AstarState {
            f_value,
            last_direction,
            distance_from_start,
            puzzle_state,
        } = astar_state_result.unwrap();

        assert_eq!(36, f_value);
        assert_eq!(None, last_direction);
        assert_eq!(0, distance_from_start);
        assert_eq!(puzzle_state, expected_puzzle_state);
    }

    #[test]
    fn initial_state_unsolvable() {
        let puzzle_state = PuzzleState::<BIGGER_PUZZLE_SIZE>::new([
            [Some(1), Some(2), Some(3)],
            [Some(4), Some(5), Some(6)],
            [Some(8), Some(7), None],
        ])
        .unwrap();

        let astar_state_result = AstarState::inital(puzzle_state, &DumbHeuristic);

        assert!(matches!(
            astar_state_result,
            Err(AstarStateError::InitialStateNotSolvable)
        ));
    }

    #[test]
    fn moved_to_neighbour() {
        let puzzle_state = PuzzleState::<BIGGER_PUZZLE_SIZE>::new([
            [Some(1), Some(2), Some(3)],
            [Some(4), Some(5), Some(6)],
            [Some(7), Some(8), None],
        ])
        .unwrap();

        let astar_state = AstarState::inital(puzzle_state, &DumbHeuristic).unwrap();
        let mut neighbours = astar_state.neighbours();

        let first_neighbour = neighbours.pop().unwrap();

        let (direction, puzzle_state) = first_neighbour.into_direction_and_puzzle_state();
        let AstarState {
            f_value,
            last_direction,
            distance_from_start,
            puzzle_state: _,
        } = astar_state.moved_to_neighbour(direction, puzzle_state, &DumbHeuristic);

        assert_eq!(37, f_value);
        assert_eq!(Some(Direction::Left), last_direction);
        assert_eq!(1, distance_from_start);

        let second_neighbour = neighbours.pop().unwrap();

        let (direction, puzzle_state) = second_neighbour.into_direction_and_puzzle_state();
        let AstarState {
            f_value,
            last_direction,
            distance_from_start,
            puzzle_state: _,
        } = astar_state.moved_to_neighbour(direction, puzzle_state, &DumbHeuristic);

        assert_eq!(37, f_value);
        assert_eq!(Some(Direction::Up), last_direction);
        assert_eq!(1, distance_from_start);
    }

    #[test]
    fn create_route_works() {
        let astar_state = AstarState {
            f_value: 0,
            last_direction: Some(Direction::Right),
            distance_from_start: 2,
            puzzle_state: PuzzleState::new([
                [Some(1), Some(2), Some(3)],
                [Some(4), Some(5), Some(6)],
                [Some(7), Some(8), None],
            ])
            .unwrap(),
        };

        let mut last_directions = HashMap::new();

        last_directions.insert(
            PuzzleState::new([
                [Some(1), Some(2), Some(3)],
                [Some(4), Some(5), Some(6)],
                [Some(7), None, Some(8)],
            ])
            .unwrap(),
            Some(Direction::Right),
        );

        last_directions.insert(
            PuzzleState::new([
                [Some(1), Some(2), Some(3)],
                [Some(4), Some(5), Some(6)],
                [None, Some(7), Some(8)],
            ])
            .unwrap(),
            None,
        );

        assert_eq!(
            vec![Direction::Right, Direction::Right],
            astar_state.create_route(&last_directions)
        );
    }
}
