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
use serde::{Deserialize, Serialize};

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
pub struct PuzzleSolution([char; 5]);

impl PuzzleSolution {
    pub fn new(str: &str) -> Result<Self, PuzzleWordError> {
        let vec: Vec<char> = str.chars().map(|c| c.to_ascii_lowercase()).collect();
        match vec.try_into() {
            Ok(arr) => Ok(Self(arr)),
            Err(_) => Err(PuzzleWordError::TooFewOrTooManyLetters(str.chars().count())),
        }
    }

    pub fn inner(&self) -> [char; 5] {
        self.0
    }
}

impl Display for PuzzleSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.map(|c| c.to_string()).join(""))
    }
}

#[derive(Debug)]
pub enum PuzzleWordError {
    TooFewOrTooManyLetters(usize),
}

impl Display for PuzzleWordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PuzzleWordError::TooFewOrTooManyLetters(count) => {
                if *count > 5 {
                    write!(f, "too many letters: {count}! must be 5")
                } else {
                    write!(f, "too few letters: {count}! must be 5")
                }
            }
        }
    }
}

impl std::error::Error for PuzzleWordError {}

impl ToSql<Text, Pg> for PuzzleSolution
where
    String: ToSql<Text, Pg>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        out.write_all(self.to_string().as_bytes());
        Ok(serialize::IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for PuzzleSolution
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let s = String::from_sql(bytes)?;
        PuzzleSolution::new(&s).map_err(|e| Box::from(format!("invalid date {s}: {e}")))
    }
}
