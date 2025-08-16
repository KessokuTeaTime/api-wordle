//! Table `histories`.

use chrono::Utc;
use entity::{
    HISTORY_MAX_TRIES, PuzzleDate, PuzzleSolution, SubmitHistory, SubmitWord,
    histories::{self, Model as History},
    prelude::*,
};
use migration::OnConflict;
use sea_orm::{
    ActiveValue, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
    QuerySelect,
};
use tracing::{error, info, trace, warn};

pub async fn get_history(
    db: &DatabaseConnection,
    date: &PuzzleDate,
    session: String,
) -> Option<History> {
    info!("getting history for {date} with session {session}…");
    let history = Histories::find_by_id((date.clone(), session.clone()))
        .one(db)
        .await
        .ok()
        .flatten();

    match &history {
        Some(history) => info!("got puzzle for {date} with session {session}: {history}"),
        None => warn!("no puzzles found for {date} with session {session}!"),
    }
    history
}

#[derive(Debug, Clone)]
pub struct SubmitResult {
    pub remaining_tries: usize,
    pub is_dirty: bool,
}

pub async fn submit_to_history(
    db: &DatabaseConnection,
    date: &PuzzleDate,
    session: String,
    word: SubmitWord,
) -> Result<SubmitResult, DbErr> {
    info!("submitting {word} to history at {date} with {session}…");

    let mut remaining_tries: usize = 0;
    let active_history = match Histories::find_by_id((date.clone(), session.clone()))
        .one(db)
        .await
        .ok()
        .flatten()
    {
        Some(History { submit_history, .. }) => match submit_history {
            Some(mut history) => {
                remaining_tries = history.remaining_tries() - 1;
                history
                    .submit(word)
                    .map_err(|e| DbErr::Custom(e.to_string()))?;

                histories::ActiveModel {
                    date: ActiveValue::Unchanged(date.clone()),
                    session: ActiveValue::Unchanged(session.clone()),
                    submit_history: ActiveValue::Set(Some(history)),
                    ..Default::default()
                }
            }
            None => {
                remaining_tries = HISTORY_MAX_TRIES;
                let solution = Puzzles::find_by_id(date.clone()).one(db)
                .await.ok().flatten() {

                }

                histories::ActiveModel {
                    date: ActiveValue::Set(date.clone()),
                    session: ActiveValue::Unchanged(session.clone()),
                    submit_history: ActiveValue::Set(Some(SubmitHistory::new())),
                    original_solution:
                    uploaded_at: ActiveValue::Set(Utc::now().naive_utc())
                }
            }
        },
        None => {
            warn!("no history found for {date} with session {session}!");
            return Err(DbErr::Custom(format!("session {session} has no history")));
        }
    };

    match Histories::insert(active_history)
        .on_conflict(
            OnConflict::columns([histories::Column::Date, histories::Column::Session])
                .update_column(histories::Column::SubmitHistory)
                .to_owned(),
        )
        .exec(db)
        .await
    {
        Ok(_) => {
            info!("submitted {word} to history at {date} with session {session}");
            Ok(SubmitResult {
                remaining_tries,
                is_dirty: (),
            })
        }
        Err(err) => {
            error!("failed to submit {word} to history at {date} with session {session}: {err}");
            Err(err)
        }
    }
}

pub async fn update_solution(
    db: &DatabaseConnection,
    date: &PuzzleDate,
    solution: &PuzzleSolution,
) -> Result<(), DbErr> {
    use entity::puzzles;

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
