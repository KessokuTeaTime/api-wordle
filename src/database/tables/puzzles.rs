//! Table `puzzles`.

use entity::puzzles::Model as Puzzle;
use entity::{PuzzleDate, PuzzleSolution, prelude::*, puzzles};
use migration::OnConflict;
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait as _, QuerySelect as _};
use tracing::{error, info, trace, warn};

/// Gets all puzzle dates.
pub async fn get_dates(db: &DatabaseConnection) -> Vec<PuzzleDate> {
    info!("getting dates…");

    let dates = Puzzles::find()
        .select_only()
        .column(puzzles::Column::Date)
        .into_tuple()
        .all(db)
        .await
        .unwrap_or(Vec::new());

    trace!("got dates: {dates:?}");
    dates
}

/// Gets all puzzles.
pub async fn get_puzzles(db: &DatabaseConnection) -> Vec<Puzzle> {
    info!("getting puzzles…");
    let p = Puzzles::find().all(db).await.unwrap_or(Vec::new());

    trace!("got active puzzles: {p:?}");
    p
}

/// Gets a puzzle by date.
pub async fn get_puzzle(db: &DatabaseConnection, date: &PuzzleDate) -> Option<Puzzle> {
    info!("getting puzzle for {date}…");
    let puzzle = Puzzles::find_by_id(date.clone())
        .one(db)
        .await
        .ok()
        .flatten();

    match &puzzle {
        Some(puzzle) => info!("got puzzle for {date}: {puzzle}"),
        None => warn!("no puzzles found for {date}!"),
    }
    puzzle
}

/// Inserts a puzzle solution for a given date.
///
/// # Errors
///
/// Returns [`DbErr`] if the insertion fails.
pub async fn insert_solution(
    db: &DatabaseConnection,
    date: &PuzzleDate,
    solution: &PuzzleSolution,
) -> Result<(), DbErr> {
    info!("inserting puzzle for {date}…");

    let active_puzzle = puzzles::ActiveModel {
        date: ActiveValue::Set(date.clone()),
        solution: ActiveValue::Set(solution.clone()),
    };

    match Puzzles::insert(active_puzzle)
        .on_conflict(
            OnConflict::column(puzzles::Column::Date)
                .update_columns([puzzles::Column::Solution])
                .to_owned(),
        )
        .exec(db)
        .await
    {
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
