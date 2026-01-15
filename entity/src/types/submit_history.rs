use crate::{HISTORY_MAX_TRIES, PUZZLE_LETTERS_COUNT, SubmitWord};

use std::fmt::Display;

use sea_orm::{
    ColumnType, TryGetableFromJson, Value,
    sea_query::{ArrayType, Nullable, ValueType, ValueTypeErr},
};
use serde::{Deserialize, Serialize};

/// The submit history of a puzzle, where `N` is the number of letters in the puzzle
/// and `MAX` is the maximum number of submission attempts allowed.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmitHistory<
    const N: usize = PUZZLE_LETTERS_COUNT,
    const MAX: usize = HISTORY_MAX_TRIES,
>(pub Vec<SubmitWord<N>>);

impl<const N: usize, const MAX: usize> SubmitHistory<N, MAX> {
    /// Creates a new empty [`SubmitHistory`].
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Returns the number of submissions made.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns the number of letters in the puzzle.
    pub fn letters_count(&self) -> usize {
        N
    }

    /// Returns the number of remaining tries. That is, `MAX - len()`.
    pub fn remaining_tries(&self) -> usize {
        MAX - self.len()
    }

    /// Returns whether the submit history is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns whether the submit history is full. That is, whether `len() >= MAX`.
    pub fn is_full(&self) -> bool {
        self.0.len() >= MAX
    }

    /// Submits a new word to the history.
    ///
    /// # Errors
    ///
    /// Returns a [`SubmitHistoryError::TooManyTimes`] error if the submit history is already full.
    pub fn submit(&mut self, word: SubmitWord<N>) -> Result<(), SubmitHistoryError> {
        if self.is_full() {
            Err(SubmitHistoryError::TooManyTimes { max: MAX })
        } else {
            self.0.push(word);
            Ok(())
        }
    }

    /// Consumes the submit history and returns the inner vector of submitted words.
    pub fn into_vec(self) -> Vec<SubmitWord<N>> {
        self.0
    }
}

impl<const N: usize, const MAX: usize> From<SubmitHistory<N, MAX>> for Value {
    fn from(value: SubmitHistory<N, MAX>) -> Self {
        Self::Json(serde_json::to_value(&value).ok().map(Box::new))
    }
}

impl<const N: usize, const MAX: usize> ValueType for SubmitHistory<N, MAX> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Json(Some(json)) => serde_json::from_value(*json).map_err(|_| ValueTypeErr),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(SubmitWords).to_owned()
    }

    fn array_type() -> ArrayType {
        ArrayType::Json
    }

    fn column_type() -> ColumnType {
        ColumnType::JsonBinary
    }
}

impl<const N: usize, const MAX: usize> TryGetableFromJson for SubmitHistory<N, MAX> {}

impl<const N: usize, const MAX: usize> Nullable for SubmitHistory<N, MAX> {
    fn null() -> Value {
        Value::Json(None)
    }
}

/// The errors that can occur when submitting a word to a [`SubmitHistory`].
#[derive(Debug)]
#[non_exhaustive]
pub enum SubmitHistoryError {
    /// The submit history has reached the maximum number of submission attempts.
    TooManyTimes {
        /// The maximum number of submission attempts allowed.
        max: usize,
    },
}

impl Display for SubmitHistoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooManyTimes { max } => write!(
                f,
                "submitted for too many times, exceeding the maximum constraint of {}",
                max
            ),
        }
    }
}

impl std::error::Error for SubmitHistoryError {}
