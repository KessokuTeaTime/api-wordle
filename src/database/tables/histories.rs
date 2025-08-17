//! Table `histories`.

use chrono::Utc;
use entity::{
    HISTORY_MAX_TRIES, PuzzleDate, PuzzleSolution, SubmitHistory, SubmitWord,
    histories::{self, Model as History},
    prelude::*,
};
use migration::OnConflict;
use sea_orm::{
    ActiveValue, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter, QuerySelect,
};
use tracing::{error, info, warn};

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

#[derive(Debug, Clone)]
pub struct SubmitResult {
    pub submit_history: SubmitHistory,
    pub is_dirty: bool,
    pub is_completed: bool,
}

pub async fn submit_to_history(
    db: &DatabaseConnection,
    date: &PuzzleDate,
    session: &str,
    answer: &PuzzleSolution,
) -> Result<SubmitResult, DbErr> {
    info!("submitting {answer} to history at {date} with {session}…");

    let (mut submit_history, is_dirty, solution) =
        match Histories::find_by_id((date.to_owned(), session.to_owned()))
            .select_only()
            .columns([
                histories::Column::SubmitHistory,
                histories::Column::IsDirty,
                histories::Column::IsCompleted,
                histories::Column::OriginalSolution,
            ])
            .into_tuple::<(Option<SubmitHistory>, bool, bool, PuzzleSolution)>()
            .one(db)
            .await
            .ok()
            .flatten()
        {
            Some((submit_history, is_dirty, true, _)) => {
                warn!("history is completed for {date} with session {session}!");
                return Ok(SubmitResult {
                    submit_history: submit_history.unwrap_or_default(),
                    is_dirty,
                    is_completed: true,
                });
            }
            Some((submit_history, is_dirty, false, solution)) => {
                (submit_history.unwrap_or_default(), is_dirty, solution)
            }
            None => {
                error!("no history found for {date} with session {session}!");
                return Err(DbErr::Custom(format!("session {session} has no history")));
            }
        };

    let word = SubmitWord::tint(answer, &solution);
    submit_history
        .submit(word)
        .map_err(|e| DbErr::Custom(e.to_string()))?;

    let active_history = histories::ActiveModel {
        date: ActiveValue::Unchanged(date.to_owned()),
        session: ActiveValue::Unchanged(session.to_owned()),
        submit_history: ActiveValue::Set(Some(submit_history.clone())),
        original_solution: ActiveValue::Unchanged(solution),
        is_dirty: ActiveValue::Unchanged(false),
        is_completed: ActiveValue::Set(word.all_matches()),
        uploaded_at: ActiveValue::Unchanged(Utc::now().naive_utc()),
    };

    match Histories::insert(active_history)
        .on_conflict(
            OnConflict::columns([histories::Column::Date, histories::Column::Session])
                .update_columns([
                    histories::Column::SubmitHistory,
                    histories::Column::IsCompleted,
                ])
                .to_owned(),
        )
        .exec(db)
        .await
    {
        Ok(_) => {
            info!("submitted {answer} to history at {date} with session {session}");
            Ok(SubmitResult {
                submit_history,
                is_dirty,
                is_completed: word.all_matches(),
            })
        }
        Err(err) => {
            error!("failed to submit {answer} to history at {date} with session {session}: {err}");
            Err(err)
        }
    }
}

pub async fn mark_dirty(db: &DatabaseConnection, date: &PuzzleDate, solution: &PuzzleSolution) {
    let h = Histories::find()
        .filter(histories::Column::Date.eq(date.to_owned()))
        .all(db)
        .await
        .unwrap_or_default();

    if h.is_empty() {
        return;
    }

    info!(
        "marking dirty for {} at {date} with solution {solution}",
        match h.len() {
            1 => "1 history",
            count => &format!("{count} histories"),
        }
    );

    let active_histories: Vec<histories::ActiveModel> = h
        .into_iter()
        .map(|history| histories::ActiveModel {
            date: ActiveValue::Unchanged(date.to_owned()),
            session: ActiveValue::Unchanged(history.session),
            is_dirty: ActiveValue::Set(history.original_solution == *solution),
            ..Default::default()
        })
        .collect();

    Histories::insert_many(active_histories)
        .on_conflict(
            OnConflict::new()
                .update_column(histories::Column::IsDirty)
                .to_owned(),
        )
        .exec(db)
        .await
        .ok();
}
