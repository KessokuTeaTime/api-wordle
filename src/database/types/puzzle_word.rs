use std::{fmt, io::Write as _};

use diesel::{
    backend::Backend,
    deserialize::{self, FromSql},
    expression::AsExpression,
    pg::Pg,
    serialize::{self, Output, ToSql},
    sql_types::{SqlType, Text},
};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, SqlType, AsExpression,
)]
#[diesel(sql_type = Text)]
pub struct PuzzleWord([char; 5]);

impl PuzzleWord {
    pub fn new(word: &str) -> Result<Self, PuzzleWordError> {
        let vec: Vec<char> = word.chars().into_iter().collect();
        match vec.try_into() {
            Ok(arr) => Ok(Self(arr)),
            Err(_) => Err(PuzzleWordError::TooFewOrTooManyLetters(
                word.chars().count(),
            )),
        }
    }

    pub fn inner(&self) -> [char; 5] {
        self.0
    }

    pub fn to_string(&self) -> String {
        self.0.map(|c| c.to_string()).join("")
    }
}

#[derive(Debug)]
pub enum PuzzleWordError {
    TooFewOrTooManyLetters(usize),
}

impl fmt::Display for PuzzleWordError {
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

impl ToSql<Text, Pg> for PuzzleWord
where
    String: ToSql<Text, Pg>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        out.write_all(self.to_string().as_bytes());
        Ok(serialize::IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for PuzzleWord
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let s = String::from_sql(bytes)?;
        PuzzleWord::new(&s).map_err(|e| Box::from(format!("invalid date {s}: {e}")))
    }
}
