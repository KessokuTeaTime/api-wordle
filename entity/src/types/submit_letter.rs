use crate::Matched;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmitLetter(char, Matched);

impl SubmitLetter {
    pub fn new(c: char, matched: Matched) -> Self {
        Self(c.to_ascii_uppercase(), matched)
    }
}

impl Display for SubmitLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}
