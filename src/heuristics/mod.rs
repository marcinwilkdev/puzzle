//! Heuristics for sliding puzzle A* solver.

pub mod disjoint_databases;
pub mod manhattan_distance;
pub mod dumb_heuristic;

pub use manhattan_distance::ManhattanDistance;
pub use disjoint_databases::DisjointDatabases;

/// Trait for declaring different heuristics.
pub trait Heuristic<const PUZZLE_SIZE: usize> {
    /// Function that calculates heuristic value for given numbers in puzzle state.
    fn calculate(&self, numbers: &[[Option<u8>; PUZZLE_SIZE]; PUZZLE_SIZE]) -> u8;
}
