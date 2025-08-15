use std::fmt::{self, Display};

use sea_orm::DeriveValueType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, DeriveValueType)]
pub struct PuzzleSolution(pub String);

impl PuzzleSolution {
    pub fn inner(&self) -> &str {
        &self.0
    }
}

impl Display for PuzzleSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for PuzzleSolution {
    type Error = PuzzleWordError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 5 {
            return Err(PuzzleWordError::TooFewOrTooManyLetters(value.len()));
        }

        if value.chars().all(|c| c.is_ascii_alphabetic()) {
            Ok(Self(value.to_lowercase().to_owned()))
        } else {
            Err(PuzzleWordError::ContainsNonAsciiAlphabeticLetters)
        }
    }
}

#[derive(Debug)]
pub enum PuzzleWordError {
    TooFewOrTooManyLetters(usize),
    ContainsNonAsciiAlphabeticLetters,
}

impl Display for PuzzleWordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooFewOrTooManyLetters(count) => {
                if *count > 5 {
                    write!(f, "too many letters: {count}! must be 5")
                } else {
                    write!(f, "too few letters: {count}! must be 5")
                }
            }
            Self::ContainsNonAsciiAlphabeticLetters => {
                write!(f, "cannot contain non ascii alphabetic letters!!")
            }
        }
    }
}

impl std::error::Error for PuzzleWordError {}
