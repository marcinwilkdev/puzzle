/// Used to generate statistics for different heuristics.
use std::time::Instant;

use z1::heuristics::{DisjointDatabases, ManhattanDistance};

const PUZZLE_SIZE: usize = 4;
const NO_OF_ITERATIONS: usize = 100;

const MAX_STEPS_BACK_START: usize = 10;
const MAX_STEPS_BACK_STEP: usize = 5;
const MAX_STEPS_BACK_STEPS: usize = 13;

fn main() {
    let manhattan_distance = ManhattanDistance::<PUZZLE_SIZE>::new();
    let disjoint_databases = DisjointDatabases::new(false);

    println!("Heuristic | Solution length | Visited states | Runtime");

    for steps_increment in 0..MAX_STEPS_BACK_STEPS {
        let steps = MAX_STEPS_BACK_START + (steps_increment * MAX_STEPS_BACK_STEP);

        for _ in 0..NO_OF_ITERATIONS {
            let random_state = z1::generate_random_puzzle_state(steps);

            let md_start_time = Instant::now();
            let md_solution = z1::solve_with_heuristic(random_state, &manhattan_distance);
            let md_runtime = Instant::now() - md_start_time;

            let dd_start_time = Instant::now();
            let dd_solution = z1::solve_with_heuristic(random_state, &disjoint_databases);
            let dd_runtime = Instant::now() - dd_start_time;

            if let (Some(md_solution), Some(dd_solution)) = (md_solution, dd_solution) {
                println!(
                    "MD: {} {} {}",
                    md_solution.steps().len(),
                    md_solution.no_of_visited_states(),
                    md_runtime.as_millis()
                );

                println!(
                    "DD: {} {} {}",
                    dd_solution.steps().len(),
                    dd_solution.no_of_visited_states(),
                    dd_runtime.as_millis()
                );
            }
        }
    }
}
