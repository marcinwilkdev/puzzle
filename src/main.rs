use clap::{Parser, ValueEnum};

use puzzle::heuristics::{DisjointDatabases, Heuristic, ManhattanDistance};
use puzzle::PuzzleState;

/// Available heuristics
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum AvailableHeuristics {
    /// Manhattan Distance heuristic
    ManhattanDistance,
    /// Disjoint Databases heuristic
    DisjointDatabases,
}

#[derive(Parser)]
struct Cli {
    /// Heuristic to use.
    #[arg(long)]
    heuristic: AvailableHeuristics,

    /// Initial puzzle state
    puzzle_state: Option<String>,
}

const PUZZLE_SIZE: usize = 4;
const MAX_STEPS_BACK: usize = 100;

fn main() {
    let cli = Cli::parse();

    let used_heuristic: Box<dyn Heuristic<PUZZLE_SIZE>> = match cli.heuristic {
        AvailableHeuristics::ManhattanDistance => Box::new(ManhattanDistance::new()),
        AvailableHeuristics::DisjointDatabases => Box::new(DisjointDatabases::new(false)),
    };

    let initial_puzzle_state = if let Some(puzzle_state) = cli.puzzle_state {
        puzzle_state
            .parse::<PuzzleState<PUZZLE_SIZE>>()
            // TODO: print appropriate errors
            .expect("Couldn't parse puzzle state")
    } else {
        puzzle::generate_random_puzzle_state(MAX_STEPS_BACK)
    };

    println!("Initial puzzle state: {initial_puzzle_state}");

    let solution = puzzle::solve_with_heuristic(initial_puzzle_state, &*used_heuristic);

    if let Some(solution) = solution {
        let solution_steps = solution.steps();
        let no_of_visited_states = solution.no_of_visited_states();

        println!("Solution steps: {solution_steps:?}");
        println!("Solution len: {}", solution_steps.len());
        println!("Number of visited states: {no_of_visited_states:?}");
    } else {
        println!("State unsolvable.");
    }
}
