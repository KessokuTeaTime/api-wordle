use crate::Matches;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmitLetter {
    pub letter: char,
    pub matches: Matches,
}

impl SubmitLetter {
    pub fn new(c: char, matches: Matches) -> Self {
        Self {
            letter: c.to_ascii_uppercase(),
            matches,
        }
    }
}

impl Display for SubmitLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.letter, self.matches)
    }
}
