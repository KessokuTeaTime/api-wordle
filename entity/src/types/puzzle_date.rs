use std::fmt::{self, Display};

use sea_orm::{DeriveValueType, TryFromU64, prelude::Date};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, DeriveValueType)]
pub struct PuzzleDate(Date);

impl PuzzleDate {
    const MIN_DATE: Date = Date::from_ymd_opt(1970, 1, 1).unwrap();

    pub fn new(date_str: &str) -> Result<Self, PuzzleDateError> {
        let date = Date::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|_| PuzzleDateError::InvalidFormat)?;

        if date >= Self::MIN_DATE {
            Ok(Self(date))
        } else {
            Err(PuzzleDateError::TooEarly)
        }
    }

    pub fn inner(&self) -> Date {
        self.0
    }
}

impl Display for PuzzleDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d"))
    }
}

impl TryFromU64 for PuzzleDate {
    fn try_from_u64(n: u64) -> Result<Self, sea_orm::DbErr> {
        Ok(Self(Date::try_from_u64(n)?))
    }
}

#[derive(Debug)]
pub enum PuzzleDateError {
    InvalidFormat,
    TooEarly,
}

impl Display for PuzzleDateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat => write!(f, "the date must be formatted as YYYY-MM-DD"),
            Self::TooEarly => write!(f, "the date cannot be earlier than 1970-01-01"),
        }
    }
}

impl std::error::Error for PuzzleDateError {}
