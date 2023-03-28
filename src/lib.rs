/*!
* Solver for [sliding puzzle](https://en.wikipedia.org/wiki/Sliding_puzzle) game using
* [A*](https://en.wikipedia.org/wiki/A*_search_algorithm) with heuristics.
* Max game size supported is 4.
* TODO: Create better documentation.
*/

pub mod astar_state;
pub mod generator;
pub mod heuristics;
pub mod puzzle_state;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use astar_state::AstarState;
use heuristics::Heuristic;

pub use generator::generate_random_puzzle_state;
pub use puzzle_state::direction::Direction;
pub use puzzle_state::PuzzleState;

/// Most common used puzzle size.
pub const DEFAULT_PUZZLE_SIZE: usize = 4;

/// Solution to sliding puzzle game.
pub struct Solution {
    steps: Vec<Direction>,
    no_of_visited_states: usize,
}

impl Solution {
    /// Creates new instance of [Solution].
    pub fn new(steps: Vec<Direction>, no_of_visited_states: usize) -> Self {
        Solution {
            steps,
            no_of_visited_states,
        }
    }

    /// Accessor for `steps` field.
    pub fn steps(&self) -> &[Direction] {
        &self.steps
    }

    /// Accessor for `no_of_visited_states` field.
    pub fn no_of_visited_states(&self) -> usize {
        self.no_of_visited_states
    }
}

/**
* Solves sliding puzzle game using given heuristic in A* algorithm.
* Returns `Some(result)` if there exists solution or `None` if not.
*/
pub fn solve_with_heuristic(
    initial_state: PuzzleState<DEFAULT_PUZZLE_SIZE>,
    heuristic: &dyn Heuristic<DEFAULT_PUZZLE_SIZE>,
) -> Option<Solution> {
    let mut curr_state = AstarState::inital(initial_state, heuristic).ok()?;
    let mut last_directions = HashMap::new();
    let mut frontier = BinaryHeap::new();

    // So we can pop something in first iteration.
    frontier.push(Reverse(curr_state.clone()));

    while !curr_state.is_solved() {
        // There have to be elements in frontier if not solved yet.
        curr_state = frontier.pop().unwrap().0;

        let state_not_visited = last_directions.get(&curr_state.puzzle_state()).is_none();

        if state_not_visited {
            last_directions.insert(curr_state.puzzle_state(), curr_state.last_direction());

            let neighbours = curr_state.neighbours();

            for neighbour in neighbours {
                let (direction, puzzle_state) = neighbour.into_direction_and_puzzle_state();

                if last_directions.get(&puzzle_state).is_none() {
                    let moved_to_neighbour_state =
                        curr_state.moved_to_neighbour(direction, puzzle_state, heuristic);

                    frontier.push(Reverse(moved_to_neighbour_state));
                }
            }
        }
    }

    let solution = Solution::new(
        curr_state.create_route(&last_directions),
        last_directions.len(),
    );

    Some(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    use heuristics::disjoint_databases::DisjointDatabases;
    use heuristics::manhattan_distance::ManhattanDistance;

    #[test]
    fn solve_on_solved_works() {
        let manhattan_distance = ManhattanDistance::new();

        let puzzle_state = PuzzleState::<DEFAULT_PUZZLE_SIZE>::new([
            [Some(1), Some(2), Some(3), Some(4)],
            [Some(5), Some(6), Some(7), Some(8)],
            [Some(9), Some(10), Some(11), Some(12)],
            [Some(13), Some(14), Some(15), None],
        ])
        .unwrap();

        let solution = solve_with_heuristic(puzzle_state, &manhattan_distance);

        assert!(solution.is_some());
        assert_eq!(Vec::<Direction>::new(), solution.unwrap().steps());
    }

    #[test]
    fn solve_on_unsolvable() {
        let manhattan_distance = ManhattanDistance::new();

        let puzzle_state = PuzzleState::<DEFAULT_PUZZLE_SIZE>::new([
            [Some(1), Some(2), Some(4), Some(3)],
            [Some(5), Some(6), Some(7), Some(8)],
            [Some(9), Some(10), Some(11), Some(12)],
            [Some(13), Some(14), Some(15), None],
        ])
        .unwrap();

        let solution = solve_with_heuristic(puzzle_state, &manhattan_distance);

        assert!(solution.is_none());
    }

    #[test]
    fn solving_with_manhattan_distance_works() {
        let manhattan_distance = ManhattanDistance::new();

        let puzzle_state = PuzzleState::<DEFAULT_PUZZLE_SIZE>::new([
            [Some(1), Some(2), Some(3), Some(4)],
            [Some(5), Some(6), Some(7), Some(8)],
            [Some(9), Some(10), Some(11), Some(12)],
            [Some(13), Some(14), None, Some(15)],
        ])
        .unwrap();

        let solution = solve_with_heuristic(puzzle_state, &manhattan_distance);

        assert!(solution.is_some());
        assert_eq!(vec![Direction::Right], solution.unwrap().steps());

        let puzzle_state = PuzzleState::<DEFAULT_PUZZLE_SIZE>::new([
            [None, Some(2), Some(3), Some(4)],
            [Some(1), Some(6), Some(7), Some(8)],
            [Some(5), Some(10), Some(11), Some(12)],
            [Some(9), Some(13), Some(14), Some(15)],
        ])
        .unwrap();

        let solution = solve_with_heuristic(puzzle_state, &manhattan_distance);

        assert!(solution.is_some());
        assert_eq!(
            vec![
                Direction::Down,
                Direction::Down,
                Direction::Down,
                Direction::Right,
                Direction::Right,
                Direction::Right
            ],
            solution.unwrap().steps()
        );
    }

    #[test]
    fn solving_with_disjoint_databases_works() {
        let disjoint_databases = DisjointDatabases::new(false);

        let puzzle_state = PuzzleState::<DEFAULT_PUZZLE_SIZE>::new([
            [Some(1), Some(2), Some(3), Some(4)],
            [Some(5), Some(6), Some(7), Some(8)],
            [Some(9), Some(10), Some(11), Some(12)],
            [Some(13), Some(14), None, Some(15)],
        ])
        .unwrap();

        let solution = solve_with_heuristic(puzzle_state, &disjoint_databases);

        assert!(solution.is_some());
        assert_eq!(vec![Direction::Right], solution.unwrap().steps());

        let puzzle_state = PuzzleState::<DEFAULT_PUZZLE_SIZE>::new([
            [None, Some(2), Some(3), Some(4)],
            [Some(1), Some(6), Some(7), Some(8)],
            [Some(5), Some(10), Some(11), Some(12)],
            [Some(9), Some(13), Some(14), Some(15)],
        ])
        .unwrap();

        let solution = solve_with_heuristic(puzzle_state, &disjoint_databases);

        assert!(solution.is_some());
        assert_eq!(
            vec![
                Direction::Down,
                Direction::Down,
                Direction::Down,
                Direction::Right,
                Direction::Right,
                Direction::Right
            ],
            solution.unwrap().steps()
        );
    }
}
