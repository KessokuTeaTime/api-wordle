use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display},
    io::Write as _,
};

use diesel::{
    backend::Backend,
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    serialize::{self, Output, ToSql},
    sql_types::{SqlType, Text},
};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    SqlType,
    AsExpression,
    FromSqlRow,
)]
#[diesel(sql_type = Text)]
pub struct PuzzleDate(NaiveDate);

impl PuzzleDate {
    const MIN_DATE: NaiveDate = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();

    pub fn new(date_str: &str) -> Result<Self, PuzzleDateError> {
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|_| PuzzleDateError::InvalidFormat)?;

        if date >= Self::MIN_DATE {
            Ok(Self(date))
        } else {
            Err(PuzzleDateError::TooEarly)
        }
    }

    pub fn inner(&self) -> NaiveDate {
        self.0
    }
}

impl Display for PuzzleDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d"))
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
            PuzzleDateError::InvalidFormat => write!(f, "the date must be formatted as YYYY-MM-DD"),
            PuzzleDateError::TooEarly => write!(f, "the date cannot be earlier than 1970-01-01"),
        }
    }
}

impl std::error::Error for PuzzleDateError {}

impl ToSql<Text, Pg> for PuzzleDate
where
    String: ToSql<Text, Pg>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        out.write_all(self.to_string().as_bytes());
        Ok(serialize::IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for PuzzleDate
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let s = String::from_sql(bytes)?;
        PuzzleDate::new(&s).map_err(|e| Box::from(format!("invalid date {s}: {e}")))
    }
}
