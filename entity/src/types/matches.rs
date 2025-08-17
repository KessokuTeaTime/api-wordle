use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Matches {
    #[serde(rename = "+")]
    Yes,
    #[serde(rename = "?")]
    Partially,
    #[serde(rename = "-")]
    No,
}

impl Matches {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Yes => "+",
            Self::Partially => "?",
            Self::No => "-",
        }
    }
}

impl Display for Matches {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl From<&str> for Matches {
    fn from(value: &str) -> Self {
        match value {
            "+" => Self::Yes,
            "?" => Self::Partially,
            "-" => Self::No,
            _ => Self::No,
        }
    }
}

impl From<String> for Matches {
    fn from(value: String) -> Self {
        Self::from(&value[..])
    }
}
