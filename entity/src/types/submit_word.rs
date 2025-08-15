use crate::SubmitLetter;

use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubmitWord<const N: usize>([SubmitLetter; N]);

impl<const N: usize> SubmitWord<N> {
    pub const SEPARATOR: &str = ",";

    pub fn new(letters: [SubmitLetter; N]) -> Self {
        Self(letters)
    }
}

impl<const N: usize> Display for SubmitWord<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.map(|l| l.to_string()).join(Self::SEPARATOR))
    }
}
