//! Errors that can occur when working with `PuzzleState`.

/// Errors that can occur when creating [PuzzleState] instance.
#[derive(Debug)]
pub enum PuzzleStateCreationError {
    /// Provided `numbers` are not a valid permutation.
    NotPermutation,
    /// There is more than one `blank` in a permutation.
    TwoBlanks,
}

/// Errors that can occur when parsing [PuzzleState] instance.
#[derive(Debug)]
pub enum PuzzleStateParseError {
    /// No brackets around permutation.
    NoBrackets,
    /// Not enough numbers.
    NotEnoughNumbers,
    /// Not enough numbers.
    TooManyNumbers,
    /// Number parse error.
    NumberParseError,
    /// Provided `numbers` are not a valid permutation.
    NotPermutation,
    /// There is more than one `blank` in a permutation.
    TwoBlanks,
}

impl From<PuzzleStateCreationError> for PuzzleStateParseError {
    fn from(value: PuzzleStateCreationError) -> Self {
        match value {
            PuzzleStateCreationError::TwoBlanks => PuzzleStateParseError::TwoBlanks,
            PuzzleStateCreationError::NotPermutation => PuzzleStateParseError::NotPermutation,
        }
    }
}

