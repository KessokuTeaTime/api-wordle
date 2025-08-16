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
    session: &str,
) -> Option<History> {
    info!("getting history for {date} with session {session}…");
    let history = Histories::find_by_id((date.to_owned(), session.to_owned()))
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

pub async fn create_history(
    db: &DatabaseConnection,
    date: &PuzzleDate,
    session: &str,
    solution: &PuzzleSolution,
) -> Result<(), DbErr> {
    info!("creating history for {date} with session {session}…");
    let active_history = histories::ActiveModel {
        date: ActiveValue::Set(date.to_owned()),
        session: ActiveValue::Set(session.to_owned()),
        original_solution: ActiveValue::Set(solution.to_owned()),
        is_dirty: ActiveValue::Set(false),
        uploaded_at: ActiveValue::Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    match Histories::insert(active_history).exec(db).await {
        Ok(_) => {
            info!("created history for {date} with session {session} and solution {solution}");
            Ok(())
        }
        Err(err) => {
            error!(
                "failed to create history for {date} with session {session} and solution {solution}"
            );
            Err(err)
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SubmitResult {
    pub remaining_tries: usize,
    pub is_dirty: bool,
}

pub async fn submit_to_history(
    db: &DatabaseConnection,
    date: &PuzzleDate,
    session: &str,
    word: &SubmitWord,
) -> Result<SubmitResult, DbErr> {
    info!("submitting {word} to history at {date} with {session}…");

    let (mut submit_history, is_dirty) =
        match Histories::find_by_id((date.to_owned(), session.to_owned()))
            .select_only()
            .columns([histories::Column::SubmitHistory, histories::Column::IsDirty])
            .into_tuple::<(Option<SubmitHistory>, bool)>()
            .one(db)
            .await
            .ok()
            .flatten()
        {
            Some((submit_history, is_dirty)) => match submit_history {
                Some(history) => (history, is_dirty),
                None => (SubmitHistory::new(), false),
            },
            None => {
                error!("no history found for {date} with session {session}!");
                return Err(DbErr::Custom(format!("session {session} has no history")));
            }
        };

    submit_history
        .submit(word.to_owned())
        .map_err(|e| DbErr::Custom(e.to_string()))?;

    let remaining_tries = submit_history.remaining_tries();
    let active_history = histories::ActiveModel {
        date: ActiveValue::Unchanged(date.to_owned()),
        session: ActiveValue::Unchanged(session.to_owned()),
        submit_history: ActiveValue::Set(Some(submit_history)),
        ..Default::default()
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
                is_dirty,
            })
        }
        Err(err) => {
            error!("failed to submit {word} to history at {date} with session {session}: {err}");
            Err(err)
        }
    }
}
