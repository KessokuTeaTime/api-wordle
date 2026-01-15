//! Table `histories`.

use chrono::Utc;
use entity::{
    PuzzleDate, PuzzleSolution, SubmitHistory, SubmitWord,
    histories::{self, Model as History},
    prelude::*,
};
use migration::OnConflict;
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait as _, QuerySelect as _};

/// Gets a history by date and session.
pub async fn get_history(
    db: &DatabaseConnection,
    date: &PuzzleDate,
    session: &str,
) -> Option<History> {
    tracing::info!("getting history for {date} with session {session}…");
    let history = Histories::find_by_id((date.to_owned(), session.to_owned()))
        .one(db)
        .await
        .ok()
        .flatten();

    match &history {
        Some(history) => tracing::info!("got puzzle for {date} with session {session}: {history}"),
        None => tracing::warn!("no puzzles found for {date} with session {session}!"),
    }
    history
}

/// Creates a new history.
///
/// # Errors
///
/// Returns [`DbErr`] if the insertion fails.
pub async fn create_history(
    db: &DatabaseConnection,
    date: &PuzzleDate,
    session: &str,
    solution: &PuzzleSolution,
) -> Result<(), DbErr> {
    tracing::info!("creating history for {date} with session {session}…");
    let active_history = histories::ActiveModel {
        date: ActiveValue::Set(date.to_owned()),
        session: ActiveValue::Set(session.to_owned()),
        solution: ActiveValue::Set(solution.to_owned()),
        uploaded_at: ActiveValue::Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    match Histories::insert(active_history).exec(db).await {
        Ok(_) => {
            tracing::info!(
                "created history for {date} with session {session} and solution {solution}"
            );
            Ok(())
        }
        Err(err) => {
            tracing::error!(
                "failed to create history for {date} with session {session} and solution {solution}"
            );
            Err(err)
        }
    }
}

/// The result for submitting a word to history.
#[derive(Debug, Clone)]
pub struct SubmitResult {
    /// The updated submit history.
    pub submit_history: SubmitHistory,
    /// Whether the puzzle has been completed.
    pub is_completed: bool,
}

/// Submits a word to history.
///
/// # Errors
///
/// Returns [`DbErr`] if the submission fails.
pub async fn submit_to_history(
    db: &DatabaseConnection,
    date: &PuzzleDate,
    session: &str,
    answer: &PuzzleSolution,
) -> Result<SubmitResult, DbErr> {
    tracing::info!("submitting {answer} to history at {date} with {session}…");

    let (mut submit_history, solution) =
        match Histories::find_by_id((date.to_owned(), session.to_owned()))
            .select_only()
            .columns([
                histories::Column::SubmitHistory,
                histories::Column::IsCompleted,
                histories::Column::Solution,
            ])
            .into_tuple::<(Option<SubmitHistory>, bool, PuzzleSolution)>()
            .one(db)
            .await
            .ok()
            .flatten()
        {
            Some((submit_history, true, _)) => {
                tracing::warn!("history is completed for {date} with session {session}!");
                return Ok(SubmitResult {
                    submit_history: submit_history.unwrap_or_default(),
                    is_completed: true,
                });
            }
            Some((submit_history, false, solution)) => {
                (submit_history.unwrap_or_default(), solution)
            }
            None => {
                tracing::error!("no history found for {date} with session {session}!");
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
        solution: ActiveValue::Unchanged(solution),
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
            tracing::info!("submitted {answer} to history at {date} with session {session}");
            Ok(SubmitResult {
                submit_history,
                is_completed: word.all_matches(),
            })
        }
        Err(err) => {
            tracing::error!(
                "failed to submit {answer} to history at {date} with session {session}: {err}"
            );
            Err(err)
        }
    }
}
