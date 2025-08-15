use sea_orm::{
    ColIdx, ColumnType, QueryResult, TryGetError, TryGetable, TryGetableFromJson, Value,
    sea_query::{ArrayType, Nullable, ValueType, ValueTypeErr},
};
use serde::{Deserialize, Serialize};

use crate::SubmitWord;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmitHistory<const N: usize>(pub Vec<SubmitWord<N>>);

impl<const N: usize> SubmitHistory<N> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn submit(&mut self, word: SubmitWord<N>) {
        self.0.push(word);
    }
}

impl<const N: usize> From<SubmitHistory<N>> for Value {
    fn from(value: SubmitHistory<N>) -> Self {
        Value::Json(serde_json::to_value(&value).ok().map(Box::new))
    }
}

impl<const N: usize> ValueType for SubmitHistory<N> {
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

impl<const N: usize> TryGetableFromJson for SubmitHistory<N> {}

impl<const N: usize> Nullable for SubmitHistory<N> {
    fn null() -> Value {
        Value::Json(None)
    }
}
