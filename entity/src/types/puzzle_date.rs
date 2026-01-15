use std::fmt::{self, Display};

use sea_orm::{DeriveValueType, TryFromU64, prelude::Date};
use serde::{Deserialize, Serialize};

/// A valid puzzle date.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, DeriveValueType)]
pub struct PuzzleDate(pub Date);

impl PuzzleDate {
    /// The minimum valid date: 1970-01-01.
    pub const MIN_DATE: Date = Date::from_ymd_opt(1970, 1, 1).unwrap();
    /// The minimum valid puzzle date.
    pub const MIN: Self = Self(Self::MIN_DATE);

    /// Creates a new [`PuzzleDate`].
    pub fn new(date: Date) -> Self {
        Self(date)
    }

    /// Returns the inner [`Date`].
    pub fn inner(&self) -> Date {
        self.0
    }
}

impl Display for PuzzleDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d"))
    }
}

impl TryFrom<&str> for PuzzleDate {
    type Error = PuzzleDateError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let date =
            Date::parse_from_str(value, "%Y-%m-%d").map_err(|_| PuzzleDateError::InvalidFormat)?;

        if date >= Self::MIN_DATE {
            Ok(Self(date))
        } else {
            Err(PuzzleDateError::TooEarly)
        }
    }
}

impl TryFromU64 for PuzzleDate {
    fn try_from_u64(n: u64) -> Result<Self, sea_orm::DbErr> {
        Ok(Self(Date::try_from_u64(n)?))
    }
}

/// The errors that can occur when parsing a [`PuzzleDate`].
#[derive(Debug)]
#[non_exhaustive]
pub enum PuzzleDateError {
    /// The date format is invalid.
    InvalidFormat,
    /// The date is earlier than [`PuzzleDate::MIN`].
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
