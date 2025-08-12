use crate::{
    database::types::{NewPuzzle, Puzzle},
    schema,
};

use super::types::{PuzzleDate, PuzzleSolution};

use api_framework::framework::State;
use diesel::{PgConnection, QueryDsl, RunQueryDsl, prelude::*};
use tracing::{error, info, trace, warn};

pub fn get_puzzles(conn: &mut PgConnection) -> Vec<Puzzle> {
    use schema::puzzles::dsl::puzzles;

    info!("getting puzzles");
    let p = puzzles.load::<Puzzle>(conn).unwrap_or(Vec::new());

    trace!("got puzzles: {p:?}");
    p
}

pub fn get_dates(conn: &mut PgConnection) -> Vec<PuzzleDate> {
    use schema::puzzles::dsl::{date as d_date, puzzles};

    info!("getting dates…");
    let dates = puzzles
        .select(d_date)
        .load::<PuzzleDate>(conn)
        .unwrap_or(Vec::new());

    trace!("got dates: {dates:?}");
    dates
}

pub fn get_puzzle(conn: &mut PgConnection, date: &PuzzleDate) -> Option<Puzzle> {
    use schema::puzzles::dsl::{date as d_date, puzzles};

    info!("getting puzzle for {date}…");
    let puzzle = puzzles
        .filter(d_date.eq(date))
        .get_result::<Puzzle>(conn)
        .ok();

    match &puzzle {
        Some(puzzle) => info!("got puzzle for {date}: {puzzle}"),
        None => warn!("no puzzles found for {date}!"),
    }
    puzzle
}

pub fn put_puzzle(
    conn: &mut PgConnection,
    date: &PuzzleDate,
    solution: &PuzzleSolution,
) -> QueryResult<()> {
    use schema::puzzles::dsl::puzzles;

    info!("putting puzzle for {date}…");
    match diesel::insert_into(puzzles)
        .values(NewPuzzle {
            date: date.to_owned(),
            solution: solution.to_owned(),
        })
        .execute(conn)
    {
        Ok(_) => {
            info!("put solution {solution} for {date}");
            Ok(())
        }
        Err(err) => {
            error!("failed to put solution {solution} for {date}: {err}");
            Err(err)
        }
    }
}

pub fn delete_puzzle(conn: &mut PgConnection, date: &PuzzleDate) -> QueryResult<()> {
    use schema::puzzles::dsl::puzzles;

    warn!("deleting puzzle for {date}…");
    match diesel::delete(puzzles.find(date)).execute(conn) {
        Ok(_) => {
            info!("deleted puzzle for {date}");
            Ok(())
        }
        Err(err) => {
            error!("failed to delete puzzle for {date}: {err}");
            Err(err)
        }
    }
}

pub fn insert_or_update_solution(
    conn: &mut PgConnection,
    date: &PuzzleDate,
    solution: &PuzzleSolution,
) -> State<()> {
    use schema::puzzles::dsl::{puzzles, solution as d_solution};

    let query = puzzles.find(date);
    if let Ok(existing_puzzle) = query.get_result::<Puzzle>(conn) {
        // Updates the existing puzzle
        info!(
            "updating existing solution for {date} from {} to {solution}…",
            existing_puzzle.solution
        );

        if *solution == existing_puzzle.solution {
            warn!("solution for {date} isn't changed: {solution}");
            State::Success(())
        } else {
            match diesel::update(query)
                .set(d_solution.eq(&solution))
                .execute(conn)
            {
                Ok(_) => State::Success(()),
                Err(err) => {
                    error!("failed to update solution for {date}: {err}");
                    State::Retry
                }
            }
        }
    } else {
        // Inserts a puzzle
        info!("inserting solution {solution} for {date}…");
        match diesel::insert_into(puzzles)
            .values(NewPuzzle {
                date: date.to_owned(),
                solution: solution.to_owned(),
            })
            .execute(conn)
        {
            Ok(_) => State::Success(()),
            Err(err) => {
                error!("failed to insert solution for {date}: {err}");
                State::Retry
            }
        }
    }
}
