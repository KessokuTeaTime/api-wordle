use std::fmt::Display;

use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use super::{PuzzleDate, PuzzleSolution};
use crate::schema;

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::puzzles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPuzzle {
    date: PuzzleDate,
    solution: PuzzleSolution,
}

impl Display for NewPuzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.solution, self.date)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultPuzzle {
    date: PuzzleDate,
    solution: String,
}

impl Display for ResultPuzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.solution, self.date)
    }
}

impl From<NewPuzzle> for ResultPuzzle {
    fn from(value: NewPuzzle) -> Self {
        Self {
            date: value.date,
            solution: value.solution.to_string(),
        }
    }
}

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
#[diesel(table_name = schema::puzzles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Puzzle {
    pub date: PuzzleDate,
    pub solution: PuzzleSolution,
    pub is_deleted: bool,
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} ({})",
            if self.is_deleted { '-' } else { '+' },
            self.solution,
            self.date
        )
    }
}

impl Puzzle {
    pub fn new(date: PuzzleDate, solution: PuzzleSolution) -> Self {
        Self {
            date,
            solution,
            is_deleted: false,
        }
    }

    pub fn to_new_puzzle(self) -> NewPuzzle {
        NewPuzzle {
            date: self.date,
            solution: self.solution,
        }
    }

    pub fn to_result_puzzle(self) -> ResultPuzzle {
        ResultPuzzle {
            date: self.date,
            solution: self.solution.to_string(),
        }
    }
}
