use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Matched {
    #[serde(rename = "+")]
    Yes,
    #[serde(rename = "?")]
    Partially,
    #[serde(rename = "-")]
    No,
}

impl Display for Matched {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Yes => "+",
                Self::Partially => "?",
                Self::No => "-",
            }
        )
    }
}

impl From<&str> for Matched {
    fn from(value: &str) -> Self {
        match value {
            "+" => Self::Yes,
            "?" => Self::Partially,
            "-" => Self::No,
            _ => Self::No,
        }
    }
}

impl From<String> for Matched {
    fn from(value: String) -> Self {
        Self::from(&value[..])
    }
}
