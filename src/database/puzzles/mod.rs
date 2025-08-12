use crate::schema;

use super::types::{PuzzleDate, PuzzleWord};

use api_framework::framework::State;
use diesel::{PgConnection, QueryDsl, RunQueryDsl, prelude::*};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::puzzles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Puzzle {
    pub date: PuzzleDate,
    pub puzzle: PuzzleWord,
    pub is_deleted: bool,
}

impl Puzzle {
    pub fn new(date: PuzzleDate, puzzle: PuzzleWord) -> Self {
        Self {
            date,
            puzzle,
            is_deleted: false,
        }
    }
}

pub fn get_dates(conn: &mut PgConnection) -> Vec<PuzzleDate> {
    use schema::puzzles::dsl::{date as d_date, puzzles};

    info!("getting dates…");
    let dates = puzzles
        .select(d_date)
        .load::<PuzzleDate>(conn)
        .unwrap_or(Vec::new());

    info!("got dates: {dates:?}");
    dates
}

pub fn get_puzzle(conn: &mut PgConnection, date: &PuzzleDate) -> Option<PuzzleWord> {
    use schema::puzzles::dsl::{date as d_date, puzzle as d_puzzle, puzzles};

    info!("getting puzzle for {date}…");
    let puzzle = puzzles
        .filter(d_date.eq(date))
        .select(d_puzzle)
        .get_result::<PuzzleWord>(conn)
        .ok();

    match &puzzle {
        Some(puzzle) => info!("got puzzle for {date}: {puzzle}"),
        None => warn!("no puzzle found for {date}"),
    }
    puzzle
}

pub fn delete_puzzle(conn: &mut PgConnection, date: &PuzzleDate) {
    use schema::puzzles::dsl::puzzles;

    warn!("deleting puzzle at {date}…");
    drop(diesel::delete(puzzles.find(date)).execute(conn));
}

pub fn insert_or_update_puzzle(
    conn: &mut PgConnection,
    date: &PuzzleDate,
    puzzle: &PuzzleWord,
) -> State<()> {
    use schema::puzzles::dsl::{puzzle as d_puzzle, puzzles};

    let query = puzzles.find(date);
    if let Ok(existing_puzzle) = query.get_result::<Puzzle>(conn) {
        // Updates the existing puzzle
        info!(
            "updating existing puzzle for {date} from {} to {puzzle}…",
            existing_puzzle.puzzle
        );

        if *puzzle == existing_puzzle.puzzle {
            warn!("puzzle for {date} isn't changed: {puzzle}");
            State::Success(())
        } else {
            match diesel::update(query)
                .set(d_puzzle.eq(&puzzle))
                .execute(conn)
            {
                Ok(_) => State::Success(()),
                Err(err) => {
                    error!("failed to update puzzle for {date}: {err}");
                    State::Retry
                }
            }
        }
    } else {
        // Inserts a puzzle
        info!("inserting puzzle {puzzle} for {date}…");
        match diesel::insert_into(puzzles)
            .values(Puzzle::new(date.clone(), puzzle.to_owned()))
            .execute(conn)
        {
            Ok(_) => State::Success(()),
            Err(err) => {
                error!("failed to insert puzzle for {date}: {err}");
                State::Retry
            }
        }
    }
}
