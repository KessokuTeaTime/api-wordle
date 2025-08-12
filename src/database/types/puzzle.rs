use std::fmt::Display;

use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use super::{PuzzleDate, PuzzleSolution};
use crate::schema;

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::puzzles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPuzzle {
    pub date: PuzzleDate,
    pub solution: PuzzleSolution,
}

impl Display for NewPuzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.solution)
    }
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
#[diesel(table_name = schema::puzzles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Puzzle {
    pub date: PuzzleDate,
    pub solution: PuzzleSolution,
    pub is_deleted: bool,
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.solution)
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
}
