use sea_orm::DeriveValueType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, DeriveValueType)]
pub struct SubmitHistory(String);

impl SubmitHistory {
    // pub fn new() -> Self {}
}
