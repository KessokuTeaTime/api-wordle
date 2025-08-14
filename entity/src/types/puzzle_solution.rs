use std::fmt::{self, Display};

use sea_orm::DeriveValueType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, DeriveValueType)]
pub struct PuzzleSolution(String);

impl PuzzleSolution {
    pub fn new(str: &str) -> Result<Self, PuzzleWordError> {
        if str.len() != 5 {
            return Err(PuzzleWordError::TooFewOrTooManyLetters(str.len()));
        }

        if str.chars().all(|c| c.is_ascii_alphabetic()) {
            Ok(Self(str.to_lowercase().to_owned()))
        } else {
            Err(PuzzleWordError::ContainsNonAsciiAlphabeticLetters)
        }
    }

    pub fn inner(&self) -> &str {
        &self.0
    }
}

impl Display for PuzzleSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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
