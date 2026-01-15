use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// The match result for each letter in a submitted word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(clippy::exhaustive_enums)]
pub enum Matches {
    /// The letter matches exactly.
    #[serde(rename = "+")]
    Yes,
    /// The letter is in the word but in a different position.
    #[serde(rename = "?")]
    Partially,
    /// The letter does not match.
    #[serde(rename = "-")]
    No,
}

impl Matches {
    /// Converts the `Matches` enum to its string representation.
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
