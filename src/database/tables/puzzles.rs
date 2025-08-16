//! Table `puzzles`.

use entity::puzzles::Model as Puzzle;
use entity::{PuzzleDate, PuzzleSolution, prelude::*, puzzles};
use migration::OnConflict;
use sea_orm::{
    ActiveValue, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
    QuerySelect,
};
use tracing::{error, info, trace, warn};

pub async fn get_dates(db: &DatabaseConnection, includes_deleted: bool) -> Vec<PuzzleDate> {
    info!("getting dates…");
    let query = if includes_deleted {
        Puzzles::find()
    } else {
        Puzzles::find().filter(Condition::all().add(puzzles::Column::IsDeleted.eq(false)))
    };

    let dates = query
        .select_only()
        .column(puzzles::Column::Date)
        .into_tuple()
        .all(db)
        .await
        .unwrap_or(Vec::new());

    trace!("got dates: {dates:?}");
    dates
}

pub async fn get_puzzles(db: &DatabaseConnection, includes_deleted: bool) -> Vec<Puzzle> {
    info!("getting puzzles…");
    let query = if includes_deleted {
        Puzzles::find()
    } else {
        Puzzles::find().filter(Condition::all().add(puzzles::Column::IsDeleted.eq(false)))
    };
    let p = query.all(db).await.unwrap_or(Vec::new());

    if includes_deleted {
        trace!("got active and deleted puzzles: {p:?}");
    } else {
        trace!("got active puzzles: {p:?}");
    }
    p
}

pub async fn get_puzzle(db: &DatabaseConnection, date: &PuzzleDate) -> Option<Puzzle> {
    info!("getting puzzle for {date}…");
    let puzzle = Puzzles::find_by_id(date.clone())
        .filter(Condition::all().add(puzzles::Column::IsDeleted.eq(false)))
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

pub async fn insert_solution(
    db: &DatabaseConnection,
    date: &PuzzleDate,
    solution: &PuzzleSolution,
) -> Result<(), DbErr> {
    info!("inserting puzzle for {date}…");

    let active_puzzle = puzzles::ActiveModel {
        date: ActiveValue::Set(date.clone()),
        solution: ActiveValue::Set(solution.clone()),
        is_deleted: ActiveValue::Set(false),
    };

    match Puzzles::insert(active_puzzle)
        .on_conflict(
            OnConflict::column(puzzles::Column::Date)
                .update_columns([puzzles::Column::Solution, puzzles::Column::IsDeleted])
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

pub async fn update_solution(
    db: &DatabaseConnection,
    date: &PuzzleDate,
    solution: &PuzzleSolution,
) -> Result<(), DbErr> {
    let active_puzzle = puzzles::ActiveModel {
        date: ActiveValue::Set(date.clone()),
        solution: ActiveValue::Set(solution.clone()),
        is_deleted: ActiveValue::Set(false),
    };

    match Puzzles::update(active_puzzle).exec(db).await {
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

pub async fn delete_solution(db: &DatabaseConnection, date: &PuzzleDate) -> Result<(), DbErr> {
    match Puzzles::delete_by_id(date.clone()).exec(db).await {
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
