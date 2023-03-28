/*!
* Utilities for performing parity check on permutation contained in puzzle state.
* Such parity check is used to determine if sliding puzzle game from this state is solvable.
 */

use std::collections::HashSet;

/// Struct used to check parity of [PuzzleState] permutation.
#[derive(Debug)]
pub struct ParityCheckPermutation {
    permutation: Vec<u8>,
}

impl ParityCheckPermutation {
    /// Creates new instance of [ParityCheckPermutation] for provided `numbers`.
    pub fn from_numbers<const PUZZLE_SIZE: usize>(
        numbers: &[[Option<u8>; PUZZLE_SIZE]; PUZZLE_SIZE],
    ) -> Self {
        let puzzle_size = numbers.len() as u8;
        let mut permutation = Vec::with_capacity((puzzle_size * puzzle_size) as usize);

        for number_row in numbers {
            for number in number_row {
                if let Some(number_value) = number {
                    permutation.push(*number_value);
                } else {
                    permutation.push(puzzle_size * puzzle_size);
                }
            }
        }

        ParityCheckPermutation { permutation }
    }

    /// Checks if permutation is even.
    pub fn is_even(&self) -> bool {
        let mut not_checked_numbers: HashSet<_> = (1..=self.permutation.len()).collect();
        let mut sum_swaps = 0;

        while !not_checked_numbers.is_empty() {
            // not_checked can't be empty
            let cycle_beginning = *not_checked_numbers.iter().next().unwrap();
            not_checked_numbers.remove(&cycle_beginning);

            let mut curr_number_index = cycle_beginning - 1;

            while self.permutation[curr_number_index] != (cycle_beginning as u8) {
                let curr_number = self.permutation[curr_number_index];
                not_checked_numbers.remove(&(curr_number as usize));

                curr_number_index = (curr_number - 1) as usize;
                sum_swaps += 1;
            }
        }

        (sum_swaps % 2) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_permutation_from_numbers() {
        let numbers = [[Some(1), Some(2)], [Some(3), None]];

        let parity_check_permutation = ParityCheckPermutation::from_numbers(&numbers);

        assert_eq!(&vec![1, 2, 3, 4], &parity_check_permutation.permutation);

        let numbers = [[Some(2), Some(1)], [Some(3), None]];

        let parity_check_permutation = ParityCheckPermutation::from_numbers(&numbers);

        assert_eq!(&vec![2, 1, 3, 4], &parity_check_permutation.permutation);
    }

    #[test]
    fn correct_evenness_of_permutations() {
        let numbers = [[Some(1), Some(2)], [Some(3), None]];

        let parity_check_permutation = ParityCheckPermutation::from_numbers(&numbers);

        assert!(parity_check_permutation.is_even());

        let numbers = [[Some(2), Some(1)], [Some(3), None]];

        let parity_check_permutation = ParityCheckPermutation::from_numbers(&numbers);

        assert!(!parity_check_permutation.is_even());

        let numbers = [
            [Some(1), Some(2), Some(4), Some(3)],
            [Some(5), Some(6), Some(7), Some(8)],
            [Some(9), Some(10), Some(11), Some(12)],
            [Some(13), Some(14), None, Some(15)],
        ];

        let parity_check_permutation = ParityCheckPermutation::from_numbers(&numbers);

        assert!(parity_check_permutation.is_even());
    }
}
