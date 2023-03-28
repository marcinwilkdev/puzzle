//! Disjoint databases heuristic (works only for 15 puzzle).

pub mod bfs_state;
pub mod board_state;
pub mod combination;
pub mod database;

use std::fs::File;

use serde::{Deserialize, Serialize};

use crate::heuristics::Heuristic;
use crate::puzzle_state::coordinates::BoardCoordinates;

use combination::Combination;
use database::Database;

pub const DATABASE_SIZE: usize = 4;
pub const DATABASES_COUNT: usize = 4;
pub const PUZZLE_SIZE: usize = 4;

const DATABASE_PATH: &'static str = "15_puzzle_heuristic_database.data";

/**
* Disjoint databases heurstic works by splitting problem into many subproblems and calculating
* distances for each one of them.
*/
#[derive(Deserialize, Serialize)]
pub struct DisjointDatabases {
    databases: Vec<Database>,
}

impl DisjointDatabases {
    /// Reads instance of [DisjointDatabases] from disk or creates new if can't read.
    pub fn new(generate_fresh_databases: bool) -> DisjointDatabases {
        if generate_fresh_databases {
            return Self::create_fresh_instance();
        }

        let database_file = File::open(DATABASE_PATH);

        if let Ok(database_file) = database_file {
            let deserialize_result = serde_cbor::from_reader(&database_file);

            if let Ok(disjoint_databases) = deserialize_result {
                disjoint_databases
            } else {
                Self::create_fresh_instance()
            }
        } else {
            Self::create_fresh_instance()
        }
    }

    /// Creates new instance of [DisjointDatabases] and tries to save it to disk.
    fn create_fresh_instance() -> DisjointDatabases {
        let mut databases = vec![];

        for database_index in 0..DATABASES_COUNT {
            let database_first_element_index = database_index * DATABASES_COUNT;
            let ignore_last = database_index == (DATABASES_COUNT - 1);

            databases.push(Database::new(database_first_element_index, ignore_last));
        }

        let disjoint_databases = DisjointDatabases { databases };

        let database_file = File::create(DATABASE_PATH);

        if let Ok(database_file) = database_file {
            let _ = serde_cbor::to_writer(database_file, &disjoint_databases);
        }

        disjoint_databases
    }
}

impl Heuristic<PUZZLE_SIZE> for DisjointDatabases {
    fn calculate(&self, numbers: &[[Option<u8>; PUZZLE_SIZE]; PUZZLE_SIZE]) -> u8 {
        let mut numbers_representation = [
            [BoardCoordinates::<PUZZLE_SIZE>::new(0, 0); DATABASE_SIZE],
            [BoardCoordinates::<PUZZLE_SIZE>::new(0, 0); DATABASE_SIZE],
            [BoardCoordinates::<PUZZLE_SIZE>::new(0, 0); DATABASE_SIZE],
            [BoardCoordinates::<PUZZLE_SIZE>::new(0, 0); DATABASE_SIZE],
        ];

        // Indexes mean coordinates which will be filled in numbers_representation
        for (row_index, numbers_row) in numbers.iter().enumerate() {
            for (column_index, number) in numbers_row.iter().enumerate() {
                if let Some(number_value) = number {
                    let number_index = number_value - 1;

                    let number_row = number_index / (PUZZLE_SIZE as u8);
                    let number_column = number_index % (PUZZLE_SIZE as u8);

                    numbers_representation[number_row as usize][number_column as usize] =
                        BoardCoordinates::new(row_index as u8, column_index as u8);
                }
            }
        }

        let mut distance = 0;

        for (numbers_row_index, numbers_row) in numbers_representation.iter().enumerate() {
            let curr_database = &self.databases[numbers_row_index];
            let ignore_last = numbers_row_index == (DATABASE_SIZE - 1);
            let combination = Combination::from_readable(*numbers_row, ignore_last);

            distance += curr_database
                .get_distance(&combination)
                .expect("Database has to contain distance for this combination.");
        }

        distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::puzzle_state::PuzzleState;

    const BIGGER_PUZZLE_SIZE: usize = 4;

    #[test]
    fn databases_created_correctly() {
        let disjoint_databases = DisjointDatabases::new(false);

        assert_eq!(4, disjoint_databases.databases.len());

        assert!(disjoint_databases.databases[0]
            .get_distance(&Combination::from_readable(
                [
                    BoardCoordinates::<PUZZLE_SIZE>::new(0, 0),
                    BoardCoordinates::<PUZZLE_SIZE>::new(0, 1),
                    BoardCoordinates::<PUZZLE_SIZE>::new(0, 2),
                    BoardCoordinates::<PUZZLE_SIZE>::new(0, 3),
                ],
                false
            ))
            .is_some());

        assert!(disjoint_databases.databases[1]
            .get_distance(&Combination::from_readable(
                [
                    BoardCoordinates::<PUZZLE_SIZE>::new(1, 0),
                    BoardCoordinates::<PUZZLE_SIZE>::new(1, 1),
                    BoardCoordinates::<PUZZLE_SIZE>::new(1, 2),
                    BoardCoordinates::<PUZZLE_SIZE>::new(1, 3),
                ],
                false
            ))
            .is_some());

        assert!(disjoint_databases.databases[2]
            .get_distance(&Combination::from_readable(
                [
                    BoardCoordinates::<PUZZLE_SIZE>::new(2, 0),
                    BoardCoordinates::<PUZZLE_SIZE>::new(2, 1),
                    BoardCoordinates::<PUZZLE_SIZE>::new(2, 2),
                    BoardCoordinates::<PUZZLE_SIZE>::new(2, 3),
                ],
                false
            ))
            .is_some());

        assert!(disjoint_databases.databases[3]
            .get_distance(&Combination::from_readable(
                [
                    BoardCoordinates::<PUZZLE_SIZE>::new(3, 0),
                    BoardCoordinates::<PUZZLE_SIZE>::new(3, 1),
                    BoardCoordinates::<PUZZLE_SIZE>::new(3, 2),
                    BoardCoordinates::<PUZZLE_SIZE>::new(0, 0),
                ],
                true
            ))
            .is_some());
    }

    #[test]
    fn heuristic_works() {
        let disjoint_databases = DisjointDatabases::new(false);

        let puzzle_state = PuzzleState::<BIGGER_PUZZLE_SIZE>::new([
            [Some(1), Some(2), Some(3), Some(4)],
            [Some(5), Some(6), Some(7), Some(8)],
            [Some(9), Some(10), Some(11), Some(12)],
            [Some(13), Some(14), Some(15), None],
        ])
        .unwrap();

        let heuristic_value = puzzle_state.calculate_heuristic(&disjoint_databases);

        assert_eq!(0, heuristic_value);

        let puzzle_state = PuzzleState::<BIGGER_PUZZLE_SIZE>::new([
            [Some(1), Some(2), Some(3), Some(4)],
            [Some(5), Some(6), None, Some(8)],
            [Some(9), Some(10), Some(7), Some(12)],
            [Some(13), Some(14), Some(11), Some(15)],
        ])
        .unwrap();

        let heuristic_value = puzzle_state.calculate_heuristic(&disjoint_databases);

        assert_eq!(3, heuristic_value);
    }
}
