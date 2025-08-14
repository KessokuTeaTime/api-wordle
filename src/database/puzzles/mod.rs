use crate::database::types::Puzzle;

use super::types::{PuzzleDate, PuzzleSolution};

use api_framework::static_lazy_lock;
use tracing::{error, info, trace, warn};

static_lazy_lock! {
    pub WORDS: &[&str] = random_word::all_len(5, random_word::Lang::En).unwrap();
}

pub fn get_dates(conn: &mut PgConnection, includes_deleted: bool) -> Vec<PuzzleDate> {
    use schema::puzzles::dsl::{date as d_date, is_deleted as d_is_deleted, puzzles};

    info!("getting dates…");
    let query = if includes_deleted {
        puzzles.into_boxed()
    } else {
        puzzles.filter(d_is_deleted.eq(false)).into_boxed()
    };
    let dates = query
        .select(d_date)
        .load::<PuzzleDate>(conn)
        .unwrap_or(Vec::new());

    trace!("got dates: {dates:?}");
    dates
}

pub fn get_puzzles(conn: &mut PgConnection, includes_deleted: bool) -> Vec<Puzzle> {
    use schema::puzzles::dsl::{is_deleted as d_is_deleted, puzzles};

    info!("getting puzzles…");
    let query = if includes_deleted {
        puzzles.into_boxed()
    } else {
        puzzles.filter(d_is_deleted.eq(false)).into_boxed()
    };
    let p = query.load::<Puzzle>(conn).unwrap_or(Vec::new());

    if includes_deleted {
        trace!("got active and deleted puzzles: {p:?}");
    } else {
        trace!("got active puzzles: {p:?}");
    }
    p
}

pub fn get_puzzle(conn: &mut PgConnection, date: &PuzzleDate) -> Option<Puzzle> {
    use schema::puzzles::dsl::{date as d_date, is_deleted as d_is_deleted, puzzles};

    info!("getting puzzle for {date}…");
    let puzzle = puzzles
        .filter(d_is_deleted.eq(false))
        .filter(d_date.eq(date))
        .get_result::<Puzzle>(conn)
        .ok();

    match &puzzle {
        Some(puzzle) => info!("got puzzle for {date}: {puzzle}"),
        None => warn!("no puzzles found for {date}!"),
    }
    puzzle
}

pub fn insert_solution(
    conn: &mut PgConnection,
    date: &PuzzleDate,
    solution: &PuzzleSolution,
) -> QueryResult<()> {
    use schema::puzzles::dsl::{is_deleted as d_is_deleted, puzzles, solution as d_solution};

    info!("inserting puzzle for {date}…");
    match if puzzles
        .find(date)
        .filter(d_is_deleted.eq(true))
        .get_result::<Puzzle>(conn)
        .is_ok()
    {
        // A deleted puzzle exists
        diesel::update(puzzles)
            .set((d_solution.eq(solution), d_is_deleted.eq(false)))
            .execute(conn)
    } else {
        diesel::insert_into(puzzles)
            .values(Puzzle::new(date.to_owned(), solution.to_owned()).to_new_puzzle())
            .execute(conn)
    } {
        Ok(_) => {
            info!("inserted solution {solution} for {date}");
            Ok(())
        }
        Err(err) => {
            error!("failed to insert solution {solution} for {date}: {err}");
            Err(err)
        }
    }
}

pub fn update_solution(
    conn: &mut PgConnection,
    date: &PuzzleDate,
    solution: &PuzzleSolution,
) -> QueryResult<()> {
    use schema::puzzles::dsl::{is_deleted as d_is_deleted, puzzles, solution as d_solution};

    let query = puzzles.find(date).filter(d_is_deleted.eq(false));
    if query.get_result::<Puzzle>(conn).is_err() {
        error!(
            "failed to update solution for {date}: not found! a solution must be inserted before updating"
        );
        return Err(diesel::result::Error::NotFound);
    }

    info!("updating solution for {date}…");
    match diesel::update(query)
        .set(d_solution.eq(solution))
        .execute(conn)
    {
        Ok(_) => {
            info!("updated solution {solution} for {date}");
            Ok(())
        }
        Err(err) => {
            error!("failed to update solution {solution} for {date}: {err}");
            Err(err)
        }
    }
}

pub fn delete_solution(conn: &mut PgConnection, date: &PuzzleDate) -> QueryResult<()> {
    use schema::puzzles::dsl::{is_deleted as d_is_deleted, puzzles};

    if puzzles
        .find(date)
        .filter(d_is_deleted.eq(false))
        .get_result::<Puzzle>(conn)
        .is_err()
    {
        error!(
            "failed to delete solution for {date}: not found! a solution must be inserted before deleting"
        );
        return Err(diesel::result::Error::NotFound);
    }

    warn!("deleting solution for {date}…");
    match diesel::update(puzzles.find(date))
        .set(d_is_deleted.eq(true))
        .execute(conn)
    {
        Ok(_) => {
            info!("deleted solution for {date}");
            Ok(())
        }
        Err(err) => {
            error!("failed to delete solution for {date}: {err}");
            Err(err)
        }
    }
}

pub fn insert_or_update_solution(
    conn: &mut PgConnection,
    date: &PuzzleDate,
    solution: &PuzzleSolution,
) -> QueryResult<()> {
    use schema::puzzles::dsl::puzzles;

    if puzzles.find(date).get_result::<Puzzle>(conn).is_ok() {
        update_solution(conn, date, solution)
    } else {
        insert_solution(conn, date, solution)
    }
}
