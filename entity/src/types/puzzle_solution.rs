use crate::PUZZLE_LETTERS_COUNT;

use std::fmt::{self, Display};

use sea_orm::{
    ColIdx, ColumnType, DbErr, QueryResult, TryGetError, TryGetable, Value,
    prelude::StringLen,
    sea_query::{ArrayType, ValueType, ValueTypeErr},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor};

/// A puzzle solution consisting of exactly `N` ASCII alphabetic letters.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PuzzleSolution<const N: usize = PUZZLE_LETTERS_COUNT>(pub [char; N]);

impl<const N: usize> PuzzleSolution<N> {
    /// Returns the inner array of characters.
    pub fn inner(&self) -> &[char; N] {
        &self.0
    }

    /// Returns the length of the puzzle solution.
    pub fn len(&self) -> usize {
        N
    }

    /// Whether the puzzle solution is empty.
    pub fn is_empty(&self) -> bool {
        false
    }
}

impl<const N: usize> Display for PuzzleSolution<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.map(String::from).join(""))
    }
}

impl<const N: usize> TryFrom<&str> for PuzzleSolution<N> {
    type Error = PuzzleWordError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != N {
            return Err(PuzzleWordError::TooFewOrTooManyLetters {
                actual: value.len(),
                expected: N,
            });
        }

        if value.chars().all(|c| c.is_ascii_alphabetic()) {
            let chars: Vec<char> = value.chars().collect();
            Ok(Self(chars[..].try_into().unwrap()))
        } else {
            Err(PuzzleWordError::ContainsNonAsciiAlphabeticLetters)
        }
    }
}

impl<const N: usize> Serialize for PuzzleSolution<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, const N: usize> Deserialize<'de> for PuzzleSolution<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PuzzleSolutionVisitor::<N>)
    }
}

impl<const N: usize> From<PuzzleSolution<N>> for Value {
    fn from(value: PuzzleSolution<N>) -> Self {
        Self::String(Some(Box::new(value.to_string())))
    }
}

impl<const N: usize> ValueType for PuzzleSolution<N> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(string)) => {
                <Self as TryFrom<&str>>::try_from(&string).map_err(|_| ValueTypeErr)
            }
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(PuzzleSolution<N>).to_owned()
    }

    fn array_type() -> ArrayType {
        ArrayType::String
    }

    fn column_type() -> ColumnType {
        ColumnType::String(StringLen::N(N.try_into().unwrap_or(u32::MAX)))
    }
}

impl<const N: usize> TryGetable for PuzzleSolution<N> {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let value: String = res.try_get_by(index).map_err(TryGetError::DbErr)?;
        <Self as TryFrom<&str>>::try_from(&value[..])
            .map_err(|e| TryGetError::DbErr(DbErr::Custom(e.to_string())))
    }
}

struct PuzzleSolutionVisitor<const N: usize>;

impl<const N: usize> Visitor<'_> for PuzzleSolutionVisitor<N> {
    type Value = PuzzleSolution<N>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&format!("a string of length {N}"))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match <Self::Value as TryFrom<&str>>::try_from(v) {
            Ok(value) => Ok(value),
            Err(err) => Err(E::custom(err.to_string())),
        }
    }
}

/// The errors that can occur when parsing a [`PuzzleSolution`].
#[derive(Debug)]
#[non_exhaustive]
pub enum PuzzleWordError {
    /// The puzzle solution has too few or too many letters.
    TooFewOrTooManyLetters {
        /// The actual number of letters.
        actual: usize,
        /// The expected number of letters.
        expected: usize,
    },
    /// The puzzle solution contains non ASCII alphabetic letters.
    ContainsNonAsciiAlphabeticLetters,
}

impl Display for PuzzleWordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooFewOrTooManyLetters { actual, expected } => {
                if *actual > *expected {
                    write!(f, "too many letters: {actual}, must be {expected}")
                } else {
                    write!(f, "too few letters: {actual}, must be {expected}")
                }
            }
            Self::ContainsNonAsciiAlphabeticLetters => {
                write!(f, "cannot contain non ascii alphabetic letters!")
            }
        }
    }
}

impl std::error::Error for PuzzleWordError {}
