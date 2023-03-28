//! Database containing route lenghts to subset of numbers from 15 puzzle game.

use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

use super::bfs_state::BFSState;
use super::combination::Combination;

/// Each database contains all possible combinations of 4 elements with distance of that
/// combination from solution.
#[derive(Serialize, Deserialize)]
pub struct Database {
    distances: HashMap<Combination, u8>,
}

impl Database {
    /// Creates new instance of [Database].
    pub fn new(database_first_element_index: usize, ignore_last: bool) -> Database {
        let mut distances = HashMap::new();
        let mut visited = HashSet::new();
        let mut frontier = BinaryHeap::new();

        let initial_state = BFSState::initial(database_first_element_index, ignore_last);

        visited.insert(initial_state.board_state());
        frontier.push(Reverse(initial_state));

        while !frontier.is_empty() {
            let curr_bfs_state = frontier.pop().expect("Frontier can't be empty").0;
            let (combination, distance) = curr_bfs_state.combination_and_distance();

            if !distances.contains_key(&combination) {
                distances.insert(combination, distance);
            }

            let neighbours = curr_bfs_state.neighbours();

            for neighbour in neighbours {
                if visited.insert(neighbour.board_state()) {
                    frontier.push(Reverse(neighbour));
                }
            }
        }

        Database { distances }
    }

    /// Returns distance for given combination.
    pub fn get_distance(&self, combination: &Combination) -> Option<&u8> {
        self.distances.get(combination)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn database_creation_works() {
        let database = Database::new(0, false);

        assert_eq!(16 * 15 * 14 * 13, database.distances.len());
    }

    #[test]
    fn database_creation_ignore_last_works() {
        let database = Database::new(12, true);

        assert_eq!(16 * 15 * 14, database.distances.len());
    }
}
