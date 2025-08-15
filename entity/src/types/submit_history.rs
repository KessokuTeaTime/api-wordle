use std::fmt::Display;

use sea_orm::{
    ColumnType, TryGetableFromJson, Value,
    sea_query::{ArrayType, Nullable, ValueType, ValueTypeErr},
};
use serde::{Deserialize, Serialize};

use crate::SubmitWord;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmitHistory<const N: usize, const MAX: usize>(pub Vec<SubmitWord<N>>);

impl<const N: usize, const MAX: usize> SubmitHistory<N, MAX> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn submit(&mut self, word: SubmitWord<N>) -> Result<(), SubmitHistoryError> {
        if self.0.len() < MAX {
            Ok(self.0.push(word))
        } else {
            Err(SubmitHistoryError::TooManyTimes { max: MAX })
        }
    }
}

impl<const N: usize, const MAX: usize> From<SubmitHistory<N, MAX>> for Value {
    fn from(value: SubmitHistory<N, MAX>) -> Self {
        Value::Json(serde_json::to_value(&value).ok().map(Box::new))
    }
}

impl<const N: usize, const MAX: usize> ValueType for SubmitHistory<N, MAX> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Json(Some(json)) => Ok(serde_json::from_value(*json).map_err(|_| ValueTypeErr)?),
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

#[derive(Debug)]
pub enum SubmitHistoryError {
    TooManyTimes { max: usize },
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
